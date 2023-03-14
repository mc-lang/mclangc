use crate::{constants::{Operator, OpType, Token}, util};
use color_eyre::Result;

pub fn cross_ref(mut tokens: Vec<Operator>) -> Vec<Operator> {
    let mut stack: Vec<u32> = Vec::new();
    for ip in 0..tokens.len() {
        let op = &tokens.clone()[ip];
        match op.typ {
            OpType::If => {
                stack.push(ip as u32)
            }
            OpType::Else => {
                let if_ip = stack.pop().unwrap();
                let mut if_og = &mut tokens[if_ip as usize];
                if !vec![OpType::If].contains(&(*if_og).typ)  {
                    util::logger::pos_error(op.clone().pos,"'end' can only close 'if' blocks");
                    std::process::exit(1); // idc
                }

                (*if_og).jmp = (ip + 1) as i32;
                stack.push(ip as u32);
            },
            OpType::End => {
                let block_ip = stack.pop().unwrap();
                let mut block_og = &mut tokens[block_ip as usize].clone();
                if vec![OpType::If, OpType::Else].contains(&(*block_og).typ)  {
                    
                    (*block_og).jmp = ip as i32;
                    tokens[block_ip as usize] = block_og.clone();

                    let do_og = &mut tokens[ip as usize].clone(); 
                    do_og.jmp = (ip + 1) as i32; 
                    
                    tokens[ip as usize] = (*do_og).clone();

                } else if (*block_og).typ == OpType::Do {
                    let do_og = &mut tokens[ip as usize]; 
                    do_og.jmp = block_og.jmp;

                    tokens[ip as usize] = (*do_og).clone();
                    let mut block_og = block_og.clone();
                    block_og.jmp = (ip + 1) as i32;
                    tokens[block_ip as usize] = block_og.clone();
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
                (&mut tokens[ip as usize]).jmp = while_ip as i32;
                stack.push(ip as u32);
            }
            _ => ()
        }

    }
    tokens.clone()
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

    pub fn parse(&mut self) -> Result<Vec<Operator>, ()> {
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
                "drop" => tokens.push(Operator::new(OpType::Pop, 0, token.file.clone(), token.line, token.col)),

                // comp and math
                "+" => tokens.push(Operator::new(OpType::Plus, 0, token.file.clone(), token.line, token.col)),
                "-" => tokens.push(Operator::new(OpType::Minus, 0, token.file.clone(), token.line, token.col)),
                "=" => tokens.push(Operator::new(OpType::Equals, 0, token.file.clone(), token.line, token.col)),
                ">" => tokens.push(Operator::new(OpType::Gt, 0, token.file.clone(), token.line, token.col)),
                "<" => tokens.push(Operator::new(OpType::Lt, 0, token.file.clone(), token.line, token.col)),
                
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

                


                t => {
                    util::logger::pos_error(pos, format!("Unknown token '{}'", t).as_str());
                    return Err(());
                }
            }
        }

        Ok(cross_ref(tokens))
    }
}