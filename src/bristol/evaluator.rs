use std::collections::HashMap;
use crate::bristol::{Circuit,Gate,GType};

type Environment = HashMap::<u32,bool>;

pub struct Evaluator
{
    pub env:Environment,
}

impl Evaluator
{
    pub fn new() -> Self
    {
        Evaluator { env: Environment::new() }
    }

    fn eval_gate(gate: &Gate, x0: &bool,x1: Option<&bool>) -> bool
    {
        match gate.gtype
        {
            GType::AND  => *x0 && *x1.unwrap(),
            GType::NAND => !(*x0 && *x1.unwrap()),
            GType::OR   => *x0 || *x1.unwrap(),
            GType::NOR  => !(*x0 || *x1.unwrap()),
            GType::XOR  => *x0 ^ *x1.unwrap(),
            GType::XNOR => !(*x0 ^ *x1.unwrap()),
            GType::INV | GType::NOT  => !*x0,
            GType::EQ   => gate.inputs[0] == 1,
            GType::EQW  => *x0,
            _ => panic!("unsupported gate"),
        }
    }
    pub fn evaluate(&mut self, circ:&Circuit,input:&[bool]) -> Vec<bool>
    {
        assert_eq!(circ.total_num_of_input_pins(), input.len());
        
        for (i, v) in input.iter().enumerate() {
            self.env.insert(i as u32, *v);}

        for gate in &circ.gates
        {
            if self.env.get(&gate.inputs[0]) == None{println!("{:?}",gate);}
            let x0 = self.env.get(&gate.inputs[0]).unwrap();
            let x1 = if gate.inputs.len()>1 {Some(self.env.get(&gate.inputs[1]).unwrap())} else{None};
            let y = Evaluator::eval_gate(gate, x0, x1);
            self.env.insert(gate.output, y);        
        }
        
        let(start,end) = circ.output_pins();
        let mut res = Vec::<bool>::new();
        for idx in start..end{
            let val = self.env.get(&idx).unwrap();
            res.push(val.clone());
        }
        return res;
    }
}