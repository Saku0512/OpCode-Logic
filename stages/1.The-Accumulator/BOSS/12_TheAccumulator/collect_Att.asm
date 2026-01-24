section .bss
    buf resb 32
    out resb 32

section .text
    global _start

_start:
    # read(0, buf, 32)
    movq $0, %rax
    movq $0, %rdi
    movq $buf, %rsi
    movq $32, %rdx
    syscall

    movq %rax, %rcx
    movq $buf, %r15
    movq $out, %r14
    xorq %r8, %r8

.loop:
    cmpq %rcx, %r8
    jge .done

    movb (%r15,%r8,1), %al

    # uppercase?
    cmpb $0x41, %al
    jl .check_digit
    cmpb $0x5a, %al
    jg .check_digit
    addb $0x20, %al
    jmp .write

.check_digit:
    cmpb $0x30, %al
    jl .write
    cmpb $0x39, %al
    jg .write
    incb %al
    cmpb $0x3a, %al
    jne .write
    movb $0x30, %al

.write:
    movb %al, (%r14,%r8,1)
    incq %r8
    jmp .loop

.done:
    # write(1, out, rcx)
    movq %rcx, %rdx
    movq $1, %rax
    movq $1, %rdi
    movq %r14, %rsi
    syscall

    movq $60, %rax
    xorq %rdi, %rdi
    syscall

