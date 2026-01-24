section .bss
    buf resb 16

section .text
    global _start

_start:
    # MISSION: Sign Flag
    # read from stdin, check if first byte is negative, write result

    movq $60, %rax
    xorq %rdi, %rdi
    syscall

