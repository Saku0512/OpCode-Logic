section .bss
    buf resb 16

section .text
    global _start

_start:
    ; MISSION: Rotate 3
    ; INで A,B,C を読み、sys_writeで B,C,A を出力する
    
    ; exit(0)
    mov rax, 60
    xor rdi, rdi
    syscall

