section .bss
    buf resb 16

section .text
    global _start

_start:
    ; MISSION: Countdown
    ; read a digit, count down from it to '0'
    
    ; exit(0)
    mov rax, 60
    xor rdi, rdi
    syscall

