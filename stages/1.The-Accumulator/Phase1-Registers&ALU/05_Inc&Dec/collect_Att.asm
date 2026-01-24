section .bss
    buf resb 16

section .text
    global _start

_start:
    # read(0, buf, 16)
    movq $0, %rax
    movq $0, %rdi
    movq $buf, %rsi
    movq $16, %rdx
    syscall

    movq %rax, %rcx
    xorq %r8, %r8

.loop:
    cmpq %rcx, %r8
    jge .done_inc_dec
    movb buf(%r8), %al
    testq $1, %r8
    jnz .do_dec
.do_inc:
    incb %al
    jmp .next
.do_dec:
    decb %al
.next:
    movb %al, buf(%r8)
    incq %r8
    jmp .loop

.done_inc_dec:
    # write(1, buf, rcx)
    movq %rcx, %rdx
    movq $1, %rax
    movq $1, %rdi
    movq $buf, %rsi
    syscall

    # exit(0)
    movq $60, %rax
    xorq %rdi, %rdi
    syscall

