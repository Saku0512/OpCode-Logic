グランドステージ: 1.The Accumulator

フェーズ: Phase 2 - Flags & Jumps

ステージ名: Sign Flag

ステージ内容: RDI（値A）を読み取り、A < 0 なら1を、そうでなければ0を返す

学習ポイント: Sign Flag（SF）、条件付きジャンプ（JS）、符号判定

## 解説

このステージでは、Sign Flag（SF）と条件付きジャンプを使います。

- **Sign Flag (SF)**: 演算結果の最上位ビット（符号ビット）が1のときにセットされるフラグ
- **JS**: Sign Flagがセットされているときにジャンプ（負の値）
- **CMP命令**: 減算を行い、結果の符号でSFを設定

## 正解コード

```asm
section .bss
    buf resb 16

section .text
    global _start

_start:
    ; MISSION: Sign Flag
    ; read from stdin, check if first byte is negative, write result

    ; read(0, buf, 16)
    mov rax, 0          ; syscall: read
    mov rdi, 0          ; stdin
    mov rsi, buf        ; buffer
    mov rdx, 16         ; size
    syscall

    ; check if first byte is negative
    mov al, byte [buf]
    test al, al         ; sets SF based on MSB
    js .is_negative
    
    ; positive
    mov byte [buf], '+'
    mov rdx, 1
    jmp .write_result

.is_negative:
    mov byte [buf], '-'
    mov rdx, 1

.write_result:
    ; write(1, buf, rdx)
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
2. `js negative_label` - Sign Flagがセットされていれば（RDI < 0）negative_labelにジャンプ
3. `mov rax, 0` - RDI >= 0 の場合、0を返す
4. `mov rax, 1` - RDI < 0 の場合、1を返す

## ポイント

- JSは「Jump if Sign」の略で、SFがセットされている（結果が負）ときにジャンプします
- 符号付き整数では、最上位ビットが1の値は負の数として解釈されます
- CMP命令は `rdi - 0` を計算し、結果が負ならSFをセットします
