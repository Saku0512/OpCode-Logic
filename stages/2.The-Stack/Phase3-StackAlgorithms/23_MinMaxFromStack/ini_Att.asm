section .bss
    buf resb 16

section .text
    global _start

_start:
    # MISSION: Min & Max From Stack
    # 0が来るまで値を読み、最後に min と max を sys_write する

    movq $60, %rax
    xorq %rdi, %rdi
    syscall

