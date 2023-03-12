use crate::constants::OpType;
use crate::interpret::logger;
use std::io::{self, Write};
use color_eyre::Result;

fn stack_pop(mut stack: &mut Vec<i32>) -> Result<i32, &'static str> {
    match stack.pop() {
        Some(i) => Ok(i),
        None => Err("Stack underflow"),
    }
}

pub fn run(tokens: Vec<crate::constants::Operator>) -> Result<(), &'static str>{
    let mut stack: Vec<i32> = Vec::new();

    for token in tokens {
        match token.typ {
            OpType::Push => {
                stack.push(token.value);
            },
            OpType::Pop => {
                stack.pop();
            },
            OpType::Plus => {
                let a = stack_pop(&mut stack)?;
                let b = stack_pop(&mut stack)?;
                stack.push(b + a);
            },
            OpType::Minus => {
                let a = stack_pop(&mut stack)?;
                let b = stack_pop(&mut stack)?;
                stack.push(b - a);
            },
            OpType::Print => {
                let a = stack_pop(&mut stack)?;
                println!("{a}");
                // let _ = io::stdout().flush();
            },
        }
    }
    Ok(())
}