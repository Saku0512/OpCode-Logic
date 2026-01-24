section .bss
    buf resb 16

section .text
    global _start

_start:
    # MISSION: Unconditional
    # read from stdin, write to stdout using jmp

    # exit(0)
    movq $60, %rax
    xorq %rdi, %rdi
    syscall

