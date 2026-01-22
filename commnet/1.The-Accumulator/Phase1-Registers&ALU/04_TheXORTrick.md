グランドステージ: 1.The Accumulator

フェーズ: Phase 1 - Registers & ALU

ステージ名: The XOR Trick

ステージ内容: RDIの値に関係なく、常に0を返す（XORトリックを使用）

学習ポイント: XOR命令、同じ値同士のXORで0になる性質

## 解説

このステージでは、XOR命令の特殊な性質を利用します。

**XORの重要な性質**: 同じ値同士をXORすると0になります
- `A XOR A = 0`
- これは、レジスタを0にクリアする効率的な方法です

## 正解コード

```asm
section .bss
    buf resb 16

section .text
    global _start

_start:
    ; MISSION: The XOR Trick
    ; read from stdin, XOR each byte with 0x20, write to stdout

    ; read(0, buf, 16)
    mov rax, 0          ; syscall: read
    mov rdi, 0          ; stdin
    mov rsi, buf        ; buffer
    mov rdx, 16         ; size
    syscall

    ; XOR each byte with 0x20
    mov rcx, rax        ; number of bytes read
    xor r8, r8          ; index = 0
.loop:
    cmp r8, rcx
    jge .done_xor
    mov al, byte [buf + r8]
    xor al, 0x20
    mov byte [buf + r8], al
    inc r8
    jmp .loop

.done_xor:
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

1. `mov rax, rdi` - RDIの値をRAXにコピー
2. `xor rax, rax` - RAXとRAXをXOR（結果は常に0）
3. `ret` - 0を戻り値として返す

## ポイント

- `xor rax, rax` は `mov rax, 0` と同じ結果ですが、より効率的です
- これはx86アーキテクチャでよく使われる最適化テクニックです
- XORはビット単位の排他的論理和演算です
