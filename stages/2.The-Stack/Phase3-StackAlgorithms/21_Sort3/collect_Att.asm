section .bss
    buf resb 16

section .text
    global _start

_start:
    in %rax
    in %rbx
    in %rcx

    # if A > B then swap(A,B)
    cmpq %rbx, %rax
    js .ab_ok
    je .ab_ok
    pushq %rax
    movq %rbx, %rax
    popq %rbx
.ab_ok:

    # if B > C then swap(B,C)
    cmpq %rcx, %rbx
    js .bc_ok
    je .bc_ok
    pushq %rbx
    movq %rcx, %rbx
    popq %rcx
.bc_ok:

    # again: if A > B then swap(A,B)
    cmpq %rbx, %rax
    js .ab2_ok
    je .ab2_ok
    pushq %rax
    movq %rbx, %rax
    popq %rbx
.ab2_ok:

    movq $buf, %rsi
    movq %rax, (%rsi)
    incq %rsi
    movq %rbx, (%rsi)
    incq %rsi
    movq %rcx, (%rsi)

    movq $1, %rax
    movq $1, %rdi
    movq $buf, %rsi
    movq $3, %rdx
    syscall

    movq $60, %rax
    xorq %rdi, %rdi
    syscall

