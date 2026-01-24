section .bss
    buf resb 16

section .text
    global _start

_start:
    in rax
    push rax
    in rax
    push rax
    in rax
    push rax

    mov rsi, buf
    pop rax
    mov [rsi], rax
    inc rsi
    pop rax
    mov [rsi], rax
    inc rsi
    pop rax
    mov [rsi], rax

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

