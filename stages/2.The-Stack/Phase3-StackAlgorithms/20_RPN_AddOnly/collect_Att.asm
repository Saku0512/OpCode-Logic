section .bss
    buf resb 16

section .text
    global _start

_start:
.loop:
    in %rax
    cmpq $0, %rax
    je .done
    cmpq $-1, %rax
    je .add
    pushq %rax
    jmp .loop

.add:
    popq %rbx
    popq %rcx
    addq %rbx, %rcx
    pushq %rcx
    jmp .loop

.done:
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

