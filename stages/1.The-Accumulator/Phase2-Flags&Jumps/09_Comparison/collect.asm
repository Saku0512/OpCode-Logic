section .bss
    buf resb 16
    out resb 16

section .text
    global _start

_start:
    ; MISSION: Comparison
    ; read from stdin, compare consecutive bytes, output +/=/- markers

    ; read(0, buf, 16)
    mov rax, 0          ; syscall: read
    mov rdi, 0          ; stdin
    mov rsi, buf        ; buffer
    mov rdx, 16         ; size
    syscall

    mov rcx, rax        ; number of bytes read
    cmp rcx, 2
    jl .write_empty

    xor r8, r8          ; index = 0
    mov al, byte [buf]  ; prev
    xor r9, r9          ; out index = 0

.loop:
    inc r8
    cmp r8, rcx
    jge .done

    mov bl, byte [buf + r8] ; cur
    cmp bl, al
    je .eq
    jg .gt

.lt:
    mov byte [out + r9], '-'
    jmp .next
.eq:
    mov byte [out + r9], '='
    jmp .next
.gt:
    mov byte [out + r9], '+'

.next:
    mov al, bl
    inc r9
    jmp .loop

.done:
    ; write(1, out, r9)
    mov rdx, r9
    mov rax, 1
    mov rdi, 1
    mov rsi, out
    syscall
    jmp .exit

.write_empty:
    ; write nothing
    jmp .exit

.exit:
    mov rax, 60
    xor rdi, rdi
    syscall

