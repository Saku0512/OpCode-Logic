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
    popq %rbx
    movq %rbx, %r8    # min
    movq %rbx, %r9    # max
    decq %rcx

.loop:
    cmpq $0, %rcx
    je .do_write

    popq %rbx

    # if rbx < min then min = rbx
    movq %rbx, %rax
    subq %r8, %rax
    js .update_min
    jmp .check_max

.update_min:
    movq %rbx, %r8

.check_max:
    # if rbx > max then max = rbx
    movq %rbx, %rax
    subq %r9, %rax
    js .next
    je .next
    movq %rbx, %r9

.next:
    decq %rcx
    jmp .loop

.do_write:
    movq $buf, %rsi
    movq %r8, %rax
    movq %rax, (%rsi)
    incq %rsi
    movq %r9, %rax
    movq %rax, (%rsi)

    movq $1, %rax
    movq $1, %rdi
    movq $buf, %rsi
    movq $2, %rdx
    syscall

    movq $60, %rax
    xorq %rdi, %rdi
    syscall

