use crate::constants::OpType;
// use crate::util::logger;
use color_eyre::Result;

fn stack_pop(stack: &mut Vec<i32>) -> Result<i32, &'static str> {
    match stack.pop() {
        Some(i) => Ok(i),
        None => Err("Stack underflow"),
    }
}

pub fn run(tokens: Vec<crate::constants::Operator>) -> Result<(), &'static str>{
    let mut stack: Vec<i32> = Vec::new();
    let mut ti = 0;
    while ti < tokens.len() {
        let token = &tokens[ti];
        match token.typ {
            OpType::Push => {
                stack.push(token.value);
                ti += 1;
            },
            OpType::Pop => {
                stack.pop();
                ti += 1;
            },
            OpType::Plus => {
                let a = stack_pop(&mut stack)?;
                let b = stack_pop(&mut stack)?;
                stack.push(b + a);
                ti += 1;
            },
            OpType::Minus => {
                let a = stack_pop(&mut stack)?;
                let b = stack_pop(&mut stack)?;
                stack.push(b - a);
                ti += 1;
            },
            OpType::Equals => {
                let a = stack_pop(&mut stack)?;
                let b = stack_pop(&mut stack)?;
                stack.push((a == b) as i32);
                ti += 1;
            },
            OpType::Gt => {
                let a = stack_pop(&mut stack)?;
                let b = stack_pop(&mut stack)?;
                stack.push((a > b) as i32);
                ti += 1;
            },
            OpType::Lt => {
                let a = stack_pop(&mut stack)?;
                let b = stack_pop(&mut stack)?;
                stack.push((a < b) as i32);
                ti += 1;
            },
            
            OpType::Print => {
                let a = stack_pop(&mut stack)?;
                println!("{a}");
                // let _ = io::stdout().flush();
                ti += 1;
            },

            OpType::Dup => {
                let a = stack_pop(&mut stack)?;
                stack.push(a);
                stack.push(a);
            },
            OpType::If => {
                let a = stack_pop(&mut stack)?;
                if a == 0 {
                    ti = (token.value + 1) as usize;
                } else {
                    ti += 1;
                }
            },
            OpType::Else => {
                ti = token.value as usize;

            },
            OpType::End => ti += 1
        }
    }
    Ok(())
}