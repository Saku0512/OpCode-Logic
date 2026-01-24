section .bss
    buf resb 16

section .text
    global _start

_start:
    in rax
    push rax

    pop rbx
    mov rsi, buf
    mov [rsi], rbx
    inc rsi
    mov [rsi], rbx

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

