section .bss
    buf resb 16

section .text
    global _start

_start:
    ; read A
    in rax

    ; save A on stack
    push rax

    ; clobber register (prove we can restore)
    xor rax, rax

    ; restore A
    pop rax
    
    ; write(1, buf, 1)
    mov rsi, buf
    mov [rsi], rax
    mov rax, 1
    mov rdi, 1
    mov rsi, buf
    mov rdx, 1
    syscall

    ; exit(0)
    mov rax, 60
    xor rdi, rdi
    syscall

