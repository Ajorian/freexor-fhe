use std::{collections::HashMap, hash::Hash};
use crate::{bristol::*, tfhe_x::{Ciphertext, freexor::{FxClientKey, FxServerKey}}};

type Environment = HashMap::<u32,Ciphertext>;
type EstimationMap = HashMap::<u32, u32>;

pub struct Evaluator
{
    env:Environment,
    mpvals : EstimationMap,
    ck: *const FxClientKey,
}


impl Evaluator
{
    pub fn new(ck:*const FxClientKey) -> Self
    {
        Evaluator 
        { 
            env:Environment::new(), 
            mpvals:EstimationMap::new(), 
            ck:ck 
        }
    }

    fn eval_gate(gate: &Gate, sk:&FxServerKey, x0: Option<&Ciphertext>,x1: Option<&Ciphertext>) -> Ciphertext
    {
        match gate.gtype
        {
            GType::AND  => sk.and(x0.unwrap(), x1.unwrap()),
            GType::NAND => sk.nand(x0.unwrap(), x1.unwrap()),
            GType::OR   => sk.or(x0.unwrap(), x1.unwrap()),
            GType::NOR  => sk.nor(x0.unwrap(), x1.unwrap()),
            GType::XOR  => sk.xor(x0.unwrap(), x1.unwrap()),
            GType::XNOR => sk.xnor(x0.unwrap(), x1.unwrap()),
            GType::INV | GType::NOT  => sk.not(x0.unwrap()),
            GType::EQ => sk.trivial_encrypt(gate.inputs[0] == 1),
            GType::EQW  => x0.unwrap().clone(),
            _ => panic!("unsupported gate"),
        }
    }

    fn lazy_bootstrapping_not(&mut self,gate: &Gate,sk:&FxServerKey) -> u32
    {
        // since NOT increases the ct by 1, if mvals[wire]+1>(headroom-1) then refresh
        let h = sk.headroom();
        let wire = gate.inputs[0];
        let mv = self.mpvals[&wire];
        if mv > h-2
        {
            self.refresh(wire, sk);
            return 2; //input mv is 1, NOT add it by 1.
        }
        return mv+1;
    }

    fn lazy_bootstrapping_xor(&mut self,gate: &Gate,sk:&FxServerKey) -> u32
    {
        //XOR adds two ciphertexts, hence if mvals[x0] + mvals[x1] >(headroom -1) then refresh
        let h = sk.headroom();
        let (wire0, wire1) = (gate.inputs[0], gate.inputs[1]);
        let mut mv0 = self.mpvals[&wire0];
        let mut mv1 = self.mpvals[&wire1];
        if (mv0+mv1) > h -1
        {
            // refresh the larger one first, then the other only if still needed
            let order = if mv0 > mv1 { [wire0, wire1] } else { [wire1, wire0] };
            for w in order
            {
                if mv0+mv1 < h { break; }
                self.refresh(w, sk);
                if w == wire0 {mv0 = 1;}
                else {mv1 = 1;}
            }
        }
        return  mv0+mv1;
    }

    fn lazy_bootstrapping_xnor(&mut self,gate: &Gate,sk:&FxServerKey) -> u32
    {
        //XOR adds two ciphertexts +1, hence if mvals[x0] + mvals[x1] + 1 >(headroom -1) then refresh
        let h = sk.headroom();
        let (wire0, wire1) = (gate.inputs[0], gate.inputs[1]);
        let mut mv0 = self.mpvals[&wire0];
        let mut mv1 = self.mpvals[&wire1];
        if (mv0+mv1) > h -2
        {
            // refresh the larger one first, then the other only if still needed
            let order = if mv0 > mv1 { [wire0, wire1] } else { [wire1, wire0] };
            for w in order
            {
                if mv0+mv1 < h-1 { break; }
                self.refresh(w, sk);
                if w == wire0 {mv0 = 1;}
                else {mv1 = 1;}
            }
        }
        return  mv0+mv1+1;
    }

    fn lazy_bootstrapping_symmetric_gates(&mut self,gate: &Gate,sk:&FxServerKey) -> u32
    {
        //for symmetric gates 2ct0 + Π(ct1) is computed, hence if mvals[x1]>(headroom-1)
        //or 2mvals[x0]>(headroom-1) or 2mvals[x0]+1>(headroom-1) (combining the last two cases) we need to referesh
        
        let h = sk.headroom();
        let (wire0, wire1) = (gate.inputs[0], gate.inputs[1]);
        let mut mv0 = self.mpvals[&wire0];
        let mv1 = self.mpvals[&wire1];

        if (2*mv0> h-1) || (2*mv0 + 1 > h-1) // these two conditions can be merged, but i keep them for clarity
        {
            self.refresh(wire0, sk);
            mv0 = 1;
        }

        if mv1> h-1 // probably this never happens
        {
            self.refresh(wire1, sk);
        }

        return  2*mv0+1;
    }

    fn refresh(&mut self, wire:u32, sk:&FxServerKey)
    {
        let r = sk.refresh(&self.env[&wire]);
        self.env.insert(wire, r);
        self.mpvals.insert(wire, 1);
    }

    pub fn evaluate(&mut self, circ:&Circuit,input: &[Ciphertext],sk:&FxServerKey) -> Vec<Ciphertext>
    {
        assert_eq!(circ.total_num_of_input_pins(), input.len());
        
        self.env.clear();
        self.mpvals.clear();

        for (i, v) in input.iter().enumerate() 
        {
            self.env.insert(i as u32, v.clone());
            self.mpvals.insert(i as u32, 1);
        }

        for gate in &circ.gates
        {
            let mv_out = match gate.gtype
            {
                GType::EQ => 1,
                GType::EQW => self.mpvals[&gate.inputs[0]], 
                GType::XOR => self.lazy_bootstrapping_xor(&gate, sk),
                GType::XNOR => self.lazy_bootstrapping_xnor(&gate, sk),
                GType::NOT | GType::INV => self.lazy_bootstrapping_not(&gate, sk),
                GType::AND | GType::NAND| GType::OR| GType::NOR => self.lazy_bootstrapping_symmetric_gates(&gate,sk),
                _ => panic!("invalid gate type"),
            };
            
            let x_ct = if gate.gtype!= GType::EQ{self.env.get(&gate.inputs[0])}else{None};
            let y_ct = if gate.inputs.len()>1 {Some(self.env.get(&gate.inputs[1]).unwrap())} else{None};
            let z_ct = Evaluator::eval_gate(&gate,sk,x_ct, y_ct);
            
            self.env.insert(gate.output, z_ct); 
            self.mpvals.insert(gate.output,mv_out);   
        }
                
        
        let(start,end) = circ.output_pins();
        let mut res = Vec::<Ciphertext>::new();
        for idx in start..end{
            let val = self.env.get(&idx).unwrap();
            res.push(val.clone());
        }
        return res;
    }
}