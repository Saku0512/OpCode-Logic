section .bss
    buf resb 16
    out resb 16

section .text
    global _start

_start:
    # MISSION: Comparison
    # read from stdin, compare consecutive bytes, output +/=/- markers

    movq $60, %rax
    xorq %rdi, %rdi
    syscall

