section .bss
    buf resb 16

section .text
    global _start

_start:
    # MISSION: Duplicate
    # INで A を読み、sys_writeで A, A を出力する

    movq $60, %rax
    xorq %rdi, %rdi
    syscall

