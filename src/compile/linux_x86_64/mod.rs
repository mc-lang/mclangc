mod commands;
use std::{fs, io::{Write, BufWriter}};
use crate::{constants::{Operator, OpType, KeywordType}, Args};
use color_eyre::Result;
use commands::linux_x86_64_compile_and_link;
use commands::linux_x86_64_run;
use crate::constants::InstructionType;

use super::Folders;


pub fn compile(tokens: &[Operator], args: &Args, folders: &Folders) -> Result<i32>{

    let file = fs::File::create(&folders.of_a)?;
    let mut writer = BufWriter::new(&file);


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
        match token.typ {
            // stack
            OpType::Instruction(InstructionType::PushInt) => {
                writeln!(writer, "    ;; -- push int {}", token.value)?;
                writeln!(writer, "    mov rax, {}", token.value)?;
                writeln!(writer, "    push rax")?;
                ti += 1;
            },
            OpType::Instruction(InstructionType::PushStr) => {
                writeln!(writer, "    ;; -- push str \"{}\"", token.text.escape_default())?;
                writeln!(writer, "    mov rax, {}", token.text.len())?;
                writeln!(writer, "    push rax")?;
                writeln!(writer, "    push str_{}", strings.len())?;
                strings.push(token.text.clone());
                ti += 1;
            }
            OpType::Instruction(InstructionType::Drop) => {
                writeln!(writer, "    ;; -- drop")?;
                writeln!(writer, "    pop rax")?;
                ti += 1;
            },
            OpType::Instruction(InstructionType::Print) => {
                writeln!(writer, "    ;; -- print")?;
                writeln!(writer, "    pop rdi")?;
                writeln!(writer, "    call print")?;
                ti += 1;
            },

            OpType::Instruction(InstructionType::Dup) => {
                writeln!(writer, "    ;; -- dup")?;
                writeln!(writer, "    pop rax")?;
                writeln!(writer, "    push rax")?;
                writeln!(writer, "    push rax")?;

                ti += 1;
            },

            OpType::Instruction(InstructionType::Rot) => {
                writeln!(writer, "    ;; -- rot")?;
                writeln!(writer, "    pop rax")?;
                writeln!(writer, "    pop rbx")?;
                writeln!(writer, "    pop rcx")?;
                writeln!(writer, "    push rbx")?;
                writeln!(writer, "    push rax")?;
                writeln!(writer, "    push rcx")?;

                ti += 1;
            },
            OpType::Instruction(InstructionType::Swap) => {
                writeln!(writer, "    ;; -- swap")?;
                writeln!(writer, "    pop rax")?;
                writeln!(writer, "    pop rbx")?;
                writeln!(writer, "    push rax")?;
                writeln!(writer, "    push rbx")?;

                ti += 1;
            },
            OpType::Instruction(InstructionType::Over) => {
                writeln!(writer, "    ;; -- over")?;
                writeln!(writer, "    pop rax")?;
                writeln!(writer, "    pop rbx")?;
                writeln!(writer, "    push rbx")?;
                writeln!(writer, "    push rax")?;
                writeln!(writer, "    push rbx")?;

                ti += 1;
            },

            //mem
            OpType::Instruction(InstructionType::Mem) => {
                writeln!(writer, "    ;; -- mem")?;
                writeln!(writer, "    push mem")?;
                ti += 1;
            }
            OpType::Instruction(InstructionType::Load8) => {
                writeln!(writer, "    ;; -- load")?;
                writeln!(writer, "    pop rax")?;
                writeln!(writer, "    xor rbx, rbx")?;
                writeln!(writer, "    mov bl, [rax]")?;
                writeln!(writer, "    push rbx")?;
                ti += 1;
            }

            OpType::Instruction(InstructionType::Store8) => {
                writeln!(writer, "    ;; -- store")?;
                writeln!(writer, "    pop rbx")?;
                writeln!(writer, "    pop rax")?;
                writeln!(writer, "    mov [rax], bl")?;
                ti += 1;
            }

            // math
            OpType::Instruction(InstructionType::Plus) => {
                writeln!(writer, "    ;; -- plus")?;
                writeln!(writer, "    pop rax")?;
                writeln!(writer, "    pop rbx")?;
                writeln!(writer, "    add rax, rbx")?;
                writeln!(writer, "    push rax")?;
                ti += 1;
            },
            OpType::Instruction(InstructionType::Minus) => {
                writeln!(writer, "    ;; -- minus")?;
                writeln!(writer, "    pop rax")?;
                writeln!(writer, "    pop rbx")?;
                writeln!(writer, "    sub rbx, rax")?;
                writeln!(writer, "    push rbx")?;
                ti += 1;
            },
            OpType::Instruction(InstructionType::Equals) => {
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
            OpType::Instruction(InstructionType::Lt) => {
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
            OpType::Instruction(InstructionType::Gt) => {
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
            OpType::Instruction(InstructionType::NotEquals) => {
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
            OpType::Instruction(InstructionType::Le) => {
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
            OpType::Instruction(InstructionType::Ge) => {
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
            OpType::Instruction(InstructionType::Band) => {
                writeln!(writer, "    ;; -- band")?;
                writeln!(writer, "    pop rax")?;
                writeln!(writer, "    pop rbx")?;
                writeln!(writer, "    and rbx, rax")?;
                writeln!(writer, "    push rbx")?;
                ti += 1;
            },
            OpType::Instruction(InstructionType::Bor) => {
                writeln!(writer, "    ;; -- bor")?;
                writeln!(writer, "    pop rax")?;
                writeln!(writer, "    pop rbx")?;
                writeln!(writer, "    or rbx, rax")?;
                writeln!(writer, "    push rbx")?;
                ti += 1;
            },
            OpType::Instruction(InstructionType::Shr) => {
                writeln!(writer, "    ;; -- shr")?;
                writeln!(writer, "    pop rcx")?;
                writeln!(writer, "    pop rbx")?;
                writeln!(writer, "    shr rbx, cl")?;
                writeln!(writer, "    push rbx")?;
                ti += 1;
            },
            OpType::Instruction(InstructionType::Shl) => {
                writeln!(writer, "    ;; -- shl")?;
                writeln!(writer, "    pop rcx")?;
                writeln!(writer, "    pop rbx")?;
                writeln!(writer, "    shl rbx, cl")?;
                writeln!(writer, "    push rbx")?;
                ti += 1;
            },
            OpType::Instruction(InstructionType::DivMod) => {
                writeln!(writer, "    ;; -- div")?;
                writeln!(writer, "    xor rdx, rdx")?;
                writeln!(writer, "    pop rbx")?;
                writeln!(writer, "    pop rax")?;
                writeln!(writer, "    div rbx")?;
                writeln!(writer, "    push rax")?;
                writeln!(writer, "    push rdx")?;
                ti += 1;
            },
            OpType::Instruction(InstructionType::Mul) => {
                writeln!(writer, "    ;; -- mul")?;
                writeln!(writer, "    pop rax")?;
                writeln!(writer, "    pop rbx")?;
                writeln!(writer, "    mul rbx")?;
                writeln!(writer, "    push rax")?;
                ti += 1;
            },


            // block
            OpType::Keyword(KeywordType::If) => {
                writeln!(writer, "    ;; -- if")?;
                writeln!(writer, "    pop rax")?;
                writeln!(writer, "    test rax, rax")?;
                writeln!(writer, "    jz addr_{}", token.jmp)?;
                ti += 1;
            },
            OpType::Keyword(KeywordType::Else) => {
                writeln!(writer, "    ;; -- else")?;
                writeln!(writer, "    jmp addr_{}", token.jmp)?;
                ti += 1;
            },
            OpType::Keyword(KeywordType::While) => {
                writeln!(writer, "    ;; -- while")?;
                ti += 1;
            }
            OpType::Keyword(KeywordType::Do) => {
                writeln!(writer, "    ;; -- do")?;
                writeln!(writer, "    pop rax")?;
                writeln!(writer, "    test rax, rax")?;
                writeln!(writer, "    jz addr_{}", token.jmp)?;
                ti += 1;
            }
            OpType::Keyword(KeywordType::End) => {
                writeln!(writer, "    ;; -- end")?;
                if ti + 1 != token.jmp {
                    writeln!(writer, "    jmp addr_{}", token.jmp)?;
                }
                ti += 1;
            },
            OpType::Instruction(InstructionType::Syscall0) => {
                writeln!(writer, "    ;; -- syscall0")?;
                writeln!(writer, "    pop rax")?;
                writeln!(writer, "    syscall")?;
                writeln!(writer, "    push rax")?;
                ti += 1;
            },
            OpType::Instruction(InstructionType::Syscall1) => {
                writeln!(writer, "    ;; -- syscall1")?;
                writeln!(writer, "    pop rax")?;
                writeln!(writer, "    pop rdi")?;
                writeln!(writer, "    syscall")?;
                writeln!(writer, "    push rax")?;
                ti += 1;
            },
            OpType::Instruction(InstructionType::Syscall2) => {
                writeln!(writer, "    ;; -- syscall2")?;
                writeln!(writer, "    pop rax")?;
                writeln!(writer, "    pop rdi")?;
                writeln!(writer, "    pop rsi")?;
                writeln!(writer, "    syscall")?;
                writeln!(writer, "    push rax")?;
                ti += 1;
            },
            OpType::Instruction(InstructionType::Syscall3) => {
                writeln!(writer, "    ;; -- syscall3")?;
                writeln!(writer, "    pop rax")?;
                writeln!(writer, "    pop rdi")?;
                writeln!(writer, "    pop rsi")?;
                writeln!(writer, "    pop rdx")?;
                writeln!(writer, "    syscall")?;
                writeln!(writer, "    push rax")?;

                ti += 1;
            },
            OpType::Instruction(InstructionType::Syscall4) => {
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
            OpType::Instruction(InstructionType::Syscall5) => {
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
            OpType::Instruction(InstructionType::Syscall6) => {
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
            OpType::Instruction(InstructionType::None) | OpType::Keyword(KeywordType::Macro) | OpType::Keyword(KeywordType::Include) => unreachable!()
        }
    }
    writeln!(writer, "addr_{ti}:")?;
    writeln!(writer, "    mov rax, 60")?;
    writeln!(writer, "    mov rdi, 0")?;
    writeln!(writer, "    syscall")?;
    writeln!(writer, "segment .data")?;
    for (_, s) in strings.iter().enumerate() {
        let s_chars = s.chars().map(|c| (c as u32).to_string()).collect::<Vec<String>>();
        let s_list = s_chars.join(",");
        writeln!(writer, "    str_{}: db {} ; {}", s, s_list, s.escape_default())?;
    }

    writeln!(writer, "segment .bss")?;
    writeln!(writer, "mem: resb {}", crate::compile::MEM_SZ)?;

    writer.flush()?;
    linux_x86_64_compile_and_link(folders, args.quiet)?;
    if args.run {
        let c = linux_x86_64_run(&folders.of_c, &[], args.quiet)?;
        return Ok(c);
    }

    Ok(0)
}