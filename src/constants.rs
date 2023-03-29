
pub const ALLOW_MACRO_REDEFINITION: bool = true;


#[derive(Debug, Clone, PartialEq)]
pub enum InstructionType {
    
    // stack
    PushInt,
    PushStr,
    Drop,
    Print,
    Dup,
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
#[derive(Debug, Clone, PartialEq)]
pub enum KeywordType {
    If,
    Else,
    End,
    While,
    Do,
    Macro,
    Include,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PreprocessorType {
    IfDefined,
    IfNotDefined,
    Else,
    EndIf,
    Define
}

#[derive(Debug, Clone, PartialEq)]
pub enum OpType {
    Keyword(KeywordType),
    Instruction(InstructionType),
    Preprocessor(PreprocessorType)
}

#[derive(Debug, Clone)]
pub struct Operator{
    pub typ: OpType,
    pub value: usize,
    pub text: String, //? only used for OpType::PushStr
    pub addr: Option<usize>, //? only used for OpType::PushStr
    pub jmp: usize,
    pub loc: (String, usize, usize)
}

impl Operator {
    pub fn new(typ: OpType, value: usize, text: String, file: String, row: usize, col: usize) -> Self {
        Self {
            typ,
            value,
            jmp: 0,
            addr: None,
            text,
            loc: (file, row, col)
        }
    }
    
}

impl OpType {
    pub fn human(&self) -> String {
        match *self {
            OpType::Instruction(InstructionType::PushInt) => "Number",
            OpType::Instruction(InstructionType::PushStr) => "String",
            OpType::Instruction(InstructionType::Print) => "print",
            OpType::Instruction(InstructionType::Dup) => "dup",
            OpType::Instruction(InstructionType::Drop) => "drop",
            OpType::Instruction(InstructionType::Rot) => "rot",
            OpType::Instruction(InstructionType::Over) => "over",
            OpType::Instruction(InstructionType::Swap) => "swap",
            OpType::Instruction(InstructionType::Plus) => "+",
            OpType::Instruction(InstructionType::Minus) => "-",
            OpType::Instruction(InstructionType::Equals) => "=",
            OpType::Instruction(InstructionType::Gt) => ">",
            OpType::Instruction(InstructionType::Lt) => "<",
            OpType::Instruction(InstructionType::NotEquals) => "!=",
            OpType::Instruction(InstructionType::Le) => "<=",
            OpType::Instruction(InstructionType::Ge) => ">=",
            OpType::Instruction(InstructionType::Band) => "band",
            OpType::Instruction(InstructionType::Bor) => "bor",
            OpType::Instruction(InstructionType::Shr) => "shr",
            OpType::Instruction(InstructionType::Shl) => "shl",
            OpType::Instruction(InstructionType::DivMod) => "divmod",
            OpType::Instruction(InstructionType::Mul) => "*",
            OpType::Keyword(KeywordType::If) => "if",
            OpType::Keyword(KeywordType::Else) => "else",
            OpType::Keyword(KeywordType::End) => "end",
            OpType::Keyword(KeywordType::While) => "while",
            OpType::Keyword(KeywordType::Do) => "do",
            OpType::Keyword(KeywordType::Macro) => "macro",
            OpType::Keyword(KeywordType::Include) => "include",
            OpType::Instruction(InstructionType::Mem) => "mem",
            OpType::Instruction(InstructionType::Load8) => "!8",
            OpType::Instruction(InstructionType::Store8) => "@8",
            OpType::Instruction(InstructionType::Syscall0) => "syscall0",
            OpType::Instruction(InstructionType::Syscall1) => "syscall1",
            OpType::Instruction(InstructionType::Syscall2) => "syscall2",
            OpType::Instruction(InstructionType::Syscall3) => "syscall3",
            OpType::Instruction(InstructionType::Syscall4) => "syscall4",
            OpType::Instruction(InstructionType::Syscall5) => "syscall5",
            OpType::Instruction(InstructionType::Syscall6) => "syscall6",
            OpType::Instruction(InstructionType::None) => "None",
            OpType::Preprocessor(PreprocessorType::IfDefined) => "#ifdef",
            OpType::Preprocessor(PreprocessorType::IfNotDefined) => "#ifndef",
            OpType::Preprocessor(PreprocessorType::Else) => "#else",
            OpType::Preprocessor(PreprocessorType::EndIf) => "#endif",
            OpType::Preprocessor(PreprocessorType::Define) => "#define",
        }.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub file: String,
    pub line: usize,
    pub col: usize,
    pub text: String,
    pub typ: TokenType
}

#[derive(Debug, Clone, PartialEq, Copy)]
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
    pub fn human(self) -> String {
        match self {
            TokenType::Word => "Word",
            TokenType::Int => "Int",
            TokenType::String => "String",
            TokenType::Char => "Char"
        }.to_string()
    }
}

pub type Loc = (String, usize, usize);
pub mod targets {
    pub const LINUX_X86_64: &'static str = "linux_x86_64";
    pub const WIN32_X86_64: &'static str = "win32_x86_64";
}

pub fn get_win32_syscall(n: usize) {
    match n {
        
        _ => panic!("Unknown syscall {n}")
    }


}