section .bss
    buf resb 16

section .text
    global _start

_start:
    ; MISSION: Zero Flag
    ; read from stdin, replace null bytes with spaces, write to stdout
    
    ; exit(0)
    mov rax, 60
    xor rdi, rdi
    syscall

