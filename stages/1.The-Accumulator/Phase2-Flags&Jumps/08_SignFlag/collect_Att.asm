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

    # if first byte is negative -> '-', else '+'
    movq $buf, %r15
    movb (%r15), %al
    testb %al, %al
    js .is_negative

    movb $0x2b, (%r15)        # '+'
    jmp .write_result

.is_negative:
    movb $0x2d, (%r15)        # '-'

.write_result:
    movq $1, %rdx
    movq $1, %rax
    movq $1, %rdi
    movq $buf, %rsi
    syscall

    movq $60, %rax
    xorq %rdi, %rdi
    syscall

