
#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum GType
{
    AND,NAND,OR,NOR,XOR,XNOR,INV,NOT,EQ,EQW,ERR 
}

#[derive(Debug)]
pub struct Gate
{
    pub gtype:GType,
    pub inputs: Vec<u32>,
    pub output:u32
}

#[derive(Debug)]
pub struct Circuit
{
    pub n_wires:u32,
    pub inputs: Vec<(u32,u32)>,
    pub outputs: Vec<(u32,u32)>,
    pub gates:Vec<Gate>
}

pub fn parse_gtype(s: &str) -> GType
{
    match s 
    {
        "AND" => GType::AND,
        "NAND" => GType::NAND,
        "OR" => GType::OR,
        "NOR" => GType::NOR,
        "XOR" => GType::XOR,
        "XNOR" => GType::XNOR,
        "INV" => GType::INV,
        "NOT" => GType::NOT,
        "EQ" => GType::EQ,
        "EQW" => GType::EQW,
        _ => GType::ERR,
    }
}

pub fn gtype_to_str(gtype: &GType) -> &'static str
{
    match gtype 
    {
        GType::AND  => "AND",
        GType::NAND => "NAND",
        GType::OR   => "OR",
        GType::NOR  => "NOR",
        GType::XOR  => "XOR",
        GType::XNOR => "XNOR",
        GType::INV  => "INV",
        GType::NOT  => "NOT",
        GType::EQ   => "EQ",
        GType::EQW  => "EQW",
        GType::ERR  => "ERR",
    }
}