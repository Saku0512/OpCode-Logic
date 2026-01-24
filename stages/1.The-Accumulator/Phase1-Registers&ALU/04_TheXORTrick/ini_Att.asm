section .bss
    buf resb 16

section .text
    global _start

_start:
    # MISSION: The XOR Trick
    # read from stdin, XOR each byte with 0x20, write to stdout

    # exit(0)
    movq $60, %rax
    xorq %rdi, %rdi
    syscall

