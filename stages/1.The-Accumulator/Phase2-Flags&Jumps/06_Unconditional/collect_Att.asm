section .bss
    buf resb 16

section .text
    global _start

_start:
    jmp .read_input

.read_input:
    movq $0, %rax
    movq $0, %rdi
    movq $buf, %rsi
    movq $16, %rdx
    syscall
    jmp .write_output

.write_output:
    movq %rax, %rdx
    movq $1, %rax
    movq $1, %rdi
    movq $buf, %rsi
    syscall
    jmp .exit

.exit:
    movq $60, %rax
    xorq %rdi, %rdi
    syscall

