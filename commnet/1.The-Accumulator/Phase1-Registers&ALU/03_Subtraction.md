グランドステージ: 1.The Accumulator

フェーズ: Phase 1 - Registers & ALU

ステージ名: Subtraction

ステージ内容: RDI（値A）とRSI（値B）を読み取り、A - B を計算してRAXに返す

学習ポイント: SUB命令、2つのレジスタの値を減算

## 解説

このステージでは、SUB命令を使って2つの値を減算します。

- **RDI**: 第1引数（値A）
- **RSI**: 第2引数（値B）
- **RAX**: 戻り値（A - B）

SUB命令の構文は `sub 宛先, 元` で、`宛先 = 宛先 - 元` を実行します。

## 正解コード

```asm
section .bss
    buf resb 16

section .text
    global _start

_start:
    ; MISSION: Subtraction
    ; read from stdin, subtract 1 from each byte, write to stdout

    ; read(0, buf, 16)
    mov rax, 0          ; syscall: read
    mov rdi, 0          ; stdin
    mov rsi, buf        ; buffer
    mov rdx, 16         ; size
    syscall

    ; subtract 1 from each byte in the buffer
    mov rcx, rax        ; number of bytes read
    xor r8, r8          ; index = 0
.loop:
    cmp r8, rcx
    jge .done_sub
    mov al, byte [buf + r8]
    sub al, 1
    mov byte [buf + r8], al
    inc r8
    jmp .loop

.done_sub:
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
2. `sub rax, rsi` - RAXからRSIの値を減算（RAX = A - B）
3. `ret` - RAXの値（A - B）を戻り値として返す

## ポイント

- SUB命令は第1オペランドから第2オペランドを引きます
- `sub rax, rsi` は `rax = rax - rsi` と同じ意味です
- 結果が負になる場合も正しく処理されます（2の補数表現）
