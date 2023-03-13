use std::{fs, path::PathBuf, io::{Write, BufWriter}};
use crate::{constants::{Operator, OpType}, Args};
use color_eyre::Result;
use crate::compile::commands::linux_x86_64_compile_and_link;


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

    writeln!(writer, "_start:")?;
    
    let mut ti = 0;
    while ti < tokens.len() {
        let token = &tokens[ti];
        writeln!(writer, "addr_{}:", ti)?;
        match token.typ {
            OpType::Push => {
                writeln!(writer, "    ; -- PUSH {}", token.value)?;
                writeln!(writer, "    mov rax, {}", token.value)?;
                writeln!(writer, "    push rax")?;
                ti += 1;
                
            },
            OpType::Pop => {
                writeln!(writer, "    ; -- POP")?;
                writeln!(writer, "    pop")?;
                ti += 1;
            },
            OpType::Plus => {
                writeln!(writer, "    ; -- PLUS")?;
                writeln!(writer, "    pop rax")?;
                writeln!(writer, "    pop rbx")?;
                writeln!(writer, "    add rax, rbx")?;
                writeln!(writer, "    push rax")?;
                ti += 1;
            },
            OpType::Minus => {
                writeln!(writer, "    ; -- MINUS")?;
                writeln!(writer, "    pop rax")?;
                writeln!(writer, "    pop rbx")?;
                writeln!(writer, "    sub rbx, rax")?;
                writeln!(writer, "    push rbx")?;
                ti += 1;
            },
            OpType::Equals => {
                writeln!(writer, "    ; -- EQUALS")?;
                writeln!(writer, "    mov rcx, 0")?;
                writeln!(writer, "    mov rdx, 1")?;
                writeln!(writer, "    pop rax")?;
                writeln!(writer, "    pop rbx")?;
                writeln!(writer, "    cmp rax, rbx")?;
                writeln!(writer, "    cmove rcx, rdx")?;
                writeln!(writer, "    push rcx")?;
                ti += 1;

            },
            OpType::Print => {
                writeln!(writer, "    ; -- PRINT")?;
                writeln!(writer, "    pop rdi")?;
                writeln!(writer, "    call print")?;
                ti += 1;
            },
            OpType::If => {
                writeln!(writer, "    ; -- IF")?;
                writeln!(writer, "    pop rax")?;
                writeln!(writer, "    test rax, rax")?;
                writeln!(writer, "    jz addr_{}", token.value + 1)?;
                ti += 1;
            },
            OpType::Else => {
                writeln!(writer, "    ; -- ELSE")?;
                writeln!(writer, "    jmp addr_{}", token.value)?;
                ti += 1;
            },
            OpType::End => {
                writeln!(writer, "    ; -- END")?;
                ti += 1;
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