
#[derive(Debug, Clone, PartialEq)]
pub enum OpType {
    Push,
    Pop,
    Minus,
    Plus,
    Equals,
    Print,
    If,
    Else,
    End,
    Dup,
    Gt,
    Lt,
    While,
    Do,
    Mem,
    Load8,
    Store8,
    Syscall0,
    Syscall1,
    Syscall2,
    Syscall3,
    Syscall4,
    Syscall5,
    Syscall6
}


// #[derive(Debug)]
// pub enum OpType {

// }

#[derive(Debug, Clone)]
pub struct Operator {
    pub typ: OpType,
    pub value: i64,
    pub jmp: i32,
    pub pos: (String, u32, u32)
}

impl Operator {
    pub fn new(typ: OpType, value: i64, file: String, row: u32, col: u32) -> Self {
        Self {
            typ,
            value,
            jmp: 0,
            pos: (file, row, col)
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