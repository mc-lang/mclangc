use std::collections::HashMap;

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
    let mut mem: Vec<u64> = vec![0; crate::compile::MEM_SZ + crate::compile::STRING_SZ];
    let mut string_idx = 0;

    let mut memories: HashMap<usize, usize> = HashMap::new();
    // for token in &tokens {
    //     println!("{{typ: \"{:?}\", val: {}, jmp: {}}}", token.typ, token.value, token.jmp);

    // }
    while ti < tokens.len() {
        let token = &tokens[ti];
        let pos = token.loc.clone();
        // println!("{:?}", token.typ);
        match token.typ.clone() {
            OpType::Instruction(instruction) => {
                match instruction {
                    InstructionType::PushInt => {
                        stack.push(token.value);
                        ti += 1;
                    },
                    InstructionType::PushStr => {
                        if  token.addr.is_none() {
                            stack.push(token.text.len()); // string len
                            stack.push(string_idx + crate::compile::MEM_SZ);
                            
                            for c in token.text.bytes() {
                                mem[crate::compile::MEM_SZ + string_idx] = c as u64;
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
                    InstructionType::Drop => {
                        stack.pop();
                        ti += 1;
                    },
                    InstructionType::Dup => {
                        let a = stack_pop(&mut stack, &pos)?;
                        stack.push(a);
                        stack.push(a);
                        ti += 1;
                    },
        
                    InstructionType::Rot => {
                        let a = stack_pop(&mut stack, &pos)?;
                        let b = stack_pop(&mut stack, &pos)?;
                        let c = stack_pop(&mut stack, &pos)?;
                        stack.push(b);
                        stack.push(a);
                        stack.push(c);
                        ti += 1;
                    }
                    InstructionType::Swap => {
                        let a = stack_pop(&mut stack, &pos)?;
                        let b = stack_pop(&mut stack, &pos)?;
                        stack.push(a);
                        stack.push(b);
                        ti += 1;
                    }
                    InstructionType::Over => {
                        let a = stack_pop(&mut stack, &pos)?;
                        let b = stack_pop(&mut stack, &pos)?;
                        stack.push(b);
                        stack.push(a);
                        stack.push(b);
                        ti += 1;
                    }
        
                    InstructionType::Print => {
                        let a = stack_pop(&mut stack, &pos)?;
                        println!("{a}");
                        // let _ = io::stdout().flush();
                        ti += 1;
                    },
                    InstructionType::Load8 |
                    InstructionType::Load32 |
                    InstructionType::Load64 => {
                        let a = stack_pop(&mut stack, &pos)?;
                        if a > crate::compile::MEM_SZ {
                            lerror!(&token.loc, "Invalid memory address {a}");
                            return Ok(1);
                        }
                        let byte = mem[a];
                        stack.push(byte as usize);
                        ti += 1;
                    }
                    #[allow(clippy::cast_possible_truncation)]
                    InstructionType::Store8 => {
                        let val = stack_pop(&mut stack, &pos)?;
                        let addr = stack_pop(&mut stack, &pos)?;
                        
                        if addr > crate::compile::MEM_SZ {
                            lerror!(&token.loc, "Invalid memory address {addr}");
                            return Ok(1);
                        }

                        mem[addr] = val as u8 as u64;
                        ti += 1;
                    }
                    #[allow(clippy::cast_possible_truncation)]
                    InstructionType::Store32 => {
                        let val = stack_pop(&mut stack, &pos)?;
                        let addr = stack_pop(&mut stack, &pos)?;
                        
                        if addr > crate::compile::MEM_SZ {
                            lerror!(&token.loc, "Invalid memory address {addr}");
                            return Ok(1);
                        }

                        mem[addr] = val as u32 as u64;
                        ti += 1;
                    }

                    #[allow(clippy::cast_possible_truncation)]
                    InstructionType::Store64 => {
                        let val = stack_pop(&mut stack, &pos)?;
                        let addr = stack_pop(&mut stack, &pos)?;
                        
                        if addr > crate::compile::MEM_SZ {
                            lerror!(&token.loc, "Invalid memory address {addr}");
                            return Ok(1);
                        }

                        mem[addr] = val as u64;
                        ti += 1;
                    }
        
                    // math
                    InstructionType::Plus => {
                        let a = stack_pop(&mut stack, &pos)?;
                        let b = stack_pop(&mut stack, &pos)?;
                        stack.push(b + a);
                        ti += 1;
                    },
                    InstructionType::Minus => {
                        let a = stack_pop(&mut stack, &pos)?;
                        let b = stack_pop(&mut stack, &pos)?;
                        stack.push(b - a);
                        ti += 1;
                    },
                    InstructionType::Equals => {
                        let a = stack_pop(&mut stack, &pos)?;
                        let b = stack_pop(&mut stack, &pos)?;
                        stack.push(usize::from(b == a));
                        ti += 1;
                    },
                    InstructionType::Gt => {
                        let a = stack_pop(&mut stack, &pos)?;
                        let b = stack_pop(&mut stack, &pos)?;
                        stack.push(usize::from(b > a));
                        ti += 1;
                    },
                    InstructionType::Lt => {
                        let a = stack_pop(&mut stack, &pos)?;
                        let b = stack_pop(&mut stack, &pos)?;
                        stack.push(usize::from(b < a));
                        ti += 1;
                    },
                    InstructionType::NotEquals => {
                        let a = stack_pop(&mut stack, &pos)?;
                        let b = stack_pop(&mut stack, &pos)?;
                        stack.push(usize::from(b != a));
                        ti += 1;
                    },
                    InstructionType::Ge => {
                        let a = stack_pop(&mut stack, &pos)?;
                        let b = stack_pop(&mut stack, &pos)?;
                        stack.push(usize::from(b >= a));
                        ti += 1;
                    },
                    InstructionType::Le => {
                        let a = stack_pop(&mut stack, &pos)?;
                        let b = stack_pop(&mut stack, &pos)?;
                        stack.push(usize::from(b <= a));
                        ti += 1;
                    },
        
                    InstructionType::Band => {
                        let a = stack_pop(&mut stack, &pos)?;
                        let b = stack_pop(&mut stack, &pos)?;
                        stack.push(a & b);
                        ti += 1;
                    }
        
                    InstructionType::Bor => {
                        let a = stack_pop(&mut stack, &pos)?;
                        let b = stack_pop(&mut stack, &pos)?;
                        stack.push(a | b);
                        ti += 1;
                    }
        
                    InstructionType::Shr => {
                        let a = stack_pop(&mut stack, &pos)?;
                        let b = stack_pop(&mut stack, &pos)?;
                        stack.push(b >> a);
                        ti += 1;
                    }
        
                    InstructionType::Shl => {
                        let a = stack_pop(&mut stack, &pos)?;
                        let b = stack_pop(&mut stack, &pos)?;
                        stack.push(b << a);
                        ti += 1;
                    }
                    
                    InstructionType::DivMod => {
                        let a = stack_pop(&mut stack, &pos)?;
                        let b = stack_pop(&mut stack, &pos)?;
                        stack.push(b / a);
                        stack.push(b % a);
                        ti += 1;
                    }
                    InstructionType::Mul => {
                        let a = stack_pop(&mut stack, &pos)?;
                        let b = stack_pop(&mut stack, &pos)?;
                        stack.push(b * a);
                        ti += 1;
                    }
                    InstructionType::Syscall0 => {
                        todo!();
                        // ti += 1;
                    },
                    InstructionType::Syscall1 => {
                        todo!();
                        // ti += 1;
                    },
                    InstructionType::Syscall2 => {
                        todo!();
                        // ti += 1;
                    },
                    InstructionType::Syscall3 => {
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
                    InstructionType::Syscall4 => {
                        todo!();
                        // ti += 1;
                    },
                    InstructionType::Syscall5 => {
                        todo!();
                        // ti += 1;
                    },
                    InstructionType::Syscall6 => {
                        todo!();
                        // ti += 1;
                    },
                    InstructionType::MemUse => {

                        let m = memories.get(&token.addr.unwrap()).unwrap();
                        stack.push(*m);
                        ti += 1;
                    },
                    InstructionType::CastBool |
                    InstructionType::CastPtr |
                    InstructionType::CastInt |
                    InstructionType::FnCall |
                    InstructionType::Return |
                    InstructionType::CastVoid |
                    InstructionType::TypeBool |
                    InstructionType::TypePtr |
                    InstructionType::TypeInt |
                    InstructionType::TypeVoid |
                    InstructionType::TypeStr |
                    InstructionType::TypeAny |
                    InstructionType::Returns |
                    InstructionType::With => ti += 1,
                    InstructionType::None => unreachable!(),
                    InstructionType::ConstUse => todo!(),
                }

            }
            OpType::Keyword(k) => {
                match k {
                    // blocks
                    KeywordType::If => {
                        let a = stack_pop(&mut stack, &pos)?;
                        if a == 0 {
                            // println!("If({ti}) => t: {:?} j: {}", tokens[token.jmp as usize].typ, token.jmp);
                            ti = token.jmp;
                        } else {
                            ti += 1;
                        }
                    },
                    KeywordType::Else | KeywordType::End => {
                        ti = token.jmp;
                    }
                    KeywordType::While => {
                        ti += 1;
                    }
                    KeywordType::Do => {
                        let a = stack.pop().unwrap();
                        if a == 0 {
                            ti = token.jmp;
                        } else {
                            ti += 1;
                        }
                    }
                    KeywordType::Memory => {
                        memories.insert(token.addr.unwrap(), token.value);
                        ti += 1;
                    },
                    KeywordType::Include => unreachable!(),
                    KeywordType::Constant => todo!(),
                    KeywordType::Function => todo!(),
                    KeywordType::FunctionDo => todo!(),
                }
            }
            
        }
    }
    

    Ok(0)
}