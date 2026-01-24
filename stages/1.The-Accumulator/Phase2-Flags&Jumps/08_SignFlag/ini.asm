section .bss
    buf resb 16

section .text
    global _start

_start:
    ; MISSION: Sign Flag
    ; read from stdin, check if first byte is negative, write result
    
    ; exit(0)
    mov rax, 60
    xor rdi, rdi
    syscall

