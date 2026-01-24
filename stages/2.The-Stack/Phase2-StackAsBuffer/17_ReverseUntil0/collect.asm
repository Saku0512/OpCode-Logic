section .bss
    buf resb 16

section .text
    global _start

_start:
    xor rcx, rcx          ; count = 0

.read_loop:
    in rax
    cmp rax, 0
    jz .read_done

    push rax
    inc rcx
    jmp .read_loop

.read_done:
    mov r9, rcx           ; save count
    mov rsi, buf          ; output pointer

.out_loop:
    cmp rcx, 0
    jz .do_write

    pop rax
    mov [rsi], rax
    inc rsi
    dec rcx
    jmp .out_loop

.do_write:
    ; write(1, buf, count)
    mov rax, 1
    mov rdi, 1
    mov rsi, buf
    mov rdx, r9
    syscall

    ; exit(0)
    mov rax, 60
    xor rdi, rdi
    syscall

