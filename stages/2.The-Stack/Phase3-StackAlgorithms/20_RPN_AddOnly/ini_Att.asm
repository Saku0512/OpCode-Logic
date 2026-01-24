section .bss
    buf resb 16

section .text
    global _start

_start:
    # MISSION: RPN (Add Only)
    # RPNトークン列を処理し、最後にtopをsys_writeで出力する

    movq $60, %rax
    xorq %rdi, %rdi
    syscall

