グランドステージ: 1.The Accumulator

フェーズ: Phase 3 - Loop Structures

ステージ名: Countdown

ステージ内容: 入力で受け取った1文字（数字）からカウントダウンして出力

学習ポイント: loop命令、rcxレジスタの活用、ループ制御

ユーザーに初期状態で表示するコード

```asm
section .bss
    buf resb 16

section .text
    global _start

_start:
    ; MISSION: Countdown
    ; read a digit, count down from it to 0
    
    ; exit(0)
    mov rax, 60
    xor rdi, rdi
    syscall
```

正解コード(あくまでも模範解答。ユーザーが出したコードが動けば正解とする)

```asm
section .bss
    buf resb 16

section .text
    global _start

_start:
    ; MISSION: Countdown
    ; read a digit, count down from it to 0

    ; read(0, buf, 16)
    mov rax, 0          ; syscall: read
    mov rdi, 0          ; stdin
    mov rsi, buf        ; buffer
    mov rdx, 16         ; size
    syscall

    ; get the digit and convert to number
    mov al, byte [buf]
    sub al, '0'         ; convert ASCII to number
    
    ; setup for countdown
    xor rcx, rcx
    mov cl, al          ; rcx = number
    xor rsi, rsi        ; buffer index = 0

.loop:
    ; write current number
    mov al, cl
    add al, '0'         ; convert back to ASCII
    mov byte [buf + rsi], al
    inc rsi
    
    loop .loop

    ; write(1, buf, rsi)
    mov rdx, rsi        ; number of bytes
    mov rax, 1          ; syscall: write
    mov rdi, 1          ; stdout
    mov rsi, buf
    syscall
    
    ; exit(0)
    mov rax, 60
    xor rdi, rdi
    syscall
```
