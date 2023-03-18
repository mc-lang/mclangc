
#[derive(Debug, Clone, PartialEq)]
pub enum OpType {
    
    // stack
    PushInt,
    PushStr,
    Drop,
    Print,
    Dup,
    Dup2, // a b => a b a b
    Rot, // a b c => b c a
    Over, // a b => a b a
    Swap, // a b => b a

    // math
    Minus,
    Plus,
    Equals,
    Gt,
    Lt,
    Band, // &
    Bor, // |
    Shr, // >>
    Shl,  // <<
    Div, // /
    Mul,
    
    
    // mem
    Mem,
    Load8,
    Store8,

    // block
    If,
    Else,
    End,
    While,
    Do,
    
    // syscalls
    Syscall0,
    Syscall1,
    Syscall2,
    Syscall3,
    Syscall4,
    Syscall5,
    Syscall6,


}

#[derive(Debug, Clone)]
pub struct Operator {
    pub typ: OpType,
    pub value: i64,
    pub text: String, //? only used for OpType::PushStr
    pub addr: i64, //? only used for OpType::PushStr
    pub jmp: i32,
    pub pos: (String, u32, u32)
}

impl Operator {
    pub fn new(typ: OpType, value: i64, text: String, file: String, row: u32, col: u32) -> Self {
        Self {
            typ,
            value,
            jmp: 0,
            addr: -1,
            text,
            pos: (file, row, col)
        }
    }
}


#[derive(Debug, Clone)]
pub struct Token {
    pub file: String,
    pub line: u32,
    pub col: u32,
    pub text: String,
    pub typ: TokenType
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Word,
    Int,
    String,
    //TODO: Add char
}