section .bss
    buf resb 16

section .text
    global _start

_start:
    ; push A,B,C
    in rax
    push rax
    in rax
    push rax
    in rax
    push rax

    ; pop C to temp
    pop r8

    ; write B, C, A into buf
    mov rsi, buf
    pop rax              ; B
    mov [rsi], rax
    inc rsi
    mov rax, r8          ; C
    mov [rsi], rax
    inc rsi
    pop rax              ; A
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

