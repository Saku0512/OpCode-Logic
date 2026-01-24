section .bss
    buf resb 16

section .text
    global _start

_start:
    # MISSION: Countdown
    # read a digit, count down from it to '0'

    movq $60, %rax
    xorq %rdi, %rdi
    syscall

