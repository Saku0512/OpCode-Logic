section .bss
    buf resb 16

section .text
    global _start

_start:
    ; MISSION: Sort 3
    ; INで3値を読み、昇順にして sys_write する
    
    ; exit(0)
    mov rax, 60
    xor rdi, rdi
    syscall

