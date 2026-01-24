section .bss
    buf resb 16

section .text
    global _start

_start:
    # MISSION: Safe Pop
    # トークン列を処理して、空popを防ぐ
    # 最終的な深さを sys_write で出力する

    movq $60, %rax
    xorq %rdi, %rdi
    syscall

