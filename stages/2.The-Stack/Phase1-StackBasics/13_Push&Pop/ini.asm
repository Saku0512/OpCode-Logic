section .bss
    buf resb 16

section .text
    global _start

_start:
    ; MISSION: Push & Pop
    ; INで値を1つ読み、pushで退避し、popで復元してsys_writeで出力する
    
    ; exit(0)
    mov rax, 60
    xor rdi, rdi
    syscall

