section .bss
    buf resb 16

section .text
    global _start

_start:
    # read(0, buf, 16)
    movq $0, %rax
    movq $0, %rdi
    movq $buf, %rsi
    movq $16, %rdx
    syscall

    # write(1, buf, rax)
    movq %rax, %rdx
    movq $1, %rax
    movq $1, %rdi
    movq $buf, %rsi
    syscall

    # exit(0)
    movq $60, %rax
    xorq %rdi, %rdi
    syscall

