section .bss
    buf resb 16

section .text
    global _start

_start:
    ; MISSION: Sign Flag
    ; read from stdin, check if first byte is negative, write result

    ; read(0, buf, 16)
    mov rax, 0          ; syscall: read
    mov rdi, 0          ; stdin
    mov rsi, buf        ; buffer
    mov rdx, 16         ; size
    syscall

    ; check if first byte is negative
    mov al, byte [buf]
    test al, al         ; sets SF based on MSB
    js .is_negative

    ; non-negative
    mov byte [buf], '+'
    jmp .write_result

.is_negative:
    mov byte [buf], '-'

.write_result:
    ; write(1, buf, 1)
    mov rdx, 1
    mov rax, 1          ; syscall: write
    mov rdi, 1          ; stdout
    mov rsi, buf
    syscall

    ; exit(0)
    mov rax, 60
    xor rdi, rdi
    syscall

