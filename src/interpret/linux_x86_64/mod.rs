use crate::{constants::OpType, lerror, error};
// use crate::util::logger;
use color_eyre::Result;
use eyre::eyre;
mod syscalls;

fn stack_pop(stack: &mut Vec<u64>, pos: &(String, u32, u32)) -> Result<u64> {
    match stack.pop() {
        Some(i) => Ok(i),
        None => {
            lerror!(&pos.clone(), "Stack underflow");
            Err(eyre!("Stack underflow"))
        },
    }
}

pub fn run(tokens: Vec<crate::constants::Operator>) -> Result<i32>{
    let mut stack: Vec<u64> = Vec::new();
    let mut ti = 0;
    let mut mem: Vec<u8> = vec![0; crate::compile::MEM_SZ as usize + crate::compile::STRING_SZ as usize];
    let mut string_idx = 0;
    // for token in &tokens {
    //     println!("{{typ: \"{:?}\", val: {}, jmp: {}}}", token.typ, token.value, token.jmp);

    // }
    while ti < tokens.len() {
        let token = &tokens[ti];
        let pos = token.loc.clone();
        // println!("{:?}", token.typ);
        match token.typ {
            
            // stack 
            OpType::PushInt => {
                stack.push(token.value as u64);
                ti += 1;
            },
            OpType::PushStr => {
                if  token.addr < 0 {
                    stack.push(token.text.len() as u64); // string len
                    stack.push(string_idx + crate::compile::MEM_SZ as u64);
                    
                    for c in token.text.bytes() {
                        mem[crate::compile::MEM_SZ as usize + string_idx as usize] = c;
                        string_idx += 1;
                    }
                } else {
                    stack.push(token.text.len() as u64); 
                    stack.push(token.addr as u64);
                }


                ti += 1;
            },
            OpType::Drop => {
                stack.pop();
                ti += 1;
            },
            OpType::Dup => {
                let a = stack_pop(&mut stack, &pos)?;
                stack.push(a);
                stack.push(a);
                ti += 1;
            },
            OpType::Dup2 => {
                let a = stack_pop(&mut stack, &pos)?;
                let b = stack_pop(&mut stack, &pos)?;
                stack.push(b);
                stack.push(a);
                stack.push(b);
                stack.push(a);
                ti += 1;
            }
            OpType::Rot => {
                let a = stack_pop(&mut stack, &pos)?;
                let b = stack_pop(&mut stack, &pos)?;
                let c = stack_pop(&mut stack, &pos)?;
                stack.push(b);
                stack.push(a);
                stack.push(c);
                ti += 1;
            }
            OpType::Swap => {
                let a = stack_pop(&mut stack, &pos)?;
                let b = stack_pop(&mut stack, &pos)?;
                stack.push(a);
                stack.push(b);
                ti += 1;
            }
            OpType::Over => {
                let a = stack_pop(&mut stack, &pos)?;
                let b = stack_pop(&mut stack, &pos)?;
                stack.push(b);
                stack.push(a);
                stack.push(b);
                ti += 1;
            }

            OpType::Print => {
                let a = stack_pop(&mut stack, &pos)?;
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
                let a = stack_pop(&mut stack, &pos)?;
                let byte = mem[a as usize];
                stack.push(byte as u64);
                ti += 1;
            }
            OpType::Store8 => {
                let val = stack_pop(&mut stack, &pos)?;
                let addr = stack_pop(&mut stack, &pos)?;

                mem[addr as usize] = (val & 0xFF) as u8;
                ti += 1;
            }

            // math
            OpType::Plus => {
                let a = stack_pop(&mut stack, &pos)?;
                let b = stack_pop(&mut stack, &pos)?;
                stack.push(b + a);
                ti += 1;
            },
            OpType::Minus => {
                let a = stack_pop(&mut stack, &pos)?;
                let b = stack_pop(&mut stack, &pos)?;
                stack.push(b - a);
                ti += 1;
            },
            OpType::Equals => {
                let a = stack_pop(&mut stack, &pos)?;
                let b = stack_pop(&mut stack, &pos)?;
                stack.push((b == a) as u64);
                ti += 1;
            },
            OpType::Gt => {
                let a = stack_pop(&mut stack, &pos)?;
                let b = stack_pop(&mut stack, &pos)?;
                stack.push((b > a) as u64);
                ti += 1;
            },
            OpType::Lt => {
                let a = stack_pop(&mut stack, &pos)?;
                let b = stack_pop(&mut stack, &pos)?;
                stack.push((b < a) as u64);
                ti += 1;
            },
            OpType::NotEquals => {
                let a = stack_pop(&mut stack, &pos)?;
                let b = stack_pop(&mut stack, &pos)?;
                stack.push((b != a) as u64);
                ti += 1;
            },
            OpType::Ge => {
                let a = stack_pop(&mut stack, &pos)?;
                let b = stack_pop(&mut stack, &pos)?;
                stack.push((b >= a) as u64);
                ti += 1;
            },
            OpType::Le => {
                let a = stack_pop(&mut stack, &pos)?;
                let b = stack_pop(&mut stack, &pos)?;
                stack.push((b <= a) as u64);
                ti += 1;
            },

            OpType::Band => {
                let a = stack_pop(&mut stack, &pos)?;
                let b = stack_pop(&mut stack, &pos)?;
                stack.push((a & b) as u64);
                ti += 1;
            }

            OpType::Bor => {
                let a = stack_pop(&mut stack, &pos)?;
                let b = stack_pop(&mut stack, &pos)?;
                stack.push((a | b) as u64);
                ti += 1;
            }

            OpType::Shr => {
                let a = stack_pop(&mut stack, &pos)?;
                let b = stack_pop(&mut stack, &pos)?;
                stack.push((b >> a) as u64);
                ti += 1;
            }

            OpType::Shl => {
                let a = stack_pop(&mut stack, &pos)?;
                let b = stack_pop(&mut stack, &pos)?;
                stack.push((b << a) as u64);
                ti += 1;
            }
            
            OpType::DivMod => {
                let a = stack_pop(&mut stack, &pos)?;
                let b = stack_pop(&mut stack, &pos)?;
                stack.push((b / a) as u64);
                stack.push((b % a) as u64);
                ti += 1;
            }
            OpType::Mul => {
                let a = stack_pop(&mut stack, &pos)?;
                let b = stack_pop(&mut stack, &pos)?;
                stack.push((b * a) as u64);
                ti += 1;
            }


            // blocks
            OpType::If => {
                let a = stack_pop(&mut stack, &pos)?;
                if a == 0 {
                    // println!("If({ti}) => t: {:?} j: {}", tokens[token.jmp as usize].typ, token.jmp);
                    ti = (token.jmp) as usize;
                } else {
                    ti += 1;
                }
            },
            OpType::Else => {
                // println!("Else({ti}) => t: {:?} j: {}", tokens[token.jmp as usize].typ, token.jmp);
                ti = (token.jmp) as usize;
            },
            OpType::End => {
                // println!("End({ti}) => t: {:?} j: {}", tokens[token.jmp as usize].typ, token.jmp);
                ti = (token.jmp) as usize;
            }
            OpType::While => {
                ti += 1;
            }
            OpType::Do => {
                let a = stack.pop().unwrap();
                if a == 0 {
                    ti = (token.jmp) as usize;
                } else {
                    ti += 1;
                }
            }
            OpType::Syscall0 => {
                todo!();
                // ti += 1;
            },
            OpType::Syscall1 => {
                todo!();
                // ti += 1;
            },
            OpType::Syscall2 => {
                todo!();
                // ti += 1;
            },
            OpType::Syscall3 => {
                let rax = stack_pop(&mut stack, &pos)?;
                let rdi = stack_pop(&mut stack, &pos)?;
                let rsi = stack_pop(&mut stack, &pos)?;
                let rdx = stack_pop(&mut stack, &pos)?;
                // println!("yes");
                let ret = match rax {
                    1 => syscalls::sys_write(rax, rdi, rsi, rdx, &mem),
                    _ => {
                        error!("Syscall(3) #{} is not implemented", rax);
                        return Err(eyre!("Syscall not implemented"));
                    }
                };
                stack.push(ret);
                // println!("{}", stack.len());
                ti += 1;
            },
            OpType::Syscall4 => {
                todo!();
                // ti += 1;
            },
            OpType::Syscall5 => {
                todo!();
                // ti += 1;
            },
            OpType::Syscall6 => {
                todo!();
                // ti += 1;
            },
            OpType::None => unreachable!(),
            OpType::Macro => unreachable!(),
            OpType::Include => unreachable!()
        }
    }
    

    Ok(0)
}