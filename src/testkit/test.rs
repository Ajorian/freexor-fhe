use std::path::Path;
use std::time::Instant;
use super::report::*;
use crate::{bristol::GType, tfhe_x::{Ciphertext,FxClientKey}};
use super::samples::TestData;
use crate::bristol::{Circuit,gtype_to_str};
use std::io::{self, Write};

pub struct Test;

impl Test 
{
    fn report_circuit(circ:&Circuit, report:&mut Report)
    {
        let gates = circ.gates_info();
        for (typ, cnt) in gates
        {
            if cnt!=0 {report.gates.insert(gtype_to_str(&typ).to_string(), cnt);}
        }
    }

    fn parse_files(circ_path:&str) -> (Circuit,TestData)
    {
        let path = Path::new(circ_path);
        let binding = path.with_extension("csv");
        let csv_path = binding.to_str().unwrap();
        
        let circ = Circuit::parse(&circ_path).unwrap();
        let mut sizes = circ.input_sizes(); 
        sizes.extend(circ.output_sizes());
        let tpoints = TestData::new(&sizes,&csv_path);
        return (circ,tpoints);
    }

    fn eval_plain(circ:&Circuit,input:&[bool],ref_output:&[bool],report:&mut Report)
    {
        let t0 = Instant::now();
        let mut eval = crate::bristol::Evaluator::new();
        let output = eval.evaluate(&circ,input);
        report.elapsed_nanos.push(t0.elapsed().as_nanos());
        report.succeeded = ref_output == output;
        if ! report.succeeded
        {
            report.err_msg = format!("input={:?}, output={:?}, expected={:?}",bools_to_hex(&input),bools_to_hex(&output),bools_to_hex(&ref_output));
        }
    }

    fn eval_homomorphic_std(circ:&Circuit,input:&[bool],ref_output:&[bool],report:&mut Report)
    {
        let (mut ck,sk) = crate::tfhe_x::std::generate_keys();
        let mut ct_input = Vec::<Ciphertext>::new();
        for bit in input {ct_input.push(ck.encrypt(*bit));}
        
        let t0 = Instant::now();
        let ct_output = crate::tfhe_x::std::Evaluator::evaluate(circ,&ct_input,&sk);
        report.elapsed_nanos.push(t0.elapsed().as_nanos());
        let mut output = Vec::<bool>::new();
        for ct in ct_output {output.push(ck.decrypt(&ct));}
        report.succeeded = ref_output == output;
        if ! report.succeeded
        {
            report.err_msg = format!("input={:?}, output={:?}, expected={:?}",bools_to_hex(&input),bools_to_hex(&output),bools_to_hex(&ref_output));
        }
    }

    fn eval_homomorphic_freexor(circ:&Circuit,input:&[bool],ref_output:&[bool],report:&mut Report)
    {
        let (mut ck,sk) = crate::tfhe_x::freexor::generate_keys();
        let mut ct_input = Vec::<Ciphertext>::new();
        for bit in input {ct_input.push(ck.encrypt(*bit));}
        let t0 = Instant::now();
        let mut eval = crate::tfhe_x::freexor::Evaluator::new(&ck as *const FxClientKey);
        let ct_output = eval.evaluate(circ,&ct_input,&sk);
        report.elapsed_nanos.push(t0.elapsed().as_nanos());
        let mut output = Vec::<bool>::new();
        for ct in ct_output {output.push(ck.decrypt(&ct));}
        report.succeeded = ref_output == output;
        if ! report.succeeded
        {
            report.err_msg = format!("input={:?}, output={:?}, expected={:?}",bools_to_hex(&input),bools_to_hex(&output),bools_to_hex(&ref_output));
        }
    }

    fn _test(path:&str,mode:&str,num_trials:usize) -> Report
    {
        let mut report = Report::new();
        let (circ,testdata) = Test::parse_files(path);
        Test::report_circuit(&circ, &mut report);
        report.circ_name = Path::new(path).file_name().unwrap().to_string_lossy().to_string();
        report.show_circuit_info(format!("{BLUE}{}>{RESET} ",mode).as_str());

        let inlen = circ.total_num_of_input_pins() as usize;
        for (idx,d) in testdata.points.iter().enumerate() 
        {
            if idx>=num_trials{break;}
            print!("\tSample {} ... ",idx+1); io::stdout().flush().unwrap();
            let input = &d[..inlen];
            let ref_output = &d[inlen..];
            if mode=="Plain" {Test::eval_plain(&circ, input, ref_output, &mut report);}
            else if mode == "Standard" {Test::eval_homomorphic_std(&circ, input, ref_output, &mut report);}
            else {Test::eval_homomorphic_freexor(&circ, input, ref_output, &mut report);}

            println!("({} ns)", report.elapsed_nanos[idx]);
            if !report.succeeded 
            {
                report.show_execution_state();
                break;
            }
        }
        report.show_avg_time("\t");
        return  report;
    }

    pub fn plain(path:&str,num_of_trials:usize) -> Report
    {
        return Test::_test(path,"Plain",num_of_trials);
    }  

    pub fn homomorphic_std(path:&str,num_of_trials:usize) -> Report
    {
        return Test::_test(path, "Standard",num_of_trials);
    }

    pub fn homomorphic_freexor(path:&str,num_of_trials:usize) -> Report
    {
        return Test::_test(path, "FreeXOR",num_of_trials);
    }

    pub fn test_gates(x:u64,y:u64,gtype:GType)
    {
        let (mut ck,sk) = crate::tfhe_x::freexor::generate_keys();
        let ct_x = ck.encrypt_small(x); 
        let ct_y = ck.encrypt_small(y);

        let ct_z = match gtype {
            GType::XOR => sk.xor(&ct_x, &ct_y),
            GType::XNOR => sk.xnor(&ct_x, &ct_y),
            GType::AND => sk.and(&ct_x, &ct_y),
            GType::NAND => sk.nand(&ct_x, &ct_y),
            GType::NOR => sk.nor(&ct_x, &ct_y),
            GType::OR => sk.or(&ct_x, &ct_y),
            _ => panic!("not supported")
        };

        let z = ck.decrypt_small(&ct_z);
        let a = ck.decrypt(&ct_x);
        let b = ck.decrypt(&ct_y);
        let c = ck.decrypt(&ct_z);
        println!("x={},y={},z={},a={},b={},c={}", x,y,z,a,b,c); 
    }
}