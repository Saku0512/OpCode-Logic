section .bss
    buf resb 16

section .text
    global _start

_start:
    xorq %rcx, %rcx

.loop:
    in %rax
    cmpq $0, %rax
    je .done
    cmpq $1, %rax
    je .do_push
    cmpq $-1, %rax
    je .do_pop
    jmp .loop

.do_push:
    in %rax
    pushq %rax
    incq %rcx
    jmp .loop

.do_pop:
    cmpq $0, %rcx
    je .loop
    popq %rbx
    decq %rcx
    jmp .loop

.done:
    movq %rcx, %rax
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

