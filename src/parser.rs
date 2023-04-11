use std::ops::Deref;

use crate::{constants::{Operator, OpType, Token, TokenType, Loc, KeywordType, InstructionType}, lerror, preprocessor::Preprocessor, Args};
use color_eyre::Result;
use eyre::eyre;

pub fn cross_ref(mut program: Vec<Operator>) -> Result<Vec<Operator>> {
    let mut stack: Vec<usize> = Vec::new();

    for ip in 0..program.len() {
        let op = &program.clone()[ip];
        match op.typ {
            OpType::Keyword(KeywordType::If) | 
            OpType::Keyword(KeywordType::Function) | 
            OpType::Keyword(KeywordType::While) => {
                stack.push(ip);
            }
            OpType::Keyword(KeywordType::Else) => {
                let if_ip = if let Some(x) = stack.pop() { x } else {
                    lerror!(&op.loc, "Unclosed-if else block");
                    return Err(eyre!("Cross referencing"));
                };
                if program[if_ip].typ != OpType::Keyword(KeywordType::If) {
                    lerror!(&op.clone().loc,"'else' can only close 'if' blocks");
                    return Err(eyre!("Bad block"));
                }
                
                program[if_ip].jmp = ip + 1;
                stack.push(ip);
            },
            OpType::Keyword(KeywordType::End) => {
                let block_ip = if let Some(block_ip) = stack.pop() { block_ip } else {
                    lerror!(&op.loc, "Unclosed if, if-else, while-do, function, memory, or constant");
                    return Err(eyre!("Cross referencing"));
                };

                match &program[block_ip].typ {
                    OpType::Keyword(KeywordType::If) |
                    OpType::Keyword(KeywordType::Else) => {
                        program[block_ip].jmp = ip;
                        program[ip].jmp = ip + 1;
                    }

                    OpType::Keyword(KeywordType::Do) => {
                        program[ip].jmp = program[block_ip].jmp;
                        program[block_ip].jmp = ip + 1;
                    }
                    OpType::Keyword(KeywordType::FunctionDo) => {
                        program[ip].typ = OpType::Instruction(InstructionType::Return);
                    }
                    OpType::Keyword(KeywordType::Memory) |
                    OpType::Keyword(KeywordType::Function) |
                    OpType::Keyword(KeywordType::Constant) => (),

                    a => {
                        println!("{a:?}");
                        lerror!(&op.clone().loc,"'end' can only close if, if-else, while-do, function, memory, or constant blocks");
                        return  Err(eyre!(""));
                    }
                }

            }
            OpType::Keyword(KeywordType::Do) => {
                let block_ip = if let Some(x) = stack.pop() { x } else {
                    lerror!(&op.loc, "Unclosed while-do block");
                    return Err(eyre!("Cross referencing"));
                };

                if program[block_ip].typ == OpType::Keyword(KeywordType::Function) {
                    program[ip].typ = OpType::Keyword(KeywordType::FunctionDo);
                }

                program[ip].jmp = block_ip;
                stack.push(ip);
            }
            _ => ()
        }

    }
    if !stack.is_empty() {
        println!("{:?}", stack);
        lerror!(&program[stack.pop().expect("Empy stack")].clone().loc,"Unclosed block, {:?}", program[stack.pop().expect("Empy stack")].clone());
        return Err(eyre!("Unclosed block"));
    }

    Ok(program.clone())
}

pub struct Parser {
    tokens: Vec<Token>
}

impl Parser {
    pub fn new(file: Vec<Token>) -> Self {
        Self{
            tokens: file
        }
    }

    pub fn parse(&mut self, args: &Args) -> Result<Vec<Operator>> {
        let mut tokens = Vec::new();

        for token in &self.tokens {
            if token.text.is_empty() {
                continue;
            }
            let pos = (token.file.clone(), token.line, token.col);
            match token.typ {
                TokenType::Word => {
                    let word_type = if token.op_typ == OpType::Instruction(InstructionType::MemUse) {
                        OpType::Instruction(InstructionType::MemUse)
                    } else {
                        lookup_word(&token.text, &pos)
                    };

                    tokens.push(Operator::new(word_type, token.typ, token.value.unwrap_or(0), token.text.clone(), token.file.clone(), token.line, token.col).set_addr(token.addr.unwrap_or(0)));
                },
                TokenType::Int => {// negative numbers not yet implemented
                    tokens.push(Operator::new(OpType::Instruction(InstructionType::PushInt), token.typ, token.text.parse::<usize>()?, String::new(), token.file.clone(), token.line, token.col));
                },
                TokenType::String => {
                    tokens.push(Operator::new(OpType::Instruction(InstructionType::PushStr), token.typ, 0, token.text.clone(), token.file.clone(), token.line, token.col));
                }
                TokenType::Char => {
                    let c = token.text.clone();
                    if c.len() != 1 {
                        lerror!(&token.loc(), "Chars can only be of lenght 1, got {}", c.len());
                        return Err(eyre!(""));
                    }

                    tokens.push(Operator::new(OpType::Instruction(InstructionType::PushInt), token.typ, token.text.chars().next().unwrap() as usize, String::new(), token.file.clone(), token.line, token.col));
                }
            };


        }

        let t = Preprocessor::new(tokens.clone(), args).preprocess()?.get_ops();
        let t = cross_ref(t.clone())?;

        Ok(t)
    }
}


pub fn lookup_word<P: Deref<Target = Loc>>(s: &str, _pos: P) -> OpType {
    let n = s.parse::<usize>();
    if let Ok(_) = n {
        return OpType::Instruction(InstructionType::PushInt);
    }
    match s {
        //stack
        "print" => OpType::Instruction(InstructionType::Print),
        "dup" => OpType::Instruction(InstructionType::Dup),
        "drop" => OpType::Instruction(InstructionType::Drop),
        "rot" => OpType::Instruction(InstructionType::Rot),
        "over" => OpType::Instruction(InstructionType::Over),
        "swap" => OpType::Instruction(InstructionType::Swap),

        // comp and math
        "+" => OpType::Instruction(InstructionType::Plus),
        "-" => OpType::Instruction(InstructionType::Minus),
        "=" => OpType::Instruction(InstructionType::Equals),
        "!=" => OpType::Instruction(InstructionType::NotEquals),
        ">" => OpType::Instruction(InstructionType::Gt),
        "<" => OpType::Instruction(InstructionType::Lt),
        ">=" => OpType::Instruction(InstructionType::Ge),
        "<=" => OpType::Instruction(InstructionType::Le),
        
        "band" => OpType::Instruction(InstructionType::Band),
        "bor" => OpType::Instruction(InstructionType::Bor),
        "shr" => OpType::Instruction(InstructionType::Shr),
        "shl" => OpType::Instruction(InstructionType::Shl),
        "divmod" => OpType::Instruction(InstructionType::DivMod),
        "*" => OpType::Instruction(InstructionType::Mul),
        
        
        // mem
        "load8" => OpType::Instruction(InstructionType::Load8),
        "store8" => OpType::Instruction(InstructionType::Store8),
        "load32" => OpType::Instruction(InstructionType::Load32),
        "store32" => OpType::Instruction(InstructionType::Store32),
        "load64" => OpType::Instruction(InstructionType::Load64),
        "store64" => OpType::Instruction(InstructionType::Store64),
        
        "syscall0" => OpType::Instruction(InstructionType::Syscall0),
        "syscall1" => OpType::Instruction(InstructionType::Syscall1),
        "syscall2" => OpType::Instruction(InstructionType::Syscall2),
        "syscall3" => OpType::Instruction(InstructionType::Syscall3),
        "syscall4" => OpType::Instruction(InstructionType::Syscall4),
        "syscall5" => OpType::Instruction(InstructionType::Syscall5),
        "syscall6" => OpType::Instruction(InstructionType::Syscall6),
        "cast(bool)" => OpType::Instruction(InstructionType::CastBool),
        "cast(ptr)" => OpType::Instruction(InstructionType::CastPtr),
        "cast(int)" => OpType::Instruction(InstructionType::CastInt),
        "cast(void)" => OpType::Instruction(InstructionType::CastVoid),
        // block
        "if" => OpType::Keyword(KeywordType::If),
        "else" => OpType::Keyword(KeywordType::Else),
        "end" => OpType::Keyword(KeywordType::End),
        "while" => OpType::Keyword(KeywordType::While),
        "do" => OpType::Keyword(KeywordType::Do),
        "include" => OpType::Keyword(KeywordType::Include),
        "memory" => OpType::Keyword(KeywordType::Memory),
        "const" => OpType::Keyword(KeywordType::Constant),
        "fn" => OpType::Keyword(KeywordType::Function),
        "return" => OpType::Instruction(InstructionType::Return),
        "returns" => OpType::Instruction(InstructionType::Returns),
        "bool" => OpType::Instruction(InstructionType::TypeBool),
        "int" => OpType::Instruction(InstructionType::TypeInt),
        "ptr" => OpType::Instruction(InstructionType::TypePtr),
        "void" => OpType::Instruction(InstructionType::TypeVoid),
        "any" => OpType::Instruction(InstructionType::TypeAny),
        "str" => OpType::Instruction(InstructionType::TypeStr),
        "with" => OpType::Instruction(InstructionType::With),
        _ => OpType::Instruction(InstructionType::None)
    }

}