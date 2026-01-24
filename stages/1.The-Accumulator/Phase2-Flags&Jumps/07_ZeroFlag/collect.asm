section .bss
    buf resb 16

section .text
    global _start

_start:
    ; MISSION: Zero Flag
    ; read from stdin, replace null bytes with spaces, write to stdout

    ; read(0, buf, 16)
    mov rax, 0          ; syscall: read
    mov rdi, 0          ; stdin
    mov rsi, buf        ; buffer
    mov rdx, 16         ; size
    syscall

    ; replace null bytes with spaces
    mov rcx, rax        ; number of bytes read
    xor r8, r8          ; index = 0
.loop:
    cmp r8, rcx
    jge .done_replace
    mov al, byte [buf + r8]
    cmp al, 0           ; sets ZF if al == 0
    jnz .next
    mov byte [buf + r8], 0x20
.next:
    inc r8
    jmp .loop

.done_replace:
    ; write(1, buf, rcx)
    mov rdx, rcx
    mov rax, 1
    mov rdi, 1
    mov rsi, buf
    syscall

    ; exit(0)
    mov rax, 60
    xor rdi, rdi
    syscall

