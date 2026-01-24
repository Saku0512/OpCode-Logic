section .bss
    buf resb 16

section .text
    global _start

_start:
    # MISSION: Swap Two
    # INで A, B を読み、sys_writeで B, A の順に出力する

    movq $60, %rax
    xorq %rdi, %rdi
    syscall

