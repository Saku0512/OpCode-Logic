グランドステージ: 1.The Accumulator

フェーズ: BOSS Stage

ステージ名: The Accumulator

ステージ内容: IN命令を使って入力を読み取り、0が入力されるまで合計を累積し、最終的な合計をRAXに返す

学習ポイント: IN命令、ループ、条件判定、累積処理の組み合わせ

## 解説

このステージはボスステージで、これまで学んだすべての技術を組み合わせます。

- **IN命令**: 入力キューから値を読み取る
- **ループ**: 0が入力されるまで繰り返す
- **累積処理**: 読み取った値を合計に加算

## 正解コード

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

## コード解説

1. `xor rax, rax` - RAXを0で初期化（合計の初期値）
2. `loop_start:` - ループの開始ラベル
3. `in rdi` - 入力キューから値を読み取り、RDIに格納
4. `cmp rdi, 0` - 読み取った値が0かチェック
5. `jz done` - 0なら終了ラベルにジャンプ
6. `add rax, rdi` - 合計に値を加算
7. `jmp loop_start` - ループの先頭に戻る
8. `done:` - 終了ラベル、RAXに合計が入っている

## ポイント

- IN命令は入力キューから値を1つずつ読み取ります
- 0が入力されるまでループを継続します
- 累積処理で合計を計算します
- このステージでは、これまで学んだすべての技術が使われます
