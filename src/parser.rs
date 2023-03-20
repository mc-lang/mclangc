use std::{collections::HashMap, ops::Deref};

use crate::{constants::{Operator, OpType, Token, TokenType}, lerror};
use color_eyre::Result;
use eyre::eyre;

pub fn cross_ref(mut program: Vec<Operator>) -> Result<Vec<Operator>> {
    let mut stack: Vec<u32> = Vec::new();
    for ip in 0..program.len() {
        let op = &program.clone()[ip];
        match op.typ {
            OpType::If => {
                stack.push(ip as u32)
            }
            OpType::Else => {
                let if_ip = stack.pop().unwrap();
                if program[if_ip as usize].typ != OpType::If {
                    lerror!(&op.clone().pos,"'end' can only close 'if' blocks");
                    return Err(eyre!("Bad block"));
                }
                
                // let mut if_og = &mut tokens[if_ip as usize];
                // (*if_og).jmp = (ip + 1) as i32;
                program[if_ip as usize].jmp = (ip + 1) as i32;
                stack.push(ip as u32);
            },
            OpType::End => {
                let block_ip = stack.pop().unwrap();
                // let mut block_og = &mut tokens[block_ip as usize].clone();
                if program[block_ip as usize].typ == OpType::If || 
                   program[block_ip as usize].typ == OpType::Else {
                    
                    program[block_ip as usize].jmp = ip as i32;
                    program[ip as usize].jmp = (ip + 1)as i32;

                } else if program[block_ip as usize].typ == OpType::Do {
                    program[ip].jmp = program[block_ip as usize].jmp;
                    program[block_ip as usize].jmp = (ip + 1) as i32;
                } else {
                    lerror!(&op.clone().pos,"'end' can only close 'if' blocks");
                    return  Err(eyre!(""));
                }

            }
            OpType::While => {
                stack.push(ip as u32);
            }
            OpType::Do => {
                let while_ip = stack.pop().unwrap();
                program[ip as usize].jmp = while_ip as i32;
                stack.push(ip as u32);
            }
            _ => ()
        }

    }
    if stack.len() > 0 {
        lerror!(&program[stack.pop().expect("Empy stack") as usize].clone().pos,"Unclosed block");
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
                    let word_type = lookup_word(token.text.clone(), &pos)?;
                    tokens.push(Operator::new(word_type, 0, token.text.clone(), token.file.clone(), token.line, token.col));
                },
                TokenType::Int => {// negative numbers not yet implemented
                    tokens.push(Operator::new(OpType::PushInt, token.text.parse::<i64>()?, String::new(), token.file.clone(), token.line, token.col));
                },
                TokenType::String => {
                    tokens.push(Operator::new(OpType::PushStr, 0, token.text.clone(), token.file.clone(), token.line, token.col));
                }
            };

            
            //"print" => tokens.push(Operator::new(OpType::Print, 0, token.file.clone(), token.line, token.col)),
        }

        Ok(cross_ref(tokens)?)
    }
}


pub fn lookup_word<P: Deref<Target = (String, u32, u32)>>(s: String, _pos: P) -> Result<OpType>{
    let lookup_table: HashMap<&str, OpType> = HashMap::from([
        //stack
        ("print", OpType::Print),
        ("dup", OpType::Dup),
        ("drop", OpType::Drop),
        ("2dup", OpType::Dup2),
        ("rot", OpType::Rot),
        ("over", OpType::Over),
        ("swap", OpType::Swap),

        // comp and math
        ("+", OpType::Plus),
        ("-", OpType::Minus),
        ("=", OpType::Equals),
        (">", OpType::Gt),
        ("<", OpType::Lt),
        ("band", OpType::Band),
        ("bor", OpType::Bor),
        ("shr", OpType::Shr),
        ("shl", OpType::Shl),
        ("/", OpType::Div),
        ("*", OpType::Mul),
        
        // block
        ("if", OpType::If),
        ("else", OpType::Else),
        ("end", OpType::End),
        ("while", OpType::While),
        ("do", OpType::Do),
        ("macro", OpType::Macro),

        // mem
        ("mem", OpType::Mem),
        ("!8", OpType::Load8),
        ("@8", OpType::Store8),

        ("syscall0", OpType::Syscall0),
        ("syscall1", OpType::Syscall1),
        ("syscall2", OpType::Syscall2),
        ("syscall3", OpType::Syscall3),
        ("syscall4", OpType::Syscall4),
        ("syscall5", OpType::Syscall5),
        ("syscall6", OpType::Syscall6),
    ]);

    match lookup_table.get(s.as_str()) {
        Some(v) => Ok(v.clone()),
        None => {
            Ok(OpType::None)
        }
    }
}