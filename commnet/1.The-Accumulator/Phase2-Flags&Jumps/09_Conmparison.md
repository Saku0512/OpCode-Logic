グランドステージ: 1.The Accumulator

フェーズ: Phase 2 - Flags & Jumps

ステージ名: Comparison

ステージ内容: RDI（値A）とRSI（値B）を読み取り、大きい方の値を返す

学習ポイント: CMP命令、条件付きジャンプ（JG/JL/JE）、値の比較

## 解説

このステージでは、2つの値を比較して大きい方を返します。

- **CMP命令**: 2つの値を比較してフラグを設定
- **JG**: 第1オペランドが第2オペランドより大きいときにジャンプ（Jump if Greater）
- **JL**: 第1オペランドが第2オペランドより小さいときにジャンプ（Jump if Less）
- **JE**: 2つの値が等しいときにジャンプ（Jump if Equal）

## 正解コード

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

## コード解説

1. `cmp rdi, rsi` - RDIとRSIを比較（RDI - RSIを計算してフラグを設定）
2. `jg rdi_greater` - RDI > RSI なら rdi_greater にジャンプ
3. `mov rax, rsi` - RDI <= RSI の場合、RSIを返す
4. `mov rax, rdi` - RDI > RSI の場合、RDIを返す

## ポイント

- JGは符号付き比較で「より大きい」を判定します
- 等しい場合はどちらを返しても良いので、RSIを返すようにしています
- CMP命令は値を変更せず、フラグだけを設定します
