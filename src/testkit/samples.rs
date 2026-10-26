use std::fs::File;
use std::io::{BufRead,BufReader};
use super::helper::*;

pub struct TestData
{
    pub types:Vec<String>,
    pub sizes:Vec<usize>,
    pub points:Vec<Vec<bool>>,
}

impl TestData
{
    pub fn new(sizes:&[usize], path:&str) -> Self
    {
        let file = File::open(path).expect("Failed to open file");
        let reader = BufReader::new(file);
        let mut points = Vec::<Vec<bool>>::new();
        let mut types = Vec::<String>::new();

        let mut first = true;
        for l in reader.lines()
        {
            let line = match l {Ok(line) => line,Err(_) => panic!("failed")};
            if line.is_empty() {continue;}
            if first
            {
                first = false;
                types = TestData::parse_types(line.as_str());
                assert!(sizes.len() == types.len(),"TestPoints: invalid parameters");
            }
            else{points.push(TestData::parse_test_point(sizes,&types,&line));}
        }
        
        Self{types:types,sizes:sizes.to_vec(), points:points}
    }

    fn parse_types(types_str:&str) -> Vec::<String>
    {
        let mut res = Vec::<String>::new();
        const VALID_TYPES: &[&str] = &["bool", "int", "uint", "f64","hex"];
        let tokens:Vec<&str> = types_str.trim().split(",").map(|x| x.trim()).collect();
        for typ in &tokens
        {
            assert!(VALID_TYPES.contains(typ), "TestPoints: invalid type {}", typ);
            res.push(typ.to_string());
        }
        return res;
    }

    fn parse_test_point(sizes:&[usize],types:&[String],point_str:&str) -> Vec<bool>
    {
        let tokens:Vec<&str> = point_str.trim().split(",").map(|x| x.trim()).collect();
        assert!(tokens.len() == sizes.len(), "TestPoints: invalid data item {}",point_str);
        
        let mut vls = Vec::<bool>::new();
        for i in 0..tokens.len()
        {
            let val = &tokens[i];
            let vtype = &types[i];
            let size = sizes[i];
            
            match vtype.as_str()
            {
                "bool" => {vls.push(to_bits_boolean(val));}
                "int" => {vls.append(&mut to_bits_signed(val,size));}
                "uint" => {vls.append(&mut to_bits_unsigned(val,size));}
                "hex" => {vls.append(&mut to_bits_hex(val,size));}
                "f64" => {vls.append(&mut to_bits_f64(val));}
                _ =>{assert!(false, "Tester: not supported type {}", vtype);}
            }   
        } 
        return vls;       
    }

}