section .bss
    buf resb 16

section .text
    global _start

_start:
    in %rax
    pushq %rax

    popq %rbx
    movq $buf, %rsi
    movq %rbx, (%rsi)
    incq %rsi
    movq %rbx, (%rsi)

    movq $1, %rax
    movq $1, %rdi
    movq $buf, %rsi
    movq $2, %rdx
    syscall

    movq $60, %rax
    xorq %rdi, %rdi
    syscall

