
pub const ALLOW_MACRO_REDEFINITION: bool = true;


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
    Ge,
    Le,
    NotEquals,
    Band, // &
    Bor, // |
    Shr, // >>
    Shl,  // <<
    DivMod, // /
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
    Macro,
    Include,
    
    // syscalls
    Syscall0,
    Syscall1,
    Syscall2,
    Syscall3,
    Syscall4,
    Syscall5,
    Syscall6,

    None // Used for macros and any other non built in word definitions

}

#[derive(Debug, Clone)]
pub struct Operator {
    pub typ: OpType,
    pub value: i64,
    pub text: String, //? only used for OpType::PushStr
    pub addr: i64, //? only used for OpType::PushStr
    pub jmp: i32,
    pub loc: (String, u32, u32)
}

impl Operator {
    pub fn new(typ: OpType, value: i64, text: String, file: String, row: u32, col: u32) -> Self {
        Self {
            typ,
            value,
            jmp: 0,
            addr: -1,
            text,
            loc: (file, row, col)
        }
    }
    
}

impl OpType {
    pub fn human(&self) -> String {
        match self {
            &OpType::PushInt => "Number",
            &OpType::PushStr => "String",
            &OpType::Print => "print",
            &OpType::Dup => "dup",
            &OpType::Drop => "drop",
            &OpType::Dup2 => "2dup",
            &OpType::Rot => "rot",
            &OpType::Over => "over",
            &OpType::Swap => "swap",
            &OpType::Plus => "+",
            &OpType::Minus => "-",
            &OpType::Equals => "=",
            &OpType::Gt => ">",
            &OpType::Lt => "<",
            &OpType::NotEquals => "!=",
            &OpType::Le => "<=",
            &OpType::Ge => ">=",
            &OpType::Band => "band",
            &OpType::Bor => "bor",
            &OpType::Shr => "shr",
            &OpType::Shl => "shl",
            &OpType::DivMod => "divmod",
            &OpType::Mul => "*",
            &OpType::If => "if",
            &OpType::Else => "else",
            &OpType::End => "end",
            &OpType::While => "while",
            &OpType::Do => "do",
            &OpType::Macro => "macro",
            &OpType::Include => "include",
            &OpType::Mem => "mem",
            &OpType::Load8 => "!8",
            &OpType::Store8 => "@8",
            &OpType::Syscall0 => "syscall0",
            &OpType::Syscall1 => "syscall1",
            &OpType::Syscall2 => "syscall2",
            &OpType::Syscall3 => "syscall3",
            &OpType::Syscall4 => "syscall4",
            &OpType::Syscall5 => "syscall5",
            &OpType::Syscall6 => "syscall6",
            &OpType::None => "None"
        }.to_string()
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
    Char
}

impl Token {
    pub fn loc(&self) -> Loc {
        (
            self.file.clone(),
            self.line,
            self.col
        )
    }
}

impl TokenType {
    pub fn human(&self) -> String {
        match self {
            TokenType::Word => "Word",
            TokenType::Int => "Int",
            TokenType::String => "String",
            TokenType::Char => "Char"
        }.to_string()
    }
}

pub type Loc = (String, u32, u32);