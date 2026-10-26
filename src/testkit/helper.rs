use colored::Colorize;

pub fn to_bits_unsigned(s: &str, size: usize) -> Vec<bool>
{
    let x: u64 = s.parse().unwrap();
    (0..size).map(|i| ((x >> i) & 1) != 0).collect()
}

pub fn to_bits_signed(s: &str, bits: usize) -> Vec<bool> 
{
    let x: i64 = s.parse().unwrap();
    let y = x as u64;
    (0..bits).map(|i| ((y >> i) & 1) != 0).collect()
}

pub fn to_bits_f64(s: &str) -> Vec<bool>
{
    let x: f64 = s.parse().unwrap();
    let y = x.to_bits(); // u64 containing IEEE-754 representation
    (0..64).map(|i| ((y >> i) & 1) != 0).collect()
}

pub fn to_bits_hex(hex: &str, size: usize) -> Vec<bool> 
{
    let mut bits = Vec::new();

    for c in hex.chars().rev() 
    {
        let n = c.to_digit(16).unwrap();
        bits.push((n & 1) != 0);
        bits.push((n & 2) != 0);
        bits.push((n & 4) != 0);
        bits.push((n & 8) != 0);
    }

    bits.resize(size, false);
    assert!(bits.len() <= size, "hex string {} is larger than {} bits",hex.red(),size);
    bits
}

pub fn to_bits_boolean(s: &str) -> bool
{
    match s.to_uppercase().as_str()
    {
        "TRUE" => {return true;}
        "FALSE" => {return false;}
        _ => {assert!(false,"invalid bool value {}",s);}
    }
    false
}

