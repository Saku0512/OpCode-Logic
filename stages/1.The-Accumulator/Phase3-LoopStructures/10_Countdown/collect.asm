section .bss
    buf resb 16

section .text
    global _start

_start:
    ; MISSION: Countdown
    ; read a digit, count down from it to '0'

    ; read(0, buf, 1)
    mov rax, 0          ; syscall: read
    mov rdi, 0          ; stdin
    mov rsi, buf        ; buffer
    mov rdx, 1          ; size
    syscall

    ; convert ASCII digit to number (0..9)
    mov al, byte [buf]
    sub al, '0'

    ; rcx = n + 1 (to include 0)
    xor rcx, rcx
    mov cl, al
    inc rcx

    xor r8, r8          ; output index

.loop:
    ; current digit = (rcx - 1)
    mov al, cl
    dec al
    add al, '0'
    mov byte [buf + r8], al
    inc r8
    loop .loop

    ; write(1, buf, r8)
    mov rdx, r8
    mov rax, 1
    mov rdi, 1
    mov rsi, buf
    syscall

    ; exit(0)
    mov rax, 60
    xor rdi, rdi
    syscall

