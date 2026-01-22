グランドステージ: 1.The Accumulator

フェーズ: Phase 3 - Loop Structures

ステージ名: Accumulate 3

ステージ内容: 入力を読み、最初の3文字の合計を出力

学習ポイント: 累積ループ、複数の値の加算、ループ制御

ユーザーに初期状態で表示するコード

```asm
section .bss
    buf resb 16
    out resb 16

section .text
    global _start

_start:
    ; MISSION: Accumulate 3
    ; read input, sum the first 3 bytes, write result
    
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
    ; MISSION: Accumulate 3
    ; read input, sum the first 3 bytes, write result

    ; read(0, buf, 16)
    mov rax, 0          ; syscall: read
    mov rdi, 0          ; stdin
    mov rsi, buf        ; buffer
    mov rdx, 16         ; size
    syscall

    ; accumulate first 3 bytes
    xor rax, rax        ; sum = 0
    xor rcx, rcx        ; index = 0
    
.loop:
    cmp rcx, 3
    jge .done_accumulate
    
    mov bl, byte [buf + rcx]
    movzx rbx, bl       ; zero-extend to 64-bit
    add rax, rbx
    inc rcx
    jmp .loop

.done_accumulate:
    ; convert sum to decimal string
    ; for simplicity, assume single digit result
    add al, '0'
    mov byte [out], al
    mov rdx, 1

    ; write(1, out, rdx)
    mov rax, 1          ; syscall: write
    mov rdi, 1          ; stdout
    mov rsi, out
    syscall
    
    ; exit(0)
    mov rax, 60
    xor rdi, rdi
    syscall
```
