section .bss
    buf resb 16

section .text
    global _start

_start:
    ; read A,B,C
    in rax
    in rbx
    in rcx

    ; if A > B then swap(A,B)
    cmp rax, rbx
    js .ab_ok
    jz .ab_ok
    push rax
    mov rax, rbx
    pop rbx
.ab_ok:

    ; if B > C then swap(B,C)
    cmp rbx, rcx
    js .bc_ok
    jz .bc_ok
    push rbx
    mov rbx, rcx
    pop rcx
.bc_ok:

    ; again: if A > B then swap(A,B)
    cmp rax, rbx
    js .ab2_ok
    jz .ab2_ok
    push rax
    mov rax, rbx
    pop rbx
.ab2_ok:

    mov rsi, buf
    mov [rsi], rax
    inc rsi
    mov [rsi], rbx
    inc rsi
    mov [rsi], rcx
    
    ; write(1, buf, 3)
    mov rax, 1
    mov rdi, 1
    mov rsi, buf
    mov rdx, 3
    syscall
    
    ; exit(0)
    mov rax, 60
    xor rdi, rdi
    syscall

