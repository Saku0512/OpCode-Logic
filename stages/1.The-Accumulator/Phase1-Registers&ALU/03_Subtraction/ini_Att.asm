section .bss
    buf resb 16

section .text
    global _start

_start:
    # MISSION: Subtraction
    # read from stdin, subtract 1 from each byte, write to stdout

    # exit(0)
    movq $60, %rax
    xorq %rdi, %rdi
    syscall

