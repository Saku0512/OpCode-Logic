section .bss
    buf resb 32
    out resb 32

section .text
    global _start

_start:
    ; MISSION: The Accumulator (BOSS STAGE)
    ; - A-Z -> a-z
    ; - 0-9 -> increment (wrap 9->0)
    ; - others unchanged
    
    ; exit(0)
    mov rax, 60
    xor rdi, rdi
    syscall

