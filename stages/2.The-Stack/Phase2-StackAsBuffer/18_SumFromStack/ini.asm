section .bss
    buf resb 16

section .text
    global _start

_start:
    ; MISSION: Sum From Stack
    ; INで 0 が来るまで読み、全てをpush
    ; その後 pop しながら合計し、sys_writeで出力する（0は含めない）
    
    ; exit(0)
    mov rax, 60
    xor rdi, rdi
    syscall

