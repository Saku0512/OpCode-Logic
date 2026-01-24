section .bss
    buf resb 16

section .text
    global _start

_start:
.loop:
    in %rax
    cmpq $0, %rax
    je .done
    js .op
    pushq %rax
    jmp .loop

.op:
    cmpq $-1, %rax
    je .do_add
    cmpq $-2, %rax
    je .do_sub
    cmpq $-3, %rax
    je .do_xor
    jmp .loop

.do_add:
    popq %rbx
    popq %rcx
    addq %rbx, %rcx
    pushq %rcx
    jmp .loop

.do_sub:
    popq %rbx
    popq %rcx
    subq %rbx, %rcx
    pushq %rcx
    jmp .loop

.do_xor:
    popq %rbx
    popq %rcx
    xorq %rbx, %rcx
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

