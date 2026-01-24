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
    xor rbx, rbx          ; sum = 0

.sum_loop:
    cmp rcx, 0
    jz .done
    pop rax
    add rbx, rax
    dec rcx
    jmp .sum_loop

.done:
    mov rax, rbx
    
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

