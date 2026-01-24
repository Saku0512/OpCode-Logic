section .bss
    buf resb 16

section .text
    global _start

_start:
    # MISSION: Push & Pop
    # INで値を1つ読み、pushで退避し、popで復元してsys_writeで出力する

    movq $60, %rax
    xorq %rdi, %rdi
    syscall

