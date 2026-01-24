section .bss
    buf resb 32
    out resb 32

section .text
    global _start

_start:
    ; MISSION: The Accumulator (BOSS STAGE)
    ; - A-Z -> a-z
    ; - 0-9 -> increment (wrap 9->0)
    ; - others unchanged

    ; read(0, buf, 32)
    mov rax, 0
    mov rdi, 0
    mov rsi, buf
    mov rdx, 32
    syscall

    mov rcx, rax        ; bytes read
    xor r8, r8          ; index = 0

.loop:
    cmp r8, rcx
    jge .done

    mov al, byte [buf + r8]

    ; uppercase?
    cmp al, 'A'
    jl .check_digit
    cmp al, 'Z'
    jg .check_digit
    add al, 0x20
    jmp .write

.check_digit:
    cmp al, '0'
    jl .write
    cmp al, '9'
    jg .write
    inc al
    cmp al, '9' + 1
    jne .write
    mov al, '0'

.write:
    mov byte [out + r8], al
    inc r8
    jmp .loop

.done:
    ; write(1, out, rcx)
    mov rdx, rcx
    mov rax, 1
    mov rdi, 1
    mov rsi, out
    syscall

    ; exit(0)
    mov rax, 60
    xor rdi, rdi
    syscall

