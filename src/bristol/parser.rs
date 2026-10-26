use std::{fs::File};
use super::circuit::*;
use std::collections::HashMap;
use std::io::{Result,BufRead,BufReader};

impl Gate
{
    pub fn parse(line:&str,n_wires:u32) -> Result<Gate> 
    {
        let mut inputs = Vec::<u32>::new();
        let tokens:Vec<&str> = line.trim().split_whitespace().collect();

        let n_in:usize = tokens[0].parse().unwrap();
        let n_out:usize = tokens[1].parse().unwrap();
        assert!(n_out ==1, "Not supported gate");
        assert!(tokens.len() == (n_in+n_out+3),"Invalid gate {}",line);

        for i in 2..2+n_in
        {
            let widx = tokens[i].parse().unwrap();
            assert!(widx < n_wires, "invalid wire index for gate {}", line);
            inputs.push(widx);
        }
        
        let output:u32 = tokens[2+n_in].parse().unwrap();
        assert!(output < n_wires, "invalid wire index for gate {}", line);
        
        let gtype = parse_gtype(tokens[tokens.len()-1]);
        if gtype == GType::EQ{
            assert!(matches!(inputs[0], 0 | 1),"invalid EQ gate");}

        assert!(gtype!= GType::ERR, "invalid gate type for gate {}", line);

        Ok(Gate{gtype,inputs,output})
    }
}

impl Circuit
{
    pub fn parse(path:&str) -> Result< Circuit> 
    {
        let mut idx:u32 = 0;
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        
        let mut n_gates:u32 = 0;
        let mut n_wires:u32 = 0;
        let mut inputs = Vec::<(u32,u32)>::new();
        let mut outputs = Vec::<(u32,u32)>::new();
        let mut gates = Vec::<Gate>::new();

        for l in reader.lines()
        {
            let line = l?;
            if line.is_empty() {continue;}
            
            match idx 
            {
                0 =>
                {
                    let tokens:Vec<&str> = line.trim().split_whitespace().collect();
                    assert!(tokens.len()==2, "Syntax error: line 1 defines (num of gates, num of wires)");
                    n_gates = tokens[0].parse().unwrap();
                    n_wires = tokens[1].parse().unwrap();
                }

                1 =>
                {
                    let tokens:Vec<&str> = line.trim().split_whitespace().collect();
                    let n_inputs:usize = tokens[0].parse().unwrap();
                    assert!(tokens.len()== n_inputs+1, "Syntax error: line 2 defines (num of inputs, [iLen1, iLen2, ...])");
                    
                    let mut s = 0;
                    for i in 1..tokens.len()
                    {
                        let len:u32 = tokens[i].parse().unwrap();
                        inputs.push((s,s+len));
                        s += len;
                    }
                }

                2=>
                {
                    let tokens:Vec<&str> = line.trim().split_whitespace().collect();
                    let n_outputs:usize = tokens[0].parse().unwrap();
                    assert!(tokens.len()== n_outputs+1, "Syntax error: line 3 defines (num of outputs, [oLen1, oLen2, ...])");
                    
                    let mut e = n_wires;
                    for i in (1..tokens.len()).rev()
                    {
                        let len:u32 = tokens[i].parse().unwrap();
                        outputs.insert(0,(e-len,e));
                        e -= len;
                    }
                }

                _=>
                {
                    let g = Gate::parse(&line,n_wires)?;
                    gates.push(g);
                }
            }
            idx +=1;
        }

        assert!(n_gates == gates.len().try_into().unwrap(), "invalid number of gates");
        let circ = Circuit{n_wires,inputs,outputs,gates};
        Ok(circ)
    }

    pub fn input_pins(&self) -> (u32,u32)
    {
        let mut sum = 0;
        for &(start, end) in &self.inputs {
            sum += end - start;}
        return  (0,sum);
    }


    pub fn output_pins(&self) -> (u32,u32)
    {
        let mut sum = 0;
        for &(start, end) in &self.outputs {
            sum += end - start;}
        return ((self.n_wires-sum),self.n_wires);
    }

    pub fn total_num_of_input_pins(&self) -> usize
    {
        let (s,e) = self.input_pins();
        return (e-s) as usize;
    }

    pub fn total_num_of_output_pins(&self) -> usize
    {
        let (s,e) = self.output_pins();
        return (e-s) as usize;
    }

    pub fn input_sizes(&self) -> Vec<usize>
    {
        let mut res = Vec::<usize>::new();
        for &(s,e) in &self.inputs{
            res.push((e-s)as usize);}
        return res;
    }

    pub fn output_sizes(&self) -> Vec<usize>
    {
        let mut res = Vec::<usize>::new();
        for &(s,e) in &self.outputs{
            res.push((e-s)as usize);}
        return res;
    }

    pub fn gates_info(&self) -> HashMap<GType, u32>
    {
        let mut res: HashMap<GType, u32> = [
            GType::AND, GType::NAND, GType::OR, GType::NOR,
            GType::XOR, GType::XNOR, GType::INV, GType::NOT,
            GType::EQ, GType::EQW, GType::ERR,
        ].into_iter().map(|g| (g, 0)).collect();
        
        for gate in &self.gates{
            *res.get_mut(&gate.gtype).unwrap() += 1;}
        return res;
    }

    pub fn debug_info(&self)
    {
        println!("num of wires = {}",self.n_wires);
        println!("inputs:");
        for &(s,e) in &self.inputs{
            println!("\t({},{})",s,e);}

        println!("outputs:");
        for &(s,e) in &self.outputs{
            println!("\t({},{})",s,e);}
    }
}

