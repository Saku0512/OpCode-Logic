section .bss
    buf resb 16

section .text
    global _start

_start:
    ; read A
    in rax
    push rax

    ; read B
    in rax
    push rax

    ; output B then A
    mov rsi, buf
    pop rax
    mov [rsi], rax
    inc rsi
    pop rax
    mov [rsi], rax

    ; write(1, buf, 2)
    mov rax, 1
    mov rdi, 1
    mov rsi, buf
    mov rdx, 2
    syscall

    ; exit(0)
    mov rax, 60
    xor rdi, rdi
    syscall

