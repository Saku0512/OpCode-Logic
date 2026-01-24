section .bss
    buf resb 16

section .text
    global _start

_start:
    ; MISSION: Subtraction
    ; read from stdin, subtract 1 from each byte, write to stdout
    
    ; exit(0)
    mov rax, 60
    xor rdi, rdi
    syscall

