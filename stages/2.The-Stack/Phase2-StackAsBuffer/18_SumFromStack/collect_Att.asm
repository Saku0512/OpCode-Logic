section .bss
    buf resb 16

section .text
    global _start

_start:
    xorq %rcx, %rcx

.read_loop:
    in %rax
    cmpq $0, %rax
    je .read_done
    pushq %rax
    incq %rcx
    jmp .read_loop

.read_done:
    xorq %rbx, %rbx

.sum_loop:
    cmpq $0, %rcx
    je .done
    popq %rax
    addq %rax, %rbx
    decq %rcx
    jmp .sum_loop

.done:
    movq %rbx, %rax

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

