section .bss
    buf resb 16

section .text
    global _start

_start:
    ; MISSION: Unconditional
    ; read from stdin, write to stdout using jmp
    
    ; exit(0)
    mov rax, 60
    xor rdi, rdi
    syscall

