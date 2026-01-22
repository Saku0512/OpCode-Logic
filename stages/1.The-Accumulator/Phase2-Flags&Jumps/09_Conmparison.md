グランドステージ: 1.The Accumulator

フェーズ: Phase 2 - Flags & Jumps

ステージ名: Comparison

ステージ内容: 入力を読み、各文字を前の文字と比較して、増加/減少/同じ をマーク

学習ポイント: cmp命令、複数の条件判定、比較フラグ

ユーザーに初期状態で表示するコード

```asm
section .bss
    buf resb 16
    out resb 16

section .text
    global _start

_start:
    ; MISSION: Comparison
    ; read from stdin, compare consecutive bytes, output +/=/- markers
    
    ; exit(0)
    mov rax, 60
    xor rdi, rdi
    syscall
```

正解コード(あくまでも模範解答。ユーザーが出したコードが動けば正解とする)

```asm
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

    ; compare consecutive bytes
    mov rcx, rax        ; number of bytes read
    xor r8, r8          ; index = 0
    mov al, byte [buf]  ; prev = first byte
    xor r9, r9          ; output index = 0

.loop:
    cmp r8, rcx
    jge .done_compare
    inc r8
    cmp r8, rcx
    jge .done_compare
    
    mov bl, byte [buf + r8] ; current = buf[index]
    cmp bl, al
    je .is_equal
    jg .is_greater
    
.is_less:
    mov byte [out + r9], '-'
    jmp .next_iter
    
.is_equal:
    mov byte [out + r9], '='
    jmp .next_iter
    
.is_greater:
    mov byte [out + r9], '+'
    
.next_iter:
    mov al, bl          ; prev = current
    inc r9
    jmp .loop

.done_compare:
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
