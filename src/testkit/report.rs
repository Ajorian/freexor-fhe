use std::collections::HashMap;

pub const BLACK: &str = "\x1b[30m";
pub const RED: &str = "\x1b[31m";
pub const GREEN: &str = "\x1b[32m";
pub const YELLOW: &str = "\x1b[33m";
pub const BLUE: &str = "\x1b[34m";
pub const MAGENTA: &str = "\x1b[35m";
pub const CYAN: &str = "\x1b[36m";
pub const WHITE: &str = "\x1b[37m";

pub const BRIGHT_BLACK: &str = "\x1b[90m"; // Gray
pub const BRIGHT_RED: &str = "\x1b[91m";
pub const BRIGHT_GREEN: &str = "\x1b[92m";
pub const BRIGHT_YELLOW: &str = "\x1b[93m";
pub const BRIGHT_BLUE: &str = "\x1b[94m";
pub const BRIGHT_MAGENTA: &str = "\x1b[95m";
pub const BRIGHT_CYAN: &str = "\x1b[96m";
pub const BRIGHT_WHITE: &str = "\x1b[97m";
pub const RESET: &str = "\x1b[0m";
pub const BOLD: &str = "\x1b[1m";
pub const BOLD_RED: &str = "\x1b[1;31m";
pub const HIGHLIGHT: &str = "\x1b[43m"; // Yellow background

#[derive(Debug)]
pub struct Report
{
    pub circ_name:String,
    pub elapsed_nanos:Vec<u128>,
    pub succeeded:bool,
    pub gates:HashMap<String,u32>,
    pub err_msg:String,
}

impl Report 
{
    pub fn new()-> Self
    {
        Report
        {
            circ_name:String::new(),
            elapsed_nanos:Vec::<u128>::new(),
            succeeded:false,
            gates:HashMap::<String,u32>::new(),
            err_msg:String::new()
        }  
    } 

    pub fn show_circuit_info(&self,prelude:&str)
    {
        print!("{}{BOLD}{}{RESET}: ",prelude,self.circ_name);
        println!("{}",self.circuit_statistics());
    }

    pub fn show_avg_time(&self,prelude:&str)
    {
        let avg = self.avg_elapsed_seconds();
        println!("{}Average time {HIGHLIGHT}{:.3}{RESET} seconds for {} samples",prelude,avg,self.elapsed_nanos.len());
    }

    pub fn show_execution_state(&self)
    {
        if self.succeeded
        {
            println!("\n{GREEN}{BOLD}Succeeded{RESET}: pased all the points in the scenario");
        }
        else
        {
            println!("{RED}{BOLD}Faild{RESET} on sample {}", self.elapsed_nanos.len());
            println!("{RED}{BOLD}Error{BOLD}{RESET}:{}", self.err_msg);
        }
    }

    pub fn show(&self)
    {
        self.show_circuit_info("");
        self.show_avg_time("");
        self.show_execution_state();
    }

    pub fn avg_elapsed_seconds(&self) -> f64
    {
        let avg_ns = self.elapsed_nanos.iter().sum::<u128>() / self.elapsed_nanos.len() as u128;
        let avg = avg_ns as f64 / 1_000_000_000.0;
        return avg;
    }

    pub fn circuit_statistics(&self)-> String
    {
        let mut res = String::new();
        let end = self.gates.len()-1;
        for (i,(g,cnt)) in self.gates.iter().enumerate()
        {
            res.push_str(&format!("{} {}{}",cnt,g, if i==end{""}else{", "}));
        }
        return res;
    }
}

pub fn bools_to_hex(bits: &[bool]) -> String 
{
    const HEX: &[u8] = b"0123456789ABCDEF";
    let mut digits = String::new();

    if bits.is_empty() {return "0x0".to_string();}

    for chunk_start in (0..bits.len()).step_by(4) 
    {
        let mut nibble = 0u8;

        for j in 0..4 
        {
            let i = chunk_start + j;
            if i < bits.len() && bits[i] {nibble |= 1 << j;}
        }

        digits.push(HEX[nibble as usize] as char);
    }

    while digits.len() > 1 && digits.ends_with('0') 
    {
        digits.pop();
    }

    format!("0x{}", digits.chars().rev().collect::<String>())
}
