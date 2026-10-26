use std::collections::HashMap;
use crate::{bristol::*, tfhe_x::std::StdServerKey, tfhe_x::Ciphertext};

pub struct Evaluator;

impl Evaluator
{
    fn eval_gate(gate: &Gate, sk:&StdServerKey, x0: &Ciphertext,x1: Option<&Ciphertext>) -> Ciphertext
    {
        match gate.gtype
        {
            GType::AND  => sk.and(x0, x1.unwrap()),
            GType::NAND => sk.nand(x0, x1.unwrap()),
            GType::OR   => sk.or(x0, x1.unwrap()),
            GType::NOR  => sk.nor(x0, x1.unwrap()),
            GType::XOR  => sk.xor(x0, x1.unwrap()),
            GType::XNOR => sk.xnor(x0, x1.unwrap()),
            GType::INV | GType::NOT  => sk.not(x0),
            GType::EQ => sk.trivial_encrypt(gate.inputs[0] == 1),
            GType::EQW  => x0.clone(),
            _ => panic!("unsupported gate"),
        }
    }
    pub fn evaluate(circ:&Circuit,input: &[Ciphertext],sk:&StdServerKey) -> Vec<Ciphertext>
    {
        let mut env = HashMap::<u32,Ciphertext>::new();
        assert_eq!(circ.total_num_of_input_pins(), input.len());
        
        for (i, v) in input.iter().enumerate() {
            env.insert(i as u32, v.clone());}

        for gate in &circ.gates
        {
            let x0 = env.get(&gate.inputs[0]).unwrap();
            let x1 = if gate.inputs.len()>1 {Some(env.get(&gate.inputs[1]).unwrap())} else{None};
            let y = Evaluator::eval_gate(gate,sk,x0, x1);
            env.insert(gate.output, y);        
        }
        
        let(start,end) = circ.output_pins();
        let mut res = Vec::<Ciphertext>::new();
        for idx in start..end{
            let val = env.get(&idx).unwrap();
            res.push(val.clone());
        }
        return res;
    }
}