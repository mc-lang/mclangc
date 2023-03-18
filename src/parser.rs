use crate::{constants::{Operator, OpType, Token}, util};
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
                    util::logger::pos_error(op.clone().pos,"'end' can only close 'if' blocks");
                    std::process::exit(1); // idc
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
                    util::logger::pos_error(op.clone().pos,"'end' can only close 'if' blocks");
                    std::process::exit(1); // idc
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
        util::logger::pos_error(program[stack.pop().expect("Empy stack") as usize].clone().pos,"Unclosed block");
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
            match token.text.as_str() {
                t if t.parse::<u64>().is_ok() => { // negative numbers not yet implemented
                    let num = t.parse::<i64>().unwrap();
                    tokens.push(Operator::new(OpType::Push, num, token.file.clone(), token.line, token.col));
                },
                
                "print" => tokens.push(Operator::new(OpType::Print, 0, token.file.clone(), token.line, token.col)),
                
                // stack
                "dup" => tokens.push(Operator::new(OpType::Dup, 0, token.file.clone(), token.line, token.col)),
                "drop" => tokens.push(Operator::new(OpType::Drop, 0, token.file.clone(), token.line, token.col)),
                "2dup" => tokens.push(Operator::new(OpType::Dup2, 0, token.file.clone(), token.line, token.col)),
                "rot" => tokens.push(Operator::new(OpType::Rot, 0, token.file.clone(), token.line, token.col)),
                "over" => tokens.push(Operator::new(OpType::Over, 0, token.file.clone(), token.line, token.col)),
                "swap" => tokens.push(Operator::new(OpType::Swap, 0, token.file.clone(), token.line, token.col)),

                // comp and math
                "+" => tokens.push(Operator::new(OpType::Plus, 0, token.file.clone(), token.line, token.col)),
                "-" => tokens.push(Operator::new(OpType::Minus, 0, token.file.clone(), token.line, token.col)),
                "=" => tokens.push(Operator::new(OpType::Equals, 0, token.file.clone(), token.line, token.col)),
                ">" => tokens.push(Operator::new(OpType::Gt, 0, token.file.clone(), token.line, token.col)),
                "<" => tokens.push(Operator::new(OpType::Lt, 0, token.file.clone(), token.line, token.col)),
                "band" => tokens.push(Operator::new(OpType::Band, 0, token.file.clone(), token.line, token.col)),
                "bor" => tokens.push(Operator::new(OpType::Bor, 0, token.file.clone(), token.line, token.col)),
                "shr" => tokens.push(Operator::new(OpType::Shr, 0, token.file.clone(), token.line, token.col)),
                "shl" => tokens.push(Operator::new(OpType::Shl, 0, token.file.clone(), token.line, token.col)),
                "/" => tokens.push(Operator::new(OpType::Div, 0, token.file.clone(), token.line, token.col)),
                "*" => tokens.push(Operator::new(OpType::Mul, 0, token.file.clone(), token.line, token.col)),
                
                // block
                "if" =>    tokens.push(Operator::new(OpType::If, 0, token.file.clone(), token.line, token.col)),
                "else" =>  tokens.push(Operator::new(OpType::Else, 0, token.file.clone(), token.line, token.col)),
                "end" =>   tokens.push(Operator::new(OpType::End, 0, token.file.clone(), token.line, token.col)),
                "while" => tokens.push(Operator::new(OpType::While, 0, token.file.clone(), token.line, token.col)),
                "do" =>    tokens.push(Operator::new(OpType::Do, 0, token.file.clone(), token.line, token.col)),

                // mem
                "mem" =>    tokens.push(Operator::new(OpType::Mem, 0, token.file.clone(), token.line, token.col)),
                "!8" =>    tokens.push(Operator::new(OpType::Load8, 0, token.file.clone(), token.line, token.col)),
                "@8" =>    tokens.push(Operator::new(OpType::Store8, 0, token.file.clone(), token.line, token.col)),

                "syscall0" =>    tokens.push(Operator::new(OpType::Syscall0, 0, token.file.clone(), token.line, token.col)),
                "syscall1" =>    tokens.push(Operator::new(OpType::Syscall1, 0, token.file.clone(), token.line, token.col)),
                "syscall2" =>    tokens.push(Operator::new(OpType::Syscall2, 0, token.file.clone(), token.line, token.col)),
                "syscall3" =>    tokens.push(Operator::new(OpType::Syscall3, 0, token.file.clone(), token.line, token.col)),
                "syscall4" =>    tokens.push(Operator::new(OpType::Syscall4, 0, token.file.clone(), token.line, token.col)),
                "syscall5" =>    tokens.push(Operator::new(OpType::Syscall5, 0, token.file.clone(), token.line, token.col)),
                "syscall6" =>    tokens.push(Operator::new(OpType::Syscall6, 0, token.file.clone(), token.line, token.col)),

                


                t => {
                    util::logger::pos_error(pos, format!("Unknown token '{}'", t).as_str());
                    return Err(eyre!("Unknown token"));
                }
            }
        }

        Ok(cross_ref(tokens)?)
    }
}