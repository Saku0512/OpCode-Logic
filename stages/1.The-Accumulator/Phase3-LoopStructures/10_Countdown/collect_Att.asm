section .bss
    buf resb 16

section .text
    global _start

_start:
    # read(0, buf, 1)
    movq $0, %rax
    movq $0, %rdi
    movq $buf, %rsi
    movq $1, %rdx
    syscall

    # n = buf[0] - '0'
    movq $buf, %r15
    movb (%r15), %al
    subb $0x30, %al

    # rcx = n + 1 (include 0)
    xorq %rcx, %rcx
    movb %al, %cl
    incq %rcx

    xorq %r8, %r8

.loop:
    movb %cl, %al
    decb %al
    addb $0x30, %al
    movb %al, buf(%r8)
    incq %r8
    decq %rcx
    jnz .loop

    # write(1, buf, r8)
    movq %r8, %rdx
    movq $1, %rax
    movq $1, %rdi
    movq $buf, %rsi
    syscall

    movq $60, %rax
    xorq %rdi, %rdi
    syscall

