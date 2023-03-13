use crate::{constants::{Operator, OpType, Token}, util};
use color_eyre::Result;


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
                    tokens.push(Operator::new(OpType::Push, num));
                },
                
                "pop" => tokens.push(Operator::new(OpType::Pop, 0)),
                "+" => tokens.push(Operator::new(OpType::Plus, 0)),
                "-" => tokens.push(Operator::new(OpType::Minus, 0)),
                "print" => tokens.push(Operator::new(OpType::Print, 0)),
                "=" => tokens.push(Operator::new(OpType::Equals, 0)),


                t => {
                    util::logger::pos_error(pos, format!("Unknown token '{}'", t).as_str());
                    return Err(());
                }
            }
        }

        Ok(tokens)
    }
}