section .bss
    buf resb 16

section .text
    global _start

_start:
    ; MISSION: Min & Max From Stack
    ; 0が来るまで値を読み、最後に min と max を sys_write する
    
    ; exit(0)
    mov rax, 60
    xor rdi, rdi
    syscall

