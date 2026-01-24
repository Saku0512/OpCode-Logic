section .bss
    buf resb 16

section .text
    global _start

_start:
.loop:
    in rax
    cmp rax, 0
    jz .done
    js .op

    ; positive -> push
    push rax
    jmp .loop

.op:
    cmp rax, -1
    jz .do_add
    cmp rax, -2
    jz .do_sub
    cmp rax, -3
    jz .do_xor
    jmp .loop            ; unknown op -> ignore

.do_add:
    pop rbx              ; b
    pop rcx              ; a
    add rcx, rbx         ; a+b
    push rcx
    jmp .loop

.do_sub:
    pop rbx              ; b
    pop rcx              ; a
    sub rcx, rbx         ; a-b
    push rcx
    jmp .loop

.do_xor:
    pop rbx              ; b
    pop rcx              ; a
    xor rcx, rbx         ; a^b
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

