section .bss
    buf resb 16

section .text
    global _start

_start:
    # MISSION: Inc & Dec
    # read from stdin, inc even-indexed, dec odd-indexed bytes, write to stdout

    # exit(0)
    movq $60, %rax
    xorq %rdi, %rdi
    syscall

