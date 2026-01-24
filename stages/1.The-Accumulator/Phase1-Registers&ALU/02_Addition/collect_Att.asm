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

    # rcx = bytes read, r8 = index
    movq %rax, %rcx
    xorq %r8, %r8

.loop:
    cmpq %rcx, %r8
    jge .done_add
    movb buf(%r8), %al
    addb $1, %al
    movb %al, buf(%r8)
    incq %r8
    jmp .loop

.done_add:
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

