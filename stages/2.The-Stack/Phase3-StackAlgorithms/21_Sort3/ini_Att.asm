section .bss
    buf resb 16

section .text
    global _start

_start:
    # MISSION: Sort 3
    # INで3値を読み、昇順にして sys_write する

    movq $60, %rax
    xorq %rdi, %rdi
    syscall

