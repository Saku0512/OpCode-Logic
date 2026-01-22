グランドステージ: 1.The Accumulator

フェーズ: Phase 1 - Registers & ALU

ステージ名: Addition

ステージ内容: RDI（値A）とRSI（値B）を読み取り、A + B を計算してRAXに返す

学習ポイント: ADD命令、2つのレジスタの値を加算、RSIレジスタの使用

## 解説

このステージでは、ADD命令を使って2つの値を加算します。

- **RDI**: 第1引数（値A）
- **RSI**: 第2引数（値B）
- **RAX**: 戻り値（A + B）

ADD命令の構文は `add 宛先, 元` で、`宛先 = 宛先 + 元` を実行します。

## 正解コード

```asm
section .bss
    buf resb 16

section .text
    global _start

_start:
    ; MISSION: Addition
    ; read from stdin, add 1 to each byte, write to stdout

    ; read(0, buf, 16)
    mov rax, 0          ; syscall: read
    mov rdi, 0          ; stdin
    mov rsi, buf        ; buffer
    mov rdx, 16         ; size
    syscall

    ; add 1 to each byte in the buffer
    mov rcx, rax        ; number of bytes read
    xor r8, r8          ; index = 0
.loop:
    cmp r8, rcx
    jge .done_add
    mov al, byte [buf + r8]
    add al, 1
    mov byte [buf + r8], al
    inc r8
    jmp .loop

.done_add:
    ; write(1, buf, rax)
    mov rdx, rcx        ; number of bytes to write
    mov rax, 1          ; syscall: write
    mov rdi, 1          ; stdout
    mov rsi, buf
    syscall
    
    ; exit(0)
    mov rax, 60
    xor rdi, rdi
    syscall

```

## コード解説

1. `mov rax, rdi` - RDIの値をRAXにコピー（RAX = A）
2. `add rax, rsi` - RSIの値をRAXに加算（RAX = A + B）
3. `ret` - RAXの値（A + B）を戻り値として返す

## ポイント

- ADD命令は第1オペランドに結果を格納します
- `add rax, rsi` は `rax = rax + rsi` と同じ意味です
- オーバーフローは考慮されません（wrapping演算）
