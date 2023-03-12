global _start
segment .text
    print:
        mov  r8, -3689348814741910323
        sub     rsp, 40
        mov     BYTE [rsp+32], 10
        lea     rcx, [rsp+31]
.L2:
        mov     rax, rdi
        mul     r8
        mov     rax, rdi
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
        lea     rcx, [rsp+32]
        lea     rax, [rsp+32]
        mov     edi, 1
        sub     rax, rdx
        sub     rdx, rcx
        lea     rsi, [rsp+32+rdx]
        mov     rdx, rax
        mov     rax, 1
        syscall
        add     rsp, 40
        ret
_start:
    ; -- PUSH 35
    mov rax, 35
    push rax
    ; -- PUSH 34
    mov rax, 34
    push rax
    ; -- PLUS
    pop rax
    pop rbx
    add rax, rbx
    push rax
    ; -- PRINT
    pop rdi
    call print
    mov rax, 60
    mov rdi, 0
    syscall
