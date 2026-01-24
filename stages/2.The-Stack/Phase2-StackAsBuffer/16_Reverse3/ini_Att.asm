section .bss
    buf resb 16

section .text
    global _start

_start:
    # MISSION: Reverse 3
    # INで A, B, C を読み、sys_writeで C, B, A を出力する

    movq $60, %rax
    xorq %rdi, %rdi
    syscall

