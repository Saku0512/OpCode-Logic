グランドステージ: 1.The Accumulator

フェーズ: Phase 3 - Loop Structures

ステージ名: Accumulate 3

ステージ内容: RDI、RSI、RDXの3つの値を読み取り、合計をRAXに返す

学習ポイント: 複数のレジスタの値を加算、累積処理

## 解説

このステージでは、3つのレジスタの値を合計します。

- **RDI**: 第1引数（値A）
- **RSI**: 第2引数（値B）
- **RDX**: 第3引数（値C）
- **RAX**: 戻り値（A + B + C）

## 正解コード

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

## コード解説

1. `mov rax, rdi` - RDIの値をRAXにコピー（合計の初期値 = A）
2. `add rax, rsi` - RSIの値をRAXに加算（RAX = A + B）
3. `add rax, rdx` - RDXの値をRAXに加算（RAX = A + B + C）
4. `ret` - 合計値を返す

## ポイント

- 複数の値を加算する場合は、1つずつ順番に加算します
- ADD命令は累積的に値を加算できます
- この方法はシンプルで効率的です
