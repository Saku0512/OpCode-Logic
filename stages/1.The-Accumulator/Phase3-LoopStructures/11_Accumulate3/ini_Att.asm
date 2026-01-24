section .bss
    buf resb 16

section .text
    global _start

_start:
    # MISSION: Accumulate 3
    # read input, sum the first 3 bytes (u8), write 1 byte result

    movq $60, %rax
    xorq %rdi, %rdi
    syscall

