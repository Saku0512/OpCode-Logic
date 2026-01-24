section .bss
    buf resb 16

section .text
    global _start

_start:
    # MISSION: Zero Flag
    # read from stdin, replace null bytes with spaces, write to stdout

    movq $60, %rax
    xorq %rdi, %rdi
    syscall

