use std::fmt::format;

use crate::{constants::OpType, util::logger};
// use crate::util::logger;
use color_eyre::Result;

mod syscalls;

fn stack_pop(stack: &mut Vec<u64>) -> Result<u64, &'static str> {
    match stack.pop() {
        Some(i) => Ok(i),
        None => Err("Stack underflow"),
    }
}

pub fn run(tokens: Vec<crate::constants::Operator>) -> Result<(), &'static str>{
    let mut stack: Vec<u64> = Vec::new();
    let mut ti = 0;
    let mut mem: Vec<u8> = vec![0; crate::compile::MEM_SZ as usize];

    while ti < tokens.len() {
        let token = &tokens[ti];
        
        match token.typ {
            // stack 
            OpType::Push => {
                stack.push(token.value as u64);
                ti += 1;
            },
            OpType::Pop => {
                stack.pop();
                ti += 1;
            },
            OpType::Dup => {
                let a = stack_pop(&mut stack)?;
                stack.push(a);
                stack.push(a);
                ti += 1;
            },

            OpType::Print => {
                let a = stack_pop(&mut stack)?;
                println!("{a}");
                // let _ = io::stdout().flush();
                ti += 1;
            },
            // mem

            OpType::Mem => {
                stack.push(0);
                ti += 1;
            }
            OpType::Load8 => {
                let a = stack_pop(&mut stack)?;

                stack.push(mem[a as usize] as u64);
                ti += 1;
            }
            OpType::Store8 => {
                let a = stack_pop(&mut stack)?;
                let b = stack_pop(&mut stack)?;

                mem[b as usize] = a as u8;
                ti += 1;
            }

            // math
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
                stack.push((a == b) as u64);
                ti += 1;
            },
            OpType::Gt => {
                let b = stack_pop(&mut stack)?;
                let a = stack_pop(&mut stack)?;
                stack.push((a > b) as u64);
                ti += 1;
            },
            OpType::Lt => {
                let b = stack_pop(&mut stack)?;
                let a = stack_pop(&mut stack)?;
                stack.push((a < b) as u64);
                ti += 1;
            },
            
            // blocks
            OpType::If => {
                let a = stack_pop(&mut stack)?;
                if a == 0 {
                    ti = (token.jmp) as usize;
                } else {
                    ti += 1;
                }
            },
            OpType::Else => {
                ti = token.jmp as usize;
            },

            OpType::While => {
                ti += 1;
            }
            OpType::Do => {
                let a = stack.pop().unwrap();
                if a == 0 {
                    ti = token.jmp as usize;
                } else {
                    ti += 1;
                }
            }            
            OpType::End => {
                ti = token.jmp as usize;
            }
            OpType::Syscall0 => {
                todo!();
                ti += 1;
            },
            OpType::Syscall1 => {
                todo!();
                ti += 1;
            },
            OpType::Syscall2 => {
                todo!();
                ti += 1;
            },
            OpType::Syscall3 => {
                let rax = stack_pop(&mut stack)?;
                let rdi = stack_pop(&mut stack)?;
                let rsi = stack_pop(&mut stack)?;
                let rdx = stack_pop(&mut stack)?;

                let ret = match rax {
                    1 => syscalls::sys_write(rax, rdi, rsi, rdx, &mem),
                    _ => {
                        logger::error(format!("Syscall(3) #{} is not implemented", rax).as_str());
                        return Err("Exiting");
                    }
                };
                stack.push(ret);
                ti += 1;
            },
            OpType::Syscall4 => {
                todo!();
                ti += 1;
            },
            OpType::Syscall5 => {
                todo!();
                ti += 1;
            },
            OpType::Syscall6 => {
                todo!();
                ti += 1;
            },
        }
    }
    Ok(())
}