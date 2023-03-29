BITS 64
segment .text
extern WriteConsoleA
extern GetStdHandle
extern ExitProcess
print:
    mov     r9, -3689348814741910323
    sub     rsp, 40
    mov     BYTE [rsp+31], 10
    lea     rcx, [rsp+30]
.L2:
    mov     rax, rdi
    lea     r8, [rsp+32]
    mul     r9
    mov     rax, rdi
    sub     r8, rcx
    shr     rdx, 3
    lea     rsi, [rdx+rdx*4]
    add     rsi, rsi
    sub     rax, rsi
    add     eax, 48
    mov     BYTE [rcx], al
    mov     rax, rdi
    mov     rdi, rdx
    mov     rdx, rcx
    sub     rcx, 1
    cmp     rax, 9
    ja      .L2
    lea     rax, [rsp+32]
    mov     edi, 1
    sub     rdx, rax
    xor     eax, eax
    lea     rsi, [rsp+32+rdx]
    mov     rdx, r8
    sub rsp, 8+8+32
    mov ecx, -11
    call GetStdHandle
    mov rcx, rax
    mov r8,  rdx
    mov rdx, rsi
    lea r9,  [rsp-16]
    mov qword [rsp-56], 0
    add rsp, 8+8+32+40
    call WriteConsoleA
    ret
global _start
_start:
sub rsp, 8
addr_0:
    ;; -- push int 10
    mov rax, 10
    push rax
addr_1:
    ;; -- print
    pop rdi
    call print
addr_2:
    mov     rcx, 0
    call    ExitProcess
segment .data
segment .bss
mem: resb 640000
