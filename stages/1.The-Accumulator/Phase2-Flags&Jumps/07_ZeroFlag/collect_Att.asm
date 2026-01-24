section .bss
    buf resb 16

section .text
    global _start

_start:
    movq $0, %rax
    movq $0, %rdi
    movq $buf, %rsi
    movq $16, %rdx
    syscall

    movq %rax, %rcx
    xorq %r8, %r8

.loop:
    cmpq %rcx, %r8
    jge .done_replace
    movb buf(%r8), %al
    cmpb $0, %al
    jne .next
    movb $0x20, buf(%r8)
.next:
    incq %r8
    jmp .loop

.done_replace:
    movq %rcx, %rdx
    movq $1, %rax
    movq $1, %rdi
    movq $buf, %rsi
    syscall

    movq $60, %rax
    xorq %rdi, %rdi
    syscall

