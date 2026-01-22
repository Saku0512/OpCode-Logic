グランドステージ: 1.The Accumulator

フェーズ: Phase 2 - Flags & Jumps

ステージ名: Zero Flag

ステージ内容: RDI（値A）を読み取り、Aが0なら1を、そうでなければ0を返す

学習ポイント: Zero Flag（ZF）、条件付きジャンプ（JZ/JNZ）、CMP命令

## 解説

このステージでは、Zero Flag（ZF）と条件付きジャンプを使います。

- **Zero Flag (ZF)**: 演算結果が0のときにセットされるフラグ
- **JZ**: Zero Flagがセットされているときにジャンプ
- **JNZ**: Zero Flagがセットされていないときにジャンプ
- **CMP命令**: 2つの値を比較してフラグを設定（実際には減算を行い、結果を捨てる）

## 正解コード

```asm
section .bss
    buf resb 16

section .text
    global _start

_start:
    ; MISSION: Zero Flag
    ; read from stdin, replace null bytes with spaces, write to stdout

    ; read(0, buf, 16)
    mov rax, 0          ; syscall: read
    mov rdi, 0          ; stdin
    mov rsi, buf        ; buffer
    mov rdx, 16         ; size
    syscall

    ; replace null bytes with spaces
    mov rcx, rax        ; number of bytes read
    xor r8, r8          ; index = 0
.loop:
    cmp r8, rcx
    jge .done_replace
    mov al, byte [buf + r8]
    cmp al, 0           ; check if byte is zero (sets ZF)
    jnz .next_byte
    mov byte [buf + r8], 0x20 ; replace with space
.next_byte:
    inc r8
    jmp .loop

.done_replace:
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

1. `cmp rdi, 0` - RDIと0を比較（RDI - 0を計算してフラグを設定）
2. `jz zero_label` - Zero Flagがセットされていれば（RDI == 0）zero_labelにジャンプ
3. `mov rax, 0` - RDI != 0 の場合、0を返す
4. `mov rax, 1` - RDI == 0 の場合、1を返す

## ポイント

- CMP命令は値を変更せず、フラグだけを設定します
- JZは「Jump if Zero」の略で、ZFがセットされているときにジャンプします
- JNZは「Jump if Not Zero」の略で、ZFがセットされていないときにジャンプします
