
#[derive(Debug)]
pub enum OpType {
    Push,
    Pop,
    Minus,
    Plus,
    Equals,
    Print
}


// #[derive(Debug)]
// pub enum OpType {

// }

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


#[derive(Debug)]
pub struct Token {
    pub file: String,
    pub line: u32,
    pub col: u32,
    pub text: String
}