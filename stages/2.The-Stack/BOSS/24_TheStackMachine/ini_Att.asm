section .bss
    buf resb 16

section .text
    global _start

_start:
    # MISSION: The Stack Machine (BOSS)
    # トークン列を評価し、最後の結果をsys_writeで出力する

    movq $60, %rax
    xorq %rdi, %rdi
    syscall

