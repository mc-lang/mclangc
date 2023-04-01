use std::ops::Deref;

use crate::{constants::{Operator, OpType, Token, TokenType, Loc, KeywordType, InstructionType}, lerror};
use color_eyre::Result;
use eyre::eyre;

pub fn cross_ref(mut program: Vec<Operator>) -> Result<Vec<Operator>> {
    let mut stack: Vec<usize> = Vec::new();
    for ip in 0..program.len() {
        let op = &program.clone()[ip];
        match op.typ {
            OpType::Keyword(KeywordType::If) | 
            OpType::Keyword(KeywordType::While) => {
                stack.push(ip);
            }
            OpType::Keyword(KeywordType::Else) => {
                let if_ip = stack.pop().unwrap();
                if program[if_ip].typ != OpType::Keyword(KeywordType::If) {
                    lerror!(&op.clone().loc,"'end' can only close 'if' blocks");
                    return Err(eyre!("Bad block"));
                }
                
                program[if_ip].jmp = ip + 1;
                stack.push(ip);
            },
            OpType::Keyword(KeywordType::End) => {
                let block_ip = stack.pop().unwrap();

                if program[block_ip].typ == OpType::Keyword(KeywordType::If) || 
                   program[block_ip].typ == OpType::Keyword(KeywordType::Else) {
                    
                    program[block_ip].jmp = ip;
                    program[ip].jmp = ip + 1;

                } else if program[block_ip].typ == OpType::Keyword(KeywordType::Do) {
                    program[ip].jmp = program[block_ip].jmp;
                    program[block_ip].jmp = ip + 1;
                } else {
                    lerror!(&op.clone().loc,"'end' can only close 'if' blocks");
                    return  Err(eyre!(""));
                }

            }
            OpType::Keyword(KeywordType::Do) => {
                let while_ip = stack.pop().unwrap();
                program[ip].jmp = while_ip;
                stack.push(ip);
            }
            _ => ()
        }

    }
    if !stack.is_empty() {
        lerror!(&program[stack.pop().expect("Empy stack")].clone().loc,"Unclosed block");
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

    pub fn parse(&mut self) -> Result<Vec<Operator>> {
        let mut tokens = Vec::new();

        for token in &self.tokens {
            if token.text.is_empty() {
                continue;
            }
            let pos = (token.file.clone(), token.line, token.col);
            match token.typ {
                TokenType::Word => {
                    let word_type = if token.op_typ == InstructionType::MemUse {
                        OpType::Instruction(InstructionType::MemUse)
                    } else {
                        lookup_word(&token.text, &pos)
                    };

                    tokens.push(Operator::new(word_type, token.value.unwrap_or(0), token.text.clone(), token.file.clone(), token.line, token.col).set_addr(token.addr.unwrap_or(0)));
                },
                TokenType::Int => {// negative numbers not yet implemented
                    tokens.push(Operator::new(OpType::Instruction(InstructionType::PushInt), token.text.parse::<usize>()?, String::new(), token.file.clone(), token.line, token.col));
                },
                TokenType::String => {
                    tokens.push(Operator::new(OpType::Instruction(InstructionType::PushStr), 0, token.text.clone(), token.file.clone(), token.line, token.col));
                }
                TokenType::Char => {
                    let c = token.text.clone();
                    if c.len() != 1 {
                        lerror!(&token.loc(), "Chars can only be of lenght 1, got {}", c.len());
                        return Err(eyre!(""));
                    }

                    tokens.push(Operator::new(OpType::Instruction(InstructionType::PushInt), token.text.chars().next().unwrap() as usize, String::new(), token.file.clone(), token.line, token.col));
                }
            };

            
            //"print" => tokens.push(Operator::new(OpType::Print, 0, token.file.clone(), token.line, token.col)),
        }

        cross_ref(tokens)
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
        "mem" => OpType::Instruction(InstructionType::Mem),
        "load8" => OpType::Instruction(InstructionType::Load8),
        "store8" => OpType::Instruction(InstructionType::Store8),
        
        "syscall0" => OpType::Instruction(InstructionType::Syscall0),
        "syscall1" => OpType::Instruction(InstructionType::Syscall1),
        "syscall2" => OpType::Instruction(InstructionType::Syscall2),
        "syscall3" => OpType::Instruction(InstructionType::Syscall3),
        "syscall4" => OpType::Instruction(InstructionType::Syscall4),
        "syscall5" => OpType::Instruction(InstructionType::Syscall5),
        "syscall6" => OpType::Instruction(InstructionType::Syscall6),
        "cast(bool" => OpType::Instruction(InstructionType::CastBool),
        "cast(ptr)" => OpType::Instruction(InstructionType::CastPtr),
        "cast(int)" => OpType::Instruction(InstructionType::CastInt),
        // block
        "if" => OpType::Keyword(KeywordType::If),
        "else" => OpType::Keyword(KeywordType::Else),
        "end" => OpType::Keyword(KeywordType::End),
        "while" => OpType::Keyword(KeywordType::While),
        "do" => OpType::Keyword(KeywordType::Do),
        "macro" => OpType::Keyword(KeywordType::Macro),
        "include" => OpType::Keyword(KeywordType::Include),
        "memory" => OpType::Keyword(KeywordType::Memory),
        _ => OpType::Instruction(InstructionType::None)
    }

}