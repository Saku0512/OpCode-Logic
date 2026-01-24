section .bss
    buf resb 16
    out resb 16

section .text
    global _start

_start:
    ; MISSION: Comparison
    ; read from stdin, compare consecutive bytes, output +/=/- markers
    
    ; exit(0)
    mov rax, 60
    xor rdi, rdi
    syscall

