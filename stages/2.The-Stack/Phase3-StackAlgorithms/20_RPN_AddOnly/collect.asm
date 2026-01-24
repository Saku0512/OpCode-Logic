section .bss
    buf resb 16

section .text
    global _start

_start:
.loop:
    in rax
    cmp rax, 0
    jz .done

    cmp rax, -1
    jz .add

    ; push number
    push rax
    jmp .loop

.add:
    pop rbx
    pop rcx
    add rcx, rbx
    push rcx
    jmp .loop

.done:
    pop rax
    
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

