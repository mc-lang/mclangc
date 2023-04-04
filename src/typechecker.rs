use crate::{constants::{Operator, Types, OpType, KeywordType, InstructionType}, Args, lerror, warn};
use color_eyre::Result;
use eyre::eyre;


pub fn typecheck(ops: &[Operator], args: &Args) -> Result<Vec<Operator>>{
    if args.unsaf {
        if !args.quiet {
            warn!("Unsafe mode enabled, disabling typechecker, goodluck");
        }
        return Ok(ops.to_vec());
    }

    let mut stack: Vec<Types> = Vec::new();

    for op in ops {
        match op.typ.clone() {
            OpType::Keyword(keyword) => {
                match keyword {
                    KeywordType::If => {
                        stack_pop(&mut stack, &op, &[Types::Bool])?;
                    },
                    KeywordType::Do => {
                        stack_pop(&mut stack, &op, &[Types::Bool])?;
                    },

                    KeywordType::Else |
                    KeywordType::End |
                    KeywordType::While |
                    KeywordType::Function |
                    KeywordType::Include |
                    KeywordType::Constant |
                    KeywordType::Memory => (),
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
                        stack_pop(&mut stack, &op, &[Types::Int, Types::Ptr])?;
                        stack.push(Types::Int);
                    },
                    InstructionType::Plus => {
                        stack_pop(&mut stack, &op, &[Types::Int])?;
                        stack_pop(&mut stack, &op, &[Types::Int, Types::Ptr])?;
                        stack.push(Types::Int);
                    },
                    InstructionType::Equals => {
                        stack_pop(&mut stack, &op, &[Types::Int, Types::Ptr])?;
                        stack_pop(&mut stack, &op, &[Types::Int, Types::Ptr])?;
                        stack.push(Types::Bool);
                    },
                    InstructionType::Gt => {
                        stack_pop(&mut stack, &op, &[Types::Int, Types::Ptr])?;
                        stack_pop(&mut stack, &op, &[Types::Int, Types::Ptr])?;
                        stack.push(Types::Bool);
                    },
                    InstructionType::Lt => {
                        stack_pop(&mut stack, &op, &[Types::Int, Types::Ptr])?;
                        stack_pop(&mut stack, &op, &[Types::Int, Types::Ptr])?;
                        stack.push(Types::Bool);
                    },
                    InstructionType::Ge => {
                        stack_pop(&mut stack, &op, &[Types::Int, Types::Ptr])?;
                        stack_pop(&mut stack, &op, &[Types::Int, Types::Ptr])?;
                        stack.push(Types::Bool);
                    },
                    InstructionType::Le => {
                        stack_pop(&mut stack, &op, &[Types::Int, Types::Ptr])?;
                        stack_pop(&mut stack, &op, &[Types::Int, Types::Ptr])?;
                        stack.push(Types::Bool);
                    },
                    InstructionType::NotEquals => {
                        stack_pop(&mut stack, &op, &[Types::Int, Types::Ptr])?;
                        stack_pop(&mut stack, &op, &[Types::Int, Types::Ptr])?;
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
                    InstructionType::Mem => {
                        stack.push(Types::Ptr);
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
                    InstructionType::MemUse => {
                        stack.push(Types::Ptr);
                    },
                    InstructionType::FnCall |
                    InstructionType::Return |
                    InstructionType::None => {},
                }
            },
        }
    }

    Ok(ops.to_vec())
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
