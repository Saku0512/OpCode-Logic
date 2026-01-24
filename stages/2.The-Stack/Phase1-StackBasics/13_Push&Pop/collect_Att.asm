section .bss
    buf resb 16

section .text
    global _start

_start:
    in %rax
    pushq %rax
    xorq %rax, %rax
    popq %rax

    movq $buf, %rsi
    movq %rax, (%rsi)

    movq $1, %rax
    movq $1, %rdi
    movq $buf, %rsi
    movq $1, %rdx
    syscall

    movq $60, %rax
    xorq %rdi, %rdi
    syscall

