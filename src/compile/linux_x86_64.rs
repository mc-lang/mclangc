use std::{fs, path::PathBuf, io::{Write, BufWriter}};
use crate::{constants::{Operator, OpType}, Args};
use color_eyre::Result;
use crate::compile::commands;

use super::commands::linux_x86_64_compile_and_link;

pub fn compile(tokens: Vec<Operator>, args: Args) -> Result<()>{
    let mut of_c = PathBuf::from(&args.out_file);
    let mut of_o = PathBuf::from(&args.out_file);
    let mut of_a = PathBuf::from(&args.out_file);
    
    of_c.set_extension("");
    of_o.set_extension("o");
    of_a.set_extension("nasm");

    let file = fs::File::create(&of_a)?;
    let mut writer = BufWriter::new(&file);

    writeln!(writer, "global _start")?;
    writeln!(writer, "segment .text")?;

    writeln!(writer, "    print:")?;
    writeln!(writer, "        mov  r8, -3689348814741910323")?;
    writeln!(writer, "        sub     rsp, 40")?;
    writeln!(writer, "        mov     BYTE [rsp+32], 10")?;
    writeln!(writer, "        lea     rcx, [rsp+31]")?;
    writeln!(writer, ".L2:")?;
    writeln!(writer, "        mov     rax, rdi")?;
    writeln!(writer, "        mul     r8")?;
    writeln!(writer, "        mov     rax, rdi")?;
    writeln!(writer, "        shr     rdx, 3")?;
    writeln!(writer, "        lea     rsi, [rdx+rdx*4]")?;
    writeln!(writer, "        add     rsi, rsi")?;
    writeln!(writer, "        sub     rax, rsi")?;
    writeln!(writer, "        add     eax, 48")?;
    writeln!(writer, "        mov     BYTE [rcx], al")?;
    writeln!(writer, "        mov     rax, rdi")?;
    writeln!(writer, "        mov     rdi, rdx")?;
    writeln!(writer, "        mov     rdx, rcx")?;
    writeln!(writer, "        sub     rcx, 1")?;
    writeln!(writer, "        cmp     rax, 9")?;
    writeln!(writer, "        ja      .L2")?;
    writeln!(writer, "        lea     rcx, [rsp+32]")?;
    writeln!(writer, "        lea     rax, [rsp+32]")?;
    writeln!(writer, "        mov     edi, 1")?;
    writeln!(writer, "        sub     rax, rdx")?;
    writeln!(writer, "        sub     rdx, rcx")?;
    writeln!(writer, "        lea     rsi, [rsp+32+rdx]")?;
    writeln!(writer, "        mov     rdx, rax")?;
    writeln!(writer, "        mov     rax, 1")?;
    writeln!(writer, "        syscall")?;
    writeln!(writer, "        add     rsp, 40")?;
    writeln!(writer, "        ret")?;

    writeln!(writer, "_start:")?;
    
    
    for token in tokens {
        match token.typ {
            OpType::Push => {
                writeln!(writer, "    ; -- PUSH {}", token.value)?;
                writeln!(writer, "    mov rax, {}", token.value)?;
                writeln!(writer, "    push rax")?;
            },
            OpType::Pop => {
                writeln!(writer, "    ; -- POP")?;
                writeln!(writer, "    pop")?;
            },
            OpType::Plus => {
                writeln!(writer, "    ; -- PLUS")?;
                writeln!(writer, "    pop rax")?;
                writeln!(writer, "    pop rbx")?;
                writeln!(writer, "    add rax, rbx")?;
                writeln!(writer, "    push rax")?;
            },
            OpType::Minus => {
                writeln!(writer, "    ; -- MINUS")?;
                writeln!(writer, "    pop rax")?;
                writeln!(writer, "    pop rbx")?;
                writeln!(writer, "    sub rbx, rax")?;
                writeln!(writer, "    push rax")?;
            },
            OpType::Print => {
                writeln!(writer, "    ; -- PRINT")?;
                writeln!(writer, "    pop rdi")?;
                writeln!(writer, "    call print")?;
            },
        }
    }

    writeln!(writer, "    mov rax, 60")?;
    writeln!(writer, "    mov rdi, 0")?;
    writeln!(writer, "    syscall")?;
    writer.flush()?;
    linux_x86_64_compile_and_link(of_a, of_o, of_c)?;
    Ok(())
}