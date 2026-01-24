section .bss
    buf resb 16

section .text
    global _start

_start:
    ; MISSION: The XOR Trick
    ; read from stdin, XOR each byte with 0x20, write to stdout

    ; read(0, buf, 16)
    mov rax, 0          ; syscall: read
    mov rdi, 0          ; stdin
    mov rsi, buf        ; buffer
    mov rdx, 16         ; size
    syscall

    ; XOR each byte with 0x20
    mov rcx, rax        ; number of bytes read
    xor r8, r8          ; index = 0
.loop:
    cmp r8, rcx
    jge .done_xor
    mov al, byte [buf + r8]
    xor al, 0x20
    mov byte [buf + r8], al
    inc r8
    jmp .loop

.done_xor:
    ; write(1, buf, rcx)
    mov rdx, rcx        ; number of bytes to write
    mov rax, 1          ; syscall: write
    mov rdi, 1          ; stdout
    mov rsi, buf
    syscall

    ; exit(0)
    mov rax, 60
    xor rdi, rdi
    syscall

