section .bss
    buf resb 16

section .text
    global _start

_start:
    # MISSION: Sum From Stack
    # INで 0 が来るまで読み、全てをpush
    # その後 pop しながら合計し、sys_writeで出力する（0は含めない）

    movq $60, %rax
    xorq %rdi, %rdi
    syscall

