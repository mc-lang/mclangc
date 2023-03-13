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

            OpType::End => {
                let block_ip = stack.pop().unwrap();
                let mut block_og = &mut tokens[block_ip as usize];
                if vec![OpType::If, OpType::Else].contains(&(*block_og).typ)  {
                    (*block_og).value = ip as i32;
                    tokens[block_ip as usize] = block_og.clone();
                } else {
                    util::logger::pos_error(op.clone().pos,"'end' can only close 'if' blocks");
                    std::process::exit(1); // idc
                }

            }
            OpType::Else => {
                let if_ip = stack.pop().unwrap();
                let mut if_og = &mut tokens[if_ip as usize];
                if !vec![OpType::If].contains(&(*if_og).typ)  {
                    util::logger::pos_error(op.clone().pos,"'end' can only close 'if' blocks");
                    std::process::exit(1); // idc
                }

                (*if_og).value = ip as i32;
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
            let pos = (token.file.clone(), token.line, token.col);
            match token.text.as_str() {
                t if t.parse::<i32>().is_ok() => {
                    let num = t.parse::<i32>().unwrap();
                    tokens.push(Operator::new(OpType::Push, num, token.file.clone(), token.line, token.col));
                },
                
                "pop" => tokens.push(Operator::new(OpType::Pop, 0, token.file.clone(), token.line, token.col)),
                "+" => tokens.push(Operator::new(OpType::Plus, 0, token.file.clone(), token.line, token.col)),
                "-" => tokens.push(Operator::new(OpType::Minus, 0, token.file.clone(), token.line, token.col)),
                "print" => tokens.push(Operator::new(OpType::Print, 0, token.file.clone(), token.line, token.col)),
                "=" => tokens.push(Operator::new(OpType::Equals, 0, token.file.clone(), token.line, token.col)),
                "if" => tokens.push(Operator::new(OpType::If, 0, token.file.clone(), token.line, token.col)),
                "else" => tokens.push(Operator::new(OpType::Else, 0, token.file.clone(), token.line, token.col)),
                "end" => tokens.push(Operator::new(OpType::End, 0, token.file.clone(), token.line, token.col)),
                "dup" => tokens.push(Operator::new(OpType::Dup, 0, token.file.clone(), token.line, token.col)),


                t => {
                    util::logger::pos_error(pos, format!("Unknown token '{}'", t).as_str());
                    return Err(());
                }
            }
        }

        Ok(cross_ref(tokens))
    }
}