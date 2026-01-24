section .bss
    buf resb 16

section .text
    global _start

_start:
    ; MISSION: Mov & Call
    ; read from stdin (syscall 0), write to stdout (syscall 1)
    
    ; exit(0)
    mov rax, 60
    xor rdi, rdi
    syscall

