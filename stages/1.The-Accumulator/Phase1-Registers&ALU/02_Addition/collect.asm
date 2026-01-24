section .bss
    buf resb 16

section .text
    global _start

_start:
    ; MISSION: Addition
    ; read from stdin, add 1 to each byte, write to stdout

    ; read(0, buf, 16)
    mov rax, 0          ; syscall: read
    mov rdi, 0          ; stdin
    mov rsi, buf        ; buffer
    mov rdx, 16         ; size
    syscall

    ; add 1 to each byte in the buffer
    mov rcx, rax        ; number of bytes read
    xor r8, r8          ; index = 0
.loop:
    cmp r8, rcx
    jge .done_add
    mov al, byte [buf + r8]
    add al, 1
    mov byte [buf + r8], al
    inc r8
    jmp .loop

.done_add:
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

