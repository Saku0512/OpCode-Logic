section .bss
    buf resb 16

section .text
    global _start

_start:
    ; MISSION: RPN (Add Only)
    ; RPNトークン列を処理し、最後にtopをsys_writeで出力する
    
    ; exit(0)
    mov rax, 60
    xor rdi, rdi
    syscall

