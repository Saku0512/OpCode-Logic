section .bss
    buf resb 16

section .text
    global _start

_start:
    # MISSION: Addition
    # read from stdin, add 1 to each byte, write to stdout

    # exit(0)
    movq $60, %rax
    xorq %rdi, %rdi
    syscall

