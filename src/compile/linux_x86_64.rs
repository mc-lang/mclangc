use std::{fs, path::PathBuf, io::{Write, BufWriter}};
use crate::{constants::{Operator, OpType, KeywordType}, Args};
use color_eyre::Result;
use crate::compile::commands::linux_x86_64_compile_and_link;
use crate::constants::InstructionType;
use super::commands::linux_x86_64_run;


pub fn compile(tokens: &[Operator], args: &Args) -> Result<i32>{
    let mut of_c = PathBuf::from(&args.out_file);
    let (mut of_o, mut of_a) = if args.out_file == *crate::DEFAULT_OUT_FILE {
        let of_o = PathBuf::from("/tmp/mclang_comp.o");
        let of_a = PathBuf::from("/tmp/mclang_comp.nasm");
        (of_o, of_a)
    } else {
        let of_o = PathBuf::from(&args.out_file);
        let of_a = PathBuf::from(&args.out_file);
        (of_o, of_a)
    };

    of_c.set_extension("");
    of_o.set_extension("o");
    of_a.set_extension("nasm");

    

    let file = fs::File::create(&of_a)?;
    let mut writer = BufWriter::new(&file);
    let mut memories:  Vec<(usize, usize)> = Vec::new();
    // println!("{}", tokens.len());
    let mut strings: Vec<String> = Vec::new();

    writeln!(writer, "BITS 64")?;
    writeln!(writer, "segment .text")?;

    writeln!(writer, "print:")?;
    writeln!(writer, "    mov     r9, -3689348814741910323")?;
    writeln!(writer, "    sub     rsp, 40")?;
    writeln!(writer, "    mov     BYTE [rsp+31], 10")?;
    writeln!(writer, "    lea     rcx, [rsp+30]")?;
    writeln!(writer, ".L2:")?;
    writeln!(writer, "    mov     rax, rdi")?;
    writeln!(writer, "    lea     r8, [rsp+32]")?;
    writeln!(writer, "    mul     r9")?;
    writeln!(writer, "    mov     rax, rdi")?;
    writeln!(writer, "    sub     r8, rcx")?;
    writeln!(writer, "    shr     rdx, 3")?;
    writeln!(writer, "    lea     rsi, [rdx+rdx*4]")?;
    writeln!(writer, "    add     rsi, rsi")?;
    writeln!(writer, "    sub     rax, rsi")?;
    writeln!(writer, "    add     eax, 48")?;
    writeln!(writer, "    mov     BYTE [rcx], al")?;
    writeln!(writer, "    mov     rax, rdi")?;
    writeln!(writer, "    mov     rdi, rdx")?;
    writeln!(writer, "    mov     rdx, rcx")?;
    writeln!(writer, "    sub     rcx, 1")?;
    writeln!(writer, "    cmp     rax, 9")?;
    writeln!(writer, "    ja      .L2")?;
    writeln!(writer, "    lea     rax, [rsp+32]")?;
    writeln!(writer, "    mov     edi, 1")?;
    writeln!(writer, "    sub     rdx, rax")?;
    writeln!(writer, "    xor     eax, eax")?;
    writeln!(writer, "    lea     rsi, [rsp+32+rdx]")?;
    writeln!(writer, "    mov     rdx, r8")?;
    writeln!(writer, "    mov     rax, 1")?;
    writeln!(writer, "    syscall")?;
    writeln!(writer, "    add     rsp, 40")?;
    writeln!(writer, "    ret")?;

    writeln!(writer, "global _start")?;
    writeln!(writer, "_start:")?;

    let mut ti = 0;
    while ti < tokens.len() {
        let token = &tokens[ti];

        writeln!(writer, "addr_{ti}:")?;
        match token.typ.clone() {
            // stack

            OpType::Instruction(instruction) => {
                match instruction {
                    InstructionType::PushInt => {
                        writeln!(writer, "    ;; -- push int {}", token.value)?;
                        writeln!(writer, "    mov rax, {}", token.value)?;
                        writeln!(writer, "    push rax")?;
                        ti += 1;
                    },
                    InstructionType::PushStr => {
                        writeln!(writer, "    ;; -- push str \"{}\"", token.text.escape_default())?;
                        writeln!(writer, "    mov rax, {}", token.text.len())?;
                        writeln!(writer, "    push rax")?;
                        writeln!(writer, "    push str_{}", strings.len())?;
                        strings.push(token.text.clone());
                        ti += 1;
                    }
                    InstructionType::Drop => {
                        writeln!(writer, "    ;; -- drop")?;
                        writeln!(writer, "    pop rax")?;
                        ti += 1;
                    },
                    InstructionType::Print => {
                        writeln!(writer, "    ;; -- print")?;
                        writeln!(writer, "    pop rdi")?;
                        writeln!(writer, "    call print")?;
                        ti += 1;
                    },
        
                    InstructionType::Dup => {
                        writeln!(writer, "    ;; -- dup")?;
                        writeln!(writer, "    pop rax")?;
                        writeln!(writer, "    push rax")?;
                        writeln!(writer, "    push rax")?;
        
                        ti += 1;
                    },
        
                    InstructionType::Rot => {
                        writeln!(writer, "    ;; -- rot")?;
                        writeln!(writer, "    pop rax")?;
                        writeln!(writer, "    pop rbx")?;
                        writeln!(writer, "    pop rcx")?;
                        writeln!(writer, "    push rbx")?;
                        writeln!(writer, "    push rax")?;
                        writeln!(writer, "    push rcx")?;
        
                        ti += 1;
                    },
                    InstructionType::Swap => {
                        writeln!(writer, "    ;; -- swap")?;
                        writeln!(writer, "    pop rax")?;
                        writeln!(writer, "    pop rbx")?;
                        writeln!(writer, "    push rax")?;
                        writeln!(writer, "    push rbx")?;
        
                        ti += 1;
                    },
                    InstructionType::Over => {
                        writeln!(writer, "    ;; -- over")?;
                        writeln!(writer, "    pop rax")?;
                        writeln!(writer, "    pop rbx")?;
                        writeln!(writer, "    push rbx")?;
                        writeln!(writer, "    push rax")?;
                        writeln!(writer, "    push rbx")?;
        
                        ti += 1;
                    },
        
                    //mem
                    InstructionType::Mem => {
                        writeln!(writer, "    ;; -- mem")?;
                        writeln!(writer, "    push mem")?;
                        ti += 1;
                    }
                    InstructionType::Load8 => {
                        writeln!(writer, "    ;; -- load")?;
                        writeln!(writer, "    pop rax")?;
                        writeln!(writer, "    xor rbx, rbx")?;
                        writeln!(writer, "    mov bl, byte [rax]")?;
                        writeln!(writer, "    push rbx")?;
                        ti += 1;
                    }
        
                    InstructionType::Store8 => {
                        writeln!(writer, "    ;; -- store")?;
                        writeln!(writer, "    pop rbx")?;
                        writeln!(writer, "    pop rax")?;
                        writeln!(writer, "    mov byte [rax], bl")?;
                        ti += 1;
                    }
                    InstructionType::Load32 => {
                        writeln!(writer, "    ;; -- load")?;
                        writeln!(writer, "    pop rax")?;
                        writeln!(writer, "    xor rbx, rbx")?;
                        writeln!(writer, "    mov bl, dword [rax]")?;
                        writeln!(writer, "    push rbx")?;
                        ti += 1;
                    }
        
                    InstructionType::Store32 => {
                        writeln!(writer, "    ;; -- store")?;
                        writeln!(writer, "    pop rbx")?;
                        writeln!(writer, "    pop rax")?;
                        writeln!(writer, "    mov dword[rax], bl")?;
                        ti += 1;
                    }
                    InstructionType::Load64 => {
                        writeln!(writer, "    ;; -- load")?;
                        writeln!(writer, "    pop rax")?;
                        writeln!(writer, "    xor rbx, rbx")?;
                        writeln!(writer, "    mov bl, qword [rax]")?;
                        writeln!(writer, "    push rbx")?;
                        ti += 1;
                    }
        
                    InstructionType::Store64 => {
                        writeln!(writer, "    ;; -- store")?;
                        writeln!(writer, "    pop rbx")?;
                        writeln!(writer, "    pop rax")?;
                        writeln!(writer, "    mov qword [rax], bl")?;
                        ti += 1;
                    }
        
                    // math
                    InstructionType::Plus => {
                        writeln!(writer, "    ;; -- plus")?;
                        writeln!(writer, "    pop rax")?;
                        writeln!(writer, "    pop rbx")?;
                        writeln!(writer, "    add rax, rbx")?;
                        writeln!(writer, "    push rax")?;
                        ti += 1;
                    },
                    InstructionType::Minus => {
                        writeln!(writer, "    ;; -- minus")?;
                        writeln!(writer, "    pop rax")?;
                        writeln!(writer, "    pop rbx")?;
                        writeln!(writer, "    sub rbx, rax")?;
                        writeln!(writer, "    push rbx")?;
                        ti += 1;
                    },
                    InstructionType::Equals => {
                        writeln!(writer, "    ;; -- equals")?;
                        writeln!(writer, "    mov rcx, 0")?;
                        writeln!(writer, "    mov rdx, 1")?;
                        writeln!(writer, "    pop rax")?;
                        writeln!(writer, "    pop rbx")?;
                        writeln!(writer, "    cmp rax, rbx")?;
                        writeln!(writer, "    cmove rcx, rdx")?;
                        writeln!(writer, "    push rcx")?;
                        ti += 1;
                    },
                    InstructionType::Lt => {
                        writeln!(writer, "    ;; -- lt")?;
                        writeln!(writer, "    mov rcx, 0")?;
                        writeln!(writer, "    mov rdx, 1")?;
                        writeln!(writer, "    pop rbx")?;
                        writeln!(writer, "    pop rax")?;
                        writeln!(writer, "    cmp rax, rbx")?;
                        writeln!(writer, "    cmovl rcx, rdx")?;
                        writeln!(writer, "    push rcx")?;
                        ti += 1;
                    },
                    InstructionType::Gt => {
                        writeln!(writer, "    ;; -- gt")?;
                        writeln!(writer, "    mov rcx, 0")?;
                        writeln!(writer, "    mov rdx, 1")?;
                        writeln!(writer, "    pop rbx")?;
                        writeln!(writer, "    pop rax")?;
                        writeln!(writer, "    cmp rax, rbx")?;
                        writeln!(writer, "    cmovg rcx, rdx")?;
                        writeln!(writer, "    push rcx")?;
                        ti += 1;
                    },
                    InstructionType::NotEquals => {
                        writeln!(writer, "    ;; -- not equals")?;
                        writeln!(writer, "    mov rcx, 1")?;
                        writeln!(writer, "    mov rdx, 0")?;
                        writeln!(writer, "    pop rax")?;
                        writeln!(writer, "    pop rbx")?;
                        writeln!(writer, "    cmp rax, rbx")?;
                        writeln!(writer, "    cmove rcx, rdx")?;
                        writeln!(writer, "    push rcx")?;
                        ti += 1;
                    },
                    InstructionType::Le => {
                        writeln!(writer, "    ;; -- lt")?;
                        writeln!(writer, "    mov rcx, 0")?;
                        writeln!(writer, "    mov rdx, 1")?;
                        writeln!(writer, "    pop rbx")?;
                        writeln!(writer, "    pop rax")?;
                        writeln!(writer, "    cmp rax, rbx")?;
                        writeln!(writer, "    cmovle rcx, rdx")?;
                        writeln!(writer, "    push rcx")?;
                        ti += 1;
                    },
                    InstructionType::Ge => {
                        writeln!(writer, "    ;; -- gt")?;
                        writeln!(writer, "    mov rcx, 0")?;
                        writeln!(writer, "    mov rdx, 1")?;
                        writeln!(writer, "    pop rbx")?;
                        writeln!(writer, "    pop rax")?;
                        writeln!(writer, "    cmp rax, rbx")?;
                        writeln!(writer, "    cmovge rcx, rdx")?;
                        writeln!(writer, "    push rcx")?;
                        ti += 1;
                    },
                    InstructionType::Band => {
                        writeln!(writer, "    ;; -- band")?;
                        writeln!(writer, "    pop rax")?;
                        writeln!(writer, "    pop rbx")?;
                        writeln!(writer, "    and rbx, rax")?;
                        writeln!(writer, "    push rbx")?;
                        ti += 1;
                    },
                    InstructionType::Bor => {
                        writeln!(writer, "    ;; -- bor")?;
                        writeln!(writer, "    pop rax")?;
                        writeln!(writer, "    pop rbx")?;
                        writeln!(writer, "    or rbx, rax")?;
                        writeln!(writer, "    push rbx")?;
                        ti += 1;
                    },
                    InstructionType::Shr => {
                        writeln!(writer, "    ;; -- shr")?;
                        writeln!(writer, "    pop rcx")?;
                        writeln!(writer, "    pop rbx")?;
                        writeln!(writer, "    shr rbx, cl")?;
                        writeln!(writer, "    push rbx")?;
                        ti += 1;
                    },
                    InstructionType::Shl => {
                        writeln!(writer, "    ;; -- shl")?;
                        writeln!(writer, "    pop rcx")?;
                        writeln!(writer, "    pop rbx")?;
                        writeln!(writer, "    shl rbx, cl")?;
                        writeln!(writer, "    push rbx")?;
                        ti += 1;
                    },
                    InstructionType::DivMod => {
                        writeln!(writer, "    ;; -- div")?;
                        writeln!(writer, "    xor rdx, rdx")?;
                        writeln!(writer, "    pop rbx")?;
                        writeln!(writer, "    pop rax")?;
                        writeln!(writer, "    div rbx")?;
                        writeln!(writer, "    push rax")?;
                        writeln!(writer, "    push rdx")?;
                        ti += 1;
                    },
                    InstructionType::Mul => {
                        writeln!(writer, "    ;; -- mul")?;
                        writeln!(writer, "    pop rax")?;
                        writeln!(writer, "    pop rbx")?;
                        writeln!(writer, "    mul rbx")?;
                        writeln!(writer, "    push rax")?;
                        ti += 1;
                    },
                    InstructionType::Syscall0 => {
                        writeln!(writer, "    ;; -- syscall0")?;
                        writeln!(writer, "    pop rax")?;
                        writeln!(writer, "    syscall")?;
                        writeln!(writer, "    push rax")?;
                        ti += 1;
                    },
                    InstructionType::Syscall1 => {
                        writeln!(writer, "    ;; -- syscall1")?;
                        writeln!(writer, "    pop rax")?;
                        writeln!(writer, "    pop rdi")?;
                        writeln!(writer, "    syscall")?;
                        writeln!(writer, "    push rax")?;
                        ti += 1;
                    },
                    InstructionType::Syscall2 => {
                        writeln!(writer, "    ;; -- syscall2")?;
                        writeln!(writer, "    pop rax")?;
                        writeln!(writer, "    pop rdi")?;
                        writeln!(writer, "    pop rsi")?;
                        writeln!(writer, "    syscall")?;
                        writeln!(writer, "    push rax")?;
                        ti += 1;
                    },
                    InstructionType::Syscall3 => {
                        writeln!(writer, "    ;; -- syscall3")?;
                        writeln!(writer, "    pop rax")?;
                        writeln!(writer, "    pop rdi")?;
                        writeln!(writer, "    pop rsi")?;
                        writeln!(writer, "    pop rdx")?;
                        writeln!(writer, "    syscall")?;
                        writeln!(writer, "    push rax")?;
        
                        ti += 1;
                    },
                    InstructionType::Syscall4 => {
                        writeln!(writer, "    ;; -- syscall4")?;
                        writeln!(writer, "    pop rax")?;
                        writeln!(writer, "    pop rdi")?;
                        writeln!(writer, "    pop rsi")?;
                        writeln!(writer, "    pop rdx")?;
                        writeln!(writer, "    pop r10")?;
                        writeln!(writer, "    syscall")?;
                        writeln!(writer, "    push rax")?;
                        ti += 1;
                    },
                    InstructionType::Syscall5 => {
                        writeln!(writer, "    ;; -- syscall5")?;
                        writeln!(writer, "    pop rax")?;
                        writeln!(writer, "    pop rdi")?;
                        writeln!(writer, "    pop rsi")?;
                        writeln!(writer, "    pop rdx")?;
                        writeln!(writer, "    pop r10")?;
                        writeln!(writer, "    pop r8")?;
                        writeln!(writer, "    syscall")?;
                        writeln!(writer, "    push rax")?;
                        ti += 1;
                    },
                    InstructionType::Syscall6 => {
                        writeln!(writer, "    ;; -- syscall6")?;
                        writeln!(writer, "    pop rax")?;
                        writeln!(writer, "    pop rdi")?;
                        writeln!(writer, "    pop rsi")?;
                        writeln!(writer, "    pop rdx")?;
                        writeln!(writer, "    pop r10")?;
                        writeln!(writer, "    pop r8")?;
                        writeln!(writer, "    pop r9")?;
                        writeln!(writer, "    syscall")?;
                        writeln!(writer, "    push rax")?;
                        ti += 1;
                    },
                    InstructionType::MemUse => {
                        writeln!(writer, "    ;; -- MemUse")?;
                        writeln!(writer, "    push mem_{}", token.addr.unwrap())?;
                        ti += 1;
                    },
                    InstructionType::None => unreachable!(),
                    InstructionType::CastBool => ti += 1,
                    InstructionType::CastPtr => ti += 1,
                    InstructionType::CastInt => ti += 1,
                }
            }


            OpType::Keyword(keyword) => {
                match keyword {

                    // block
                    KeywordType::If => {
                        writeln!(writer, "    ;; -- if")?;
                        writeln!(writer, "    pop rax")?;
                        writeln!(writer, "    test rax, rax")?;
                        writeln!(writer, "    jz addr_{}", token.jmp)?;
                        ti += 1;
                    },
                    KeywordType::Else => {
                        writeln!(writer, "    ;; -- else")?;
                        writeln!(writer, "    jmp addr_{}", token.jmp)?;
                        ti += 1;
                    },
                    KeywordType::While => {
                        writeln!(writer, "    ;; -- while")?;
                        ti += 1;
                    }
                    KeywordType::Do => {
                        writeln!(writer, "    ;; -- do")?;
                        writeln!(writer, "    pop rax")?;
                        writeln!(writer, "    test rax, rax")?;
                        writeln!(writer, "    jz addr_{}", token.jmp)?;
                        ti += 1;
                    }
                    KeywordType::End => {
                        writeln!(writer, "    ;; -- end")?;
                        if ti + 1 != token.jmp {
                            writeln!(writer, "    jmp addr_{}", token.jmp)?;
                        }
                        ti += 1;
                    },
                    KeywordType::Memory => {
                        memories.push((token.addr.unwrap(), token.value));
                        ti += 1;
                    }
                    KeywordType::Macro |
                    KeywordType::Include
                        => unreachable!()
                }
            }
        }
    }
    writeln!(writer, "addr_{ti}:")?;
    writeln!(writer, "    mov rax, 60")?;
    writeln!(writer, "    mov rdi, 0")?;
    writeln!(writer, "    syscall")?;
    writeln!(writer, "segment .data")?;
    for (i, s) in strings.iter().enumerate() {
        let s_chars = s.chars().map(|c| (c as u32).to_string()).collect::<Vec<String>>();
        let s_list = s_chars.join(",");
        writeln!(writer, "    str_{}: db {} ; {}", i, s_list, s.escape_default())?;
    }
    
    writeln!(writer, "segment .bss")?;
    for (_, s) in memories.iter().enumerate() {
        writeln!(writer, "    mem_{}: resb {}", s.0, s.1)?;
    }
    writeln!(writer, "    mem: resb {}", crate::compile::MEM_SZ)?;

    writer.flush()?;
    linux_x86_64_compile_and_link(&of_a, &of_o, &of_c, args.quiet)?;
    if args.run {
        let c = linux_x86_64_run(&of_c, &[], args.quiet)?;
        return Ok(c);
    }

    Ok(0)
}