use crate::{constants::{Operator, OpType}, util};
use color_eyre::{Result, Report};


pub struct Parser {
    file: String
}

impl Parser {
    pub fn new(file: String) -> Self {
        Self{
            file
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Operator>, &'static str> {
        let mut tokens = Vec::new();

        for tok in self.file.split([' ', '\n', '\t', '\r']) {
            if tok == "" {
                continue;
            }

            match tok {
                t if t.parse::<i32>().is_ok() => {
                    let num = t.parse::<i32>().unwrap();
                    tokens.push(Operator::new(OpType::Push, num));
                },

                "pop" => tokens.push(Operator::new(OpType::Pop, 0)),
                "+" => tokens.push(Operator::new(OpType::Plus, 0)),
                "-" => tokens.push(Operator::new(OpType::Minus, 0)),
                "print" => tokens.push(Operator::new(OpType::Print, 0)),


                t => {
                    util::logger::error("Unknown token '{t}'");
                    return Err("");
                }
            }
        }

        dbg!(&tokens);
        Ok(tokens)
    }
}