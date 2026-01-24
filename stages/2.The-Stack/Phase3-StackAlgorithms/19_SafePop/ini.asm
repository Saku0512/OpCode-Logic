section .bss
    buf resb 16

section .text
    global _start

_start:
    ; MISSION: Safe Pop
    ; トークン列を処理して、空popを防ぐ
    ; 最終的な深さを sys_write で出力する
    
    ; exit(0)
    mov rax, 60
    xor rdi, rdi
    syscall

