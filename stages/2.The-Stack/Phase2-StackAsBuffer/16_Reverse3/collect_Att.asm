section .bss
    buf resb 16

section .text
    global _start

_start:
    in %rax
    pushq %rax
    in %rax
    pushq %rax
    in %rax
    pushq %rax

    movq $buf, %rsi
    popq %rax
    movq %rax, (%rsi)
    incq %rsi
    popq %rax
    movq %rax, (%rsi)
    incq %rsi
    popq %rax
    movq %rax, (%rsi)

    movq $1, %rax
    movq $1, %rdi
    movq $buf, %rsi
    movq $3, %rdx
    syscall

    movq $60, %rax
    xorq %rdi, %rdi
    syscall

