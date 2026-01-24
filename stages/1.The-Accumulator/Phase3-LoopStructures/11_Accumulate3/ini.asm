section .bss
    buf resb 16

section .text
    global _start

_start:
    ; MISSION: Accumulate 3
    ; read input, sum the first 3 bytes, write 1 byte result
    
    ; exit(0)
    mov rax, 60
    xor rdi, rdi
    syscall

