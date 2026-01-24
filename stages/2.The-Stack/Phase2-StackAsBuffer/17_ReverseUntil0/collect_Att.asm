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
    movq %rcx, %r9
    movq $buf, %rsi

.out_loop:
    cmpq $0, %rcx
    je .do_write
    popq %rax
    movq %rax, (%rsi)
    incq %rsi
    decq %rcx
    jmp .out_loop

.do_write:
    movq $1, %rax
    movq $1, %rdi
    movq $buf, %rsi
    movq %r9, %rdx
    syscall

    movq $60, %rax
    xorq %rdi, %rdi
    syscall

