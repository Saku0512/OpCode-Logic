section .bss
    buf resb 16
    out resb 16

section .text
    global _start

_start:
    movq $0, %rax
    movq $0, %rdi
    movq $buf, %rsi
    movq $16, %rdx
    syscall

    movq %rax, %rcx
    cmpq $2, %rcx
    jl .exit

    movq $buf, %r15
    movq $out, %r14
    xorq %r8, %r8         # index
    movb (%r15), %al      # prev
    xorq %r9, %r9         # out index

.loop:
    incq %r8
    cmpq %rcx, %r8
    jge .done

    movb (%r15,%r8,1), %bl    # cur
    cmpb %al, %bl         # cur - prev
    je .eq
    jg .gt

.lt:
    movb $0x2d, (%r14,%r9,1)  # '-'
    jmp .next
.eq:
    movb $0x3d, (%r14,%r9,1)  # '='
    jmp .next
.gt:
    movb $0x2b, (%r14,%r9,1)  # '+'

.next:
    movb %bl, %al
    incq %r9
    jmp .loop

.done:
    movq %r9, %rdx
    movq $1, %rax
    movq $1, %rdi
    movq %r14, %rsi
    syscall

.exit:
    movq $60, %rax
    xorq %rdi, %rdi
    syscall

