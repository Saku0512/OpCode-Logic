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

    movq $buf, %r15
    xorl %eax, %eax
    movb (%r15), %al
    movb 1(%r15), %bl
    addb %bl, %al
    movb 2(%r15), %bl
    addb %bl, %al
    movb %al, (%r15)

    # write(1, buf, 1)
    movq $1, %rdx
    movq $1, %rax
    movq $1, %rdi
    movq $buf, %rsi
    syscall

    movq $60, %rax
    xorq %rdi, %rdi
    syscall

