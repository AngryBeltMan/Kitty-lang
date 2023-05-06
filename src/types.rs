// #![allow(unused_assignments,unused_variables)]
#[derive(Debug,Clone)]
pub enum VarType {
    Int(i32),
    UnInt(u32),
    Float(f32),
    BigInt(i64),
    BigUnInt(u64),
    BigFloat(f64),
    SmallInt(i8),
    Byte(u8),
    Bool(bool),
    String(String),
}
impl VarType {
    pub fn add(&mut self,num:f64) -> Result<(),String> {
        match *self {
            VarType::Int(n) => {*self = VarType::Int(n + num as i32);},
            VarType::UnInt(n) => {*self = VarType::UnInt(n + num as u32);},
            VarType::Float(n) => { *self = VarType::Float(n + num as f32);},
            VarType::BigInt(n) => { *self = VarType::BigInt(n + num as i64);},
            VarType::BigUnInt(n) => {  *self = VarType::BigUnInt(n + num as u64); },
            VarType::BigFloat(n) => {  *self = VarType::BigFloat(n + num as f64);},
            VarType::SmallInt(n) => {  *self = VarType::SmallInt(n + num as i8);},
            VarType::Byte(n) => {  *self = VarType::Byte(n + num as u8);},
            _ => return Err(String::from("Unsupported type"))
        }
        Ok(())
    }
    pub fn subtract(&mut self,num:f64) -> Result<(),String> {
        self.add(-num)
    }
    pub fn new(var_type:&str,value:&str) -> anyhow::Result<Self,> {
        match var_type.to_lowercase().as_str() {
            "int" => { Ok(VarType::Int(value.parse()?)) },
            "bigint" => { Ok(VarType::BigInt(value.parse()?)) },
            "smallint" => { Ok(VarType::SmallInt(value.parse()?)) },
            "uint" => { Ok(VarType::UnInt(value.parse()?)) },
            "biguint" => { Ok(VarType::BigUnInt(value.parse()?)) },
            "byte" => { Ok(VarType::Byte(value.parse()?)) },
            "float" => { Ok(VarType::Float(value.parse()?)) },
            "bigfloat" => { Ok(VarType::Float(value.parse()?)) },
            "string" => { Ok(VarType::String(value.to_string())) },
            "bool" => { Ok(VarType::Bool(value.parse()?)) },
            "bint" => { Ok(VarType::BigInt(value.parse()?)) },
            _ => panic!("invalid type")
        }
    }
}
