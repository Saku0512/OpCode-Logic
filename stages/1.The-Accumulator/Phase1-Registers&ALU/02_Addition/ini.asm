section .bss
    buf resb 16

section .text
    global _start

_start:
    ; MISSION: Addition
    ; read from stdin, add 1 to each byte, write to stdout
    
    ; exit(0)
    mov rax, 60
    xor rdi, rdi
    syscall

