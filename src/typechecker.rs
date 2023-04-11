use std::collections::HashMap;

use crate::{constants::{Operator, Types, OpType, KeywordType, InstructionType}, Args, lerror, warn, note};
use color_eyre::Result;
use eyre::eyre;

#[derive(Debug, Clone)]
struct Function {
    args: Vec<Types>,
    returns: Vec<Types>,
}
impl Function {
    pub fn default() -> Self {
        Self {
            args: Vec::new(),
            returns: Vec::new(),
        }
    }
}

pub fn typecheck(ops: Vec<Operator>, args: &Args) -> Result<Vec<Operator>>{
    if args.unsaf {
        if !args.quiet {
            warn!("Unsafe mode enabled, disabling typechecker, goodluck");
        }
        return Ok(ops.to_vec());
    }
    
    let mut functions: HashMap<String, Function> = HashMap::new();
    // let mut in_function: (String, Function) = (String::new(), Function::default());
    let mut stack: Vec<Types> = Vec::new();
    let mut stack_snapshots: Vec<Vec<Types>> = Vec::new();
    let mut rtokens = ops.clone();
    rtokens.reverse();
    // println!("{:#?}", ops);
    while !rtokens.is_empty() {
        let op = rtokens.pop().unwrap();
        println!("{:?}", stack.clone());
        // println!("{:?}", op);
        // println!("{}", ops.len());
        match op.typ.clone() {
            OpType::Keyword(keyword) => {
                match keyword {
                    KeywordType::If => {
                        stack_pop(&mut stack, &op, &[Types::Bool])?;
                    },
                    KeywordType::Do => {
                        stack_pop(&mut stack, &op, &[Types::Bool])?;
                    },

                    KeywordType::Function => {
                        let name = op.text.clone();

                        if let Some(p) = rtokens.pop() {
                            if p.typ != OpType::Instruction(InstructionType::With){
                                lerror!(&op.loc, "Expected {:?}, got {:?}", OpType::Instruction(InstructionType::With), p.typ);
                                return Err(eyre!(""));
                            }

                        } else {
                            lerror!(&op.loc, "Expected {:?}, got nothing", OpType::Instruction(InstructionType::With));
                            return Err(eyre!(""));
                        }
                        
                        let mut p = rtokens.pop();
                        let mut func = Function {
                            args: Vec::new(),
                            returns: Vec::new(),
                        };
                        let mut return_args = false;
                        while p.as_ref().is_some() {
                            let op = p.as_ref().unwrap();
                            if op.typ == OpType::Instruction(InstructionType::TypeBool) ||
                                op.typ == OpType::Instruction(InstructionType::TypeInt) ||
                                op.typ == OpType::Instruction(InstructionType::TypePtr) ||
                                op.typ == OpType::Instruction(InstructionType::TypeVoid) {
                                    let t = if op.typ == OpType::Instruction(InstructionType::TypeInt) {
                                        Types::Int
                                    } else 
                                    if op.typ == OpType::Instruction(InstructionType::TypeBool) {
                                        Types::Bool
                                    } else
                                    if op.typ == OpType::Instruction(InstructionType::TypePtr) {
                                        Types::Ptr
                                    } else 
                                    if op.typ == OpType::Instruction(InstructionType::TypeVoid) {
                                        if return_args {
                                            func.returns = vec![Types::Void];
                                        } else {
                                            func.args = vec![Types::Void];
                                            return_args = true;
                                            continue;
                                        }
                                        Types::Void
                                    } else
                                    if op.typ == OpType::Instruction(InstructionType::TypeStr) {
                                        Types::Str
                                    } else
                                    if op.typ == OpType::Instruction(InstructionType::TypeAny) {
                                        Types::Any
                                    } else {
                                        panic!()
                                    };

                                    if return_args {
                                        func.returns.push(t);
                                    } else {
                                        func.args.push(t);
                                    }
                            }
                                
                            if op.typ == OpType::Instruction(InstructionType::Returns) {
                                return_args = true;
                            }

                            if op.typ == OpType::Keyword(KeywordType::FunctionDo) {
                                break;
                            }
                            p = rtokens.pop();
                        };
                        functions.insert(name.clone(), func.clone());

                        // if name == "main" {
                        //     in_function = (name, func.clone());
                        // }
                        if func.args != vec![Types::Void] {
                            stack.append(&mut func.args);
                        }
                        stack_snapshots.push(stack.clone());
                    }

                    KeywordType::Else |
                    KeywordType::End |
                    KeywordType::While |
                    KeywordType::Include |
                    KeywordType::Constant |
                    KeywordType::Memory => (),
                    KeywordType::FunctionDo => (),
                }
            },
            OpType::Instruction(instruction) => {
                match instruction {
                    InstructionType::PushInt => {
                        stack.push(Types::Int);
                    },
                    InstructionType::PushStr => {
                        stack.push(Types::Int);
                        stack.push(Types::Ptr);

                    },
                    InstructionType::Drop => {
                        stack_pop(&mut stack, &op, &[Types::Any])?;
                    },
                    InstructionType::Print => {
                        stack_pop(&mut stack, &op, &[Types::Int])?;
                    },
                    InstructionType::Dup => {
                        let a = stack_pop(&mut stack, &op, &[Types::Any])?;
                        stack.push(a);
                    },
                    InstructionType::Rot => {
                        let a = stack_pop(&mut stack, &op, &[Types::Any])?;
                        let b = stack_pop(&mut stack, &op, &[Types::Any])?;
                        let c = stack_pop(&mut stack, &op, &[Types::Any])?;
                        stack.push(b);
                        stack.push(a);
                        stack.push(c);
                    },
                    InstructionType::Over => {
                        let a = stack_pop(&mut stack, &op, &[Types::Any])?;
                        let b = stack_pop(&mut stack, &op, &[Types::Any])?;
                        stack.push(b.clone());
                        stack.push(a);
                        stack.push(b);
                    },
                    InstructionType::Swap => {
                        let a = stack_pop(&mut stack, &op, &[Types::Any])?;
                        let b = stack_pop(&mut stack, &op, &[Types::Any])?;
                        stack.push(a);
                        stack.push(b);
                    },
                    InstructionType::Minus => {
                        stack_pop(&mut stack, &op, &[Types::Int])?;
                        stack_pop(&mut stack, &op, &[Types::Int])?;
                        stack.push(Types::Int);
                    },
                    InstructionType::Plus => {
                        stack_pop(&mut stack, &op, &[Types::Int])?;
                        stack_pop(&mut stack, &op, &[Types::Int])?;
                        stack.push(Types::Int);
                    },
                    InstructionType::Equals => {
                        stack_pop(&mut stack, &op, &[Types::Int])?;
                        stack_pop(&mut stack, &op, &[Types::Int])?;
                        stack.push(Types::Bool);
                    },
                    InstructionType::Gt => {
                        stack_pop(&mut stack, &op, &[Types::Int])?;
                        stack_pop(&mut stack, &op, &[Types::Int])?;
                        stack.push(Types::Bool);
                    },
                    InstructionType::Lt => {
                        stack_pop(&mut stack, &op, &[Types::Int])?;
                        stack_pop(&mut stack, &op, &[Types::Int])?;
                        stack.push(Types::Bool);
                    },
                    InstructionType::Ge => {
                        stack_pop(&mut stack, &op, &[Types::Int])?;
                        stack_pop(&mut stack, &op, &[Types::Int])?;
                        stack.push(Types::Bool);
                    },
                    InstructionType::Le => {
                        stack_pop(&mut stack, &op, &[Types::Int])?;
                        stack_pop(&mut stack, &op, &[Types::Int])?;
                        stack.push(Types::Bool);
                    },
                    InstructionType::NotEquals => {
                        stack_pop(&mut stack, &op, &[Types::Int])?;
                        stack_pop(&mut stack, &op, &[Types::Int])?;
                        stack.push(Types::Bool);
                    },
                    InstructionType::Band => {
                        stack_pop(&mut stack, &op, &[Types::Int])?;
                        stack_pop(&mut stack, &op, &[Types::Int])?;
                        stack.push(Types::Int);
                    },
                    InstructionType::Bor => {
                        stack_pop(&mut stack, &op, &[Types::Int])?;
                        stack_pop(&mut stack, &op, &[Types::Int])?;
                        stack.push(Types::Int);
                    },
                    InstructionType::Shr => {
                        stack_pop(&mut stack, &op, &[Types::Int])?;
                        stack_pop(&mut stack, &op, &[Types::Int])?;
                        stack.push(Types::Int);
                    },
                    InstructionType::Shl => {
                        stack_pop(&mut stack, &op, &[Types::Int])?;
                        stack_pop(&mut stack, &op, &[Types::Int])?;
                        stack.push(Types::Int);
                    },
                    InstructionType::DivMod => {
                        stack_pop(&mut stack, &op, &[Types::Int])?;
                        stack_pop(&mut stack, &op, &[Types::Int])?;
                        stack.push(Types::Int);
                        stack.push(Types::Int);
                    },
                    InstructionType::Mul => {
                        stack_pop(&mut stack, &op, &[Types::Int])?;
                        stack_pop(&mut stack, &op, &[Types::Int])?;
                        stack.push(Types::Int);
                    },
                    InstructionType::Load8 => {
                        stack_pop(&mut stack, &op, &[Types::Ptr])?;
                        stack.push(Types::Int);
                    },
                    InstructionType::Store8 => {
                        stack_pop(&mut stack, &op, &[Types::Int])?;
                        stack_pop(&mut stack, &op, &[Types::Ptr])?;
                    },
                    InstructionType::Load32 => {
                        stack_pop(&mut stack, &op, &[Types::Ptr])?;
                        stack.push(Types::Int);
                    },
                    InstructionType::Store32 => {
                        stack_pop(&mut stack, &op, &[Types::Int])?;
                        stack_pop(&mut stack, &op, &[Types::Ptr])?;
                    },
                    InstructionType::Load64 => {
                        stack_pop(&mut stack, &op, &[Types::Ptr])?;
                        stack.push(Types::Int);
                    },
                    InstructionType::Store64 => {
                        stack_pop(&mut stack, &op, &[Types::Int])?;
                        stack_pop(&mut stack, &op, &[Types::Ptr])?;
                    },
                    InstructionType::Syscall0 => {
                        stack_pop(&mut stack, &op, &[Types::Int])?;
                        stack.push(Types::Int);
                    },
                    InstructionType::Syscall1 => {
                        stack_pop(&mut stack, &op, &[Types::Int])?;
                        stack_pop(&mut stack, &op, &[Types::Any])?;
                        stack.push(Types::Int);
                    },
                    InstructionType::Syscall2 => {
                        stack_pop(&mut stack, &op, &[Types::Int])?;
                        stack_pop(&mut stack, &op, &[Types::Any])?;
                        stack_pop(&mut stack, &op, &[Types::Any])?;
                        stack.push(Types::Int);
                    },
                    InstructionType::Syscall3 => {
                        stack_pop(&mut stack, &op, &[Types::Int])?;
                        stack_pop(&mut stack, &op, &[Types::Any])?;
                        stack_pop(&mut stack, &op, &[Types::Any])?;
                        stack_pop(&mut stack, &op, &[Types::Any])?;
                        stack.push(Types::Int);
                    },
                    InstructionType::Syscall4 => {
                        stack_pop(&mut stack, &op, &[Types::Int])?;
                        stack_pop(&mut stack, &op, &[Types::Any])?;
                        stack_pop(&mut stack, &op, &[Types::Any])?;
                        stack_pop(&mut stack, &op, &[Types::Any])?;
                        stack_pop(&mut stack, &op, &[Types::Any])?;
                        stack.push(Types::Int);
                    },
                    InstructionType::Syscall5 => {
                        stack_pop(&mut stack, &op, &[Types::Int])?;
                        stack_pop(&mut stack, &op, &[Types::Any])?;
                        stack_pop(&mut stack, &op, &[Types::Any])?;
                        stack_pop(&mut stack, &op, &[Types::Any])?;
                        stack_pop(&mut stack, &op, &[Types::Any])?;
                        stack_pop(&mut stack, &op, &[Types::Any])?;
                        stack.push(Types::Int);
                    },
                    InstructionType::Syscall6 => {
                        stack_pop(&mut stack, &op, &[Types::Int])?;
                        stack_pop(&mut stack, &op, &[Types::Any])?;
                        stack_pop(&mut stack, &op, &[Types::Any])?;
                        stack_pop(&mut stack, &op, &[Types::Any])?;
                        stack_pop(&mut stack, &op, &[Types::Any])?;
                        stack_pop(&mut stack, &op, &[Types::Any])?;
                        stack_pop(&mut stack, &op, &[Types::Any])?;
                        stack.push(Types::Int);
                    },
                    InstructionType::CastBool => {
                        stack_pop(&mut stack, &op, &[Types::Any])?;
                        stack.push(Types::Bool);
                    },
                    InstructionType::CastPtr => {
                        stack_pop(&mut stack, &op, &[Types::Any])?;
                        stack.push(Types::Ptr);
                    },
                    InstructionType::CastInt => {
                        stack_pop(&mut stack, &op, &[Types::Any])?;
                        stack.push(Types::Int);
                    },
                    InstructionType::CastVoid => {
                        stack_pop(&mut stack, &op, &[Types::Any])?;
                        stack.push(Types::Any);
                    },
                    InstructionType::MemUse => {
                        stack.push(Types::Ptr);
                    },
                    InstructionType::FnCall  => {
                        stack_snapshots.push(stack.clone());
                        
                        let f = functions.get(&op.text).unwrap();

                        let mut s = stack.clone();
                        let mut a = f.args.clone();
                        // s.reverse();
                        a.reverse();

                        for t in a{
                            if let Some(s2) = s.pop(){
                                if t != s2 {
                                    lerror!(&op.loc, "Expected {:?}, but got {:?}", t, s2);
                                    return Err(eyre!(""));
                                }
                            } else {
                                lerror!(&op.loc, "Expected {:?}, but got nothing", t);
                                return Err(eyre!(""));
                            }
                        }

                        
                    }
                    InstructionType::Return => {
                        let snap = stack_snapshots.pop().unwrap();
                        // snap.append(&mut f.returns.clone());
                        let mut st = stack.clone();
                        for s in snap{
                            if let Some(sn) = st.pop(){
                                if s != sn {
                                    lerror!(&op.loc, "Expected {:?}, but got {:?}", s, sn);
                                    return Err(eyre!(""));
                                }
                            } else {
                                lerror!(&op.loc, "Expected {:?}, but got nothing", s);
                                return Err(eyre!(""));
                            }
                        }
                    }
                    InstructionType::None => {},

                    InstructionType::TypeBool |
                    InstructionType::TypePtr |
                    InstructionType::TypeInt |
                    InstructionType::TypeVoid |
                    InstructionType::TypeAny |
                    InstructionType::TypeStr |
                    InstructionType::Returns |
                    InstructionType::With => (),
                    InstructionType::ConstUse => todo!(),
                }
            },
            
        }

        
    }
    
    Ok(ops.clone())
}



fn stack_pop(v: &mut Vec<Types>, op: &Operator, t: &[Types]) -> Result<Types> {
    if v.is_empty() {
        lerror!(&op.loc, "Expected {:?}, but got nothing", t);
        return Err(eyre!(""));
    }
    let r = v.pop().unwrap();

    if !t.contains(&r) && t[0] != Types::Any {
        lerror!(&op.loc, "Expected {:?}, but got {:?}", t, r);
        return Err(eyre!(""));
    }

    Ok(r)
}
