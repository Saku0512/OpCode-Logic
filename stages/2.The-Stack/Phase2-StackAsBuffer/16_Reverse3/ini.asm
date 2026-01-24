section .bss
    buf resb 16

section .text
    global _start

_start:
    ; MISSION: Reverse 3
    ; INで A, B, C を読み、sys_writeで C, B, A を出力する
    
    ; exit(0)
    mov rax, 60
    xor rdi, rdi
    syscall

