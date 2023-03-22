use crate::{constants::{OpType, Loc, InstructionType, KeywordType}, lerror, error};
// use crate::util::logger;
use color_eyre::Result;
use eyre::eyre;
mod syscalls;

fn stack_pop(stack: &mut Vec<usize>, pos: &Loc) -> Result<usize> {
    if let Some(i) = stack.pop() { Ok(i) } else {
        lerror!(&pos.clone(), "Stack underflow");
        Err(eyre!("Stack underflow"))
    }
}

pub fn run(tokens: &[crate::constants::Operator]) -> Result<i32>{
    let mut stack: Vec<usize> = Vec::new();
    let mut ti = 0;
    let mut mem: Vec<u8> = vec![0; crate::compile::MEM_SZ + crate::compile::STRING_SZ];
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
            OpType::Instruction(InstructionType::PushInt) => {
                stack.push(token.value);
                ti += 1;
            },
            OpType::Instruction(InstructionType::PushStr) => {
                if  token.addr.is_none() {
                    stack.push(token.text.len()); // string len
                    stack.push(string_idx + crate::compile::MEM_SZ);
                    
                    for c in token.text.bytes() {
                        mem[crate::compile::MEM_SZ + string_idx] = c;
                        string_idx += 1;
                    }
                } else {
                    stack.push(token.text.len()); 
                    if let Some(addr) = token.addr {
                        stack.push(addr);
                    }
                }


                ti += 1;
            },
            OpType::Instruction(InstructionType::Drop) => {
                stack.pop();
                ti += 1;
            },
            OpType::Instruction(InstructionType::Dup) => {
                let a = stack_pop(&mut stack, &pos)?;
                stack.push(a);
                stack.push(a);
                ti += 1;
            },

            OpType::Instruction(InstructionType::Rot) => {
                let a = stack_pop(&mut stack, &pos)?;
                let b = stack_pop(&mut stack, &pos)?;
                let c = stack_pop(&mut stack, &pos)?;
                stack.push(b);
                stack.push(a);
                stack.push(c);
                ti += 1;
            }
            OpType::Instruction(InstructionType::Swap) => {
                let a = stack_pop(&mut stack, &pos)?;
                let b = stack_pop(&mut stack, &pos)?;
                stack.push(a);
                stack.push(b);
                ti += 1;
            }
            OpType::Instruction(InstructionType::Over) => {
                let a = stack_pop(&mut stack, &pos)?;
                let b = stack_pop(&mut stack, &pos)?;
                stack.push(b);
                stack.push(a);
                stack.push(b);
                ti += 1;
            }

            OpType::Instruction(InstructionType::Print) => {
                let a = stack_pop(&mut stack, &pos)?;
                println!("{a}");
                // let _ = io::stdout().flush();
                ti += 1;
            },
            // mem

            OpType::Instruction(InstructionType::Mem) => {
                stack.push(0);
                ti += 1;
            }
            OpType::Instruction(InstructionType::Load8) => {
                let a = stack_pop(&mut stack, &pos)?;
                let byte = mem[a];
                stack.push(byte as usize);
                ti += 1;
            }
            #[allow(clippy::cast_possible_truncation)]
            OpType::Instruction(InstructionType::Store8) => {
                let val = stack_pop(&mut stack, &pos)?;
                let addr = stack_pop(&mut stack, &pos)?;

                mem[addr] = (val & 0xFF) as u8;
                ti += 1;
            }

            // math
            OpType::Instruction(InstructionType::Plus) => {
                let a = stack_pop(&mut stack, &pos)?;
                let b = stack_pop(&mut stack, &pos)?;
                stack.push(b + a);
                ti += 1;
            },
            OpType::Instruction(InstructionType::Minus) => {
                let a = stack_pop(&mut stack, &pos)?;
                let b = stack_pop(&mut stack, &pos)?;
                stack.push(b - a);
                ti += 1;
            },
            OpType::Instruction(InstructionType::Equals) => {
                let a = stack_pop(&mut stack, &pos)?;
                let b = stack_pop(&mut stack, &pos)?;
                stack.push(usize::from(b == a));
                ti += 1;
            },
            OpType::Instruction(InstructionType::Gt) => {
                let a = stack_pop(&mut stack, &pos)?;
                let b = stack_pop(&mut stack, &pos)?;
                stack.push(usize::from(b > a));
                ti += 1;
            },
            OpType::Instruction(InstructionType::Lt) => {
                let a = stack_pop(&mut stack, &pos)?;
                let b = stack_pop(&mut stack, &pos)?;
                stack.push(usize::from(b < a));
                ti += 1;
            },
            OpType::Instruction(InstructionType::NotEquals) => {
                let a = stack_pop(&mut stack, &pos)?;
                let b = stack_pop(&mut stack, &pos)?;
                stack.push(usize::from(b != a));
                ti += 1;
            },
            OpType::Instruction(InstructionType::Ge) => {
                let a = stack_pop(&mut stack, &pos)?;
                let b = stack_pop(&mut stack, &pos)?;
                stack.push(usize::from(b >= a));
                ti += 1;
            },
            OpType::Instruction(InstructionType::Le) => {
                let a = stack_pop(&mut stack, &pos)?;
                let b = stack_pop(&mut stack, &pos)?;
                stack.push(usize::from(b <= a));
                ti += 1;
            },

            OpType::Instruction(InstructionType::Band) => {
                let a = stack_pop(&mut stack, &pos)?;
                let b = stack_pop(&mut stack, &pos)?;
                stack.push(a & b);
                ti += 1;
            }

            OpType::Instruction(InstructionType::Bor) => {
                let a = stack_pop(&mut stack, &pos)?;
                let b = stack_pop(&mut stack, &pos)?;
                stack.push(a | b);
                ti += 1;
            }

            OpType::Instruction(InstructionType::Shr) => {
                let a = stack_pop(&mut stack, &pos)?;
                let b = stack_pop(&mut stack, &pos)?;
                stack.push(b >> a);
                ti += 1;
            }

            OpType::Instruction(InstructionType::Shl) => {
                let a = stack_pop(&mut stack, &pos)?;
                let b = stack_pop(&mut stack, &pos)?;
                stack.push(b << a);
                ti += 1;
            }
            
            OpType::Instruction(InstructionType::DivMod) => {
                let a = stack_pop(&mut stack, &pos)?;
                let b = stack_pop(&mut stack, &pos)?;
                stack.push(b / a);
                stack.push(b % a);
                ti += 1;
            }
            OpType::Instruction(InstructionType::Mul) => {
                let a = stack_pop(&mut stack, &pos)?;
                let b = stack_pop(&mut stack, &pos)?;
                stack.push(b * a);
                ti += 1;
            }


            // blocks
            OpType::Keyword(KeywordType::If) => {
                let a = stack_pop(&mut stack, &pos)?;
                if a == 0 {
                    // println!("If({ti}) => t: {:?} j: {}", tokens[token.jmp as usize].typ, token.jmp);
                    ti = token.jmp;
                } else {
                    ti += 1;
                }
            },
            OpType::Keyword(KeywordType::Else) | OpType::Keyword(KeywordType::End) => {
                ti = token.jmp;
            }
            OpType::Keyword(KeywordType::While) => {
                ti += 1;
            }
            OpType::Keyword(KeywordType::Do) => {
                let a = stack.pop().unwrap();
                if a == 0 {
                    ti = token.jmp;
                } else {
                    ti += 1;
                }
            }
            OpType::Instruction(InstructionType::Syscall0) => {
                todo!();
                // ti += 1;
            },
            OpType::Instruction(InstructionType::Syscall1) => {
                todo!();
                // ti += 1;
            },
            OpType::Instruction(InstructionType::Syscall2) => {
                todo!();
                // ti += 1;
            },
            OpType::Instruction(InstructionType::Syscall3) => {
                let rax = stack_pop(&mut stack, &pos)?;
                let rdi = stack_pop(&mut stack, &pos)?;
                let rsi = stack_pop(&mut stack, &pos)?;
                let rdx = stack_pop(&mut stack, &pos)?;
                // println!("yes");
                let ret = match rax {
                    1 => syscalls::sys_write(rax, rdi, rsi, rdx, &mem),
                    0 => 0, //? temp, so clippy doesnt complain
                    _ => {
                        error!("Syscall(3) #{} is not implemented", rax);
                        return Err(eyre!("Syscall not implemented"));
                    }
                };
                stack.push(ret);
                // println!("{}", stack.len());
                ti += 1;
            },
            OpType::Instruction(InstructionType::Syscall4) => {
                todo!();
                // ti += 1;
            },
            OpType::Instruction(InstructionType::Syscall5) => {
                todo!();
                // ti += 1;
            },
            OpType::Instruction(InstructionType::Syscall6) => {
                todo!();
                // ti += 1;
            },
            OpType::Instruction(InstructionType::None) | OpType::Keyword(KeywordType::Macro) | OpType::Keyword(KeywordType::Include) => unreachable!()
        }
    }
    

    Ok(0)
}