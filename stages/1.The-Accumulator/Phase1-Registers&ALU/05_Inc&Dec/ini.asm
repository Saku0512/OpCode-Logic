section .bss
    buf resb 16

section .text
    global _start

_start:
    ; MISSION: Inc & Dec
    ; read from stdin, inc even-indexed, dec odd-indexed bytes, write to stdout
    
    ; exit(0)
    mov rax, 60
    xor rdi, rdi
    syscall

