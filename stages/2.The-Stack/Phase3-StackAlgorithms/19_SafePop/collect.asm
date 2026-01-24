section .bss
    buf resb 16

section .text
    global _start

_start:
    xor rcx, rcx          ; depth = 0

.loop:
    in rax
    cmp rax, 0
    jz .done

    cmp rax, 1
    jz .do_push

    cmp rax, -1
    jz .do_pop

    ; unknown token -> ignore
    jmp .loop

.do_push:
    in rax                ; read value
    push rax
    inc rcx
    jmp .loop

.do_pop:
    cmp rcx, 0
    jz .loop              ; nothing to pop
    pop rbx               ; discard
    dec rcx
    jmp .loop

.done:
    mov rax, rcx
    
    ; write(1, buf, 1)
    mov rsi, buf
    mov [rsi], rax
    mov rax, 1
    mov rdi, 1
    mov rsi, buf
    mov rdx, 1
    syscall

    ; exit(0)
    mov rax, 60
    xor rdi, rdi
    syscall

