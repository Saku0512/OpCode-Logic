section .bss
    buf resb 16

section .text
    global _start

_start:
    ; MISSION: Reverse Until 0
    ; INで値を読み続け、0が来たら終了
    ; それまでの値を逆順で sys_write する（0は出力しない）
    
    ; exit(0)
    mov rax, 60
    xor rdi, rdi
    syscall

