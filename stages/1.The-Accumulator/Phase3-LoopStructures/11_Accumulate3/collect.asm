section .bss
    buf resb 16

section .text
    global _start

_start:
    ; MISSION: Accumulate 3
    ; read input, sum the first 3 bytes (u8), write 1 byte result

    ; read(0, buf, 16)
    mov rax, 0
    mov rdi, 0
    mov rsi, buf
    mov rdx, 16
    syscall

    ; sum buf[0] + buf[1] + buf[2] into AL (wrapping u8)
    xor eax, eax
    mov al, byte [buf + 0]
    mov bl, byte [buf + 1]
    add al, bl
    mov bl, byte [buf + 2]
    add al, bl
    mov byte [buf], al

    ; write(1, buf, 1)
    mov rdx, 1
    mov rax, 1
    mov rdi, 1
    mov rsi, buf
    syscall

    ; exit(0)
    mov rax, 60
    xor rdi, rdi
    syscall

