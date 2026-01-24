section .bss
    buf resb 16

section .text
    global _start

_start:
    ; MISSION: Mov & Call
    ; read from stdin (syscall 0), write to stdout (syscall 1)

    ; read(0, buf, 16)
    mov rax, 0          ; syscall: read
    mov rdi, 0          ; stdin
    mov rsi, buf        ; buffer
    mov rdx, 16         ; size
    syscall

    ; write(1, buf, rax)
    mov rdx, rax        ; number of bytes read
    mov rax, 1          ; syscall: write
    mov rdi, 1          ; stdout
    mov rsi, buf
    syscall

    ; exit(0)
    mov rax, 60
    xor rdi, rdi
    syscall

