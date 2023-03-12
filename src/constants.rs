
#[derive(Debug)]
pub enum OpType {
    Push,
    Pop,
    Minus,
    Plus,
    Print
}

#[derive(Debug)]
pub struct Operator {
    pub typ: OpType,
    pub value: i32,
}

impl Operator {
    pub fn new(typ: OpType, value: i32) -> Self {
        Self {
            typ,
            value
        }
    }
}


pub struct Token {
    file: String,
    line: u32,
    col: u32,
    text: String
}