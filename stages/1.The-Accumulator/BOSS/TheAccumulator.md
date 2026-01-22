グランドステージ: 1.The Accumulator

フェーズ: BOSS Stage

ステージ名: The Accumulator

ステージ内容: 複合チャレンジ - すべての学習内容を組み合わせた総合問題

学習ポイント: mov, add, sub, xor, inc, dec, jmp, cmp, test, 条件ジャンプ、ループ

ユーザーに初期状態で表示するコード

```asm
section .bss
    buf resb 32
    out resb 32

section .text
    global _start

_start:
    ; MISSION: The Accumulator (BOSS STAGE)
    ; Challenge: Combine all learned skills
    ; - Read input
    ; - Process with various operations
    ; - Output result
    
    ; exit(0)
    mov rax, 60
    xor rdi, rdi
    syscall
```

正解コード(あくまでも模範解答。ユーザーが出したコードが動けば正解とする)

```asm
section .bss
    buf resb 32
    out resb 32

section .text
    global _start

_start:
    ; MISSION: The Accumulator (BOSS STAGE)
    ; Challenge: Combine all learned skills
    ; Read input, process each byte:
    ; - For uppercase letters: convert to lowercase
    ; - For digits: increment
    ; - For others: pass through
    ; Output result

    ; read(0, buf, 32)
    mov rax, 0          ; syscall: read
    mov rdi, 0          ; stdin
    mov rsi, buf        ; buffer
    mov rdx, 32         ; size
    syscall

    ; process each byte
    mov rcx, rax        ; number of bytes read
    xor r8, r8          ; read index = 0
    xor r9, r9          ; write index = 0

.loop:
    cmp r8, rcx
    jge .done_process
    
    mov al, byte [buf + r8]
    
    ; check if uppercase letter (A-Z: 0x41-0x5A)
    cmp al, 'A'
    jl .check_digit
    cmp al, 'Z'
    jg .check_digit
    
    ; convert to lowercase (add 0x20)
    add al, 0x20
    jmp .write_byte
    
.check_digit:
    ; check if digit (0-9: 0x30-0x39)
    cmp al, '0'
    jl .pass_through
    cmp al, '9'
    jg .pass_through
    
    ; increment digit
    inc al
    cmp al, '9' + 1
    jne .write_byte
    mov al, '0'         ; wrap 9 to 0
    jmp .write_byte
    
.pass_through:
    ; keep as is
    
.write_byte:
    mov byte [out + r9], al
    inc r9
    inc r8
    jmp .loop

.done_process:
    ; write(1, out, r9)
    mov rdx, r9         ; number of bytes to write
    mov rax, 1          ; syscall: write
    mov rdi, 1          ; stdout
    mov rsi, out
    syscall
    
    ; exit(0)
    mov rax, 60
    xor rdi, rdi
    syscall
```
