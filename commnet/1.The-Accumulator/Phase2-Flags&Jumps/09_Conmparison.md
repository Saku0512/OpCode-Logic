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
section .text
    global _start

_start:
    ; RDI と RSI を比較
    cmp rdi, rsi
    
    ; RDI > RSI の場合
    jg rdi_greater
    
    ; RDI <= RSI の場合、RSIを返す
    mov rax, rsi
    ret

rdi_greater:
    ; RDI > RSI の場合、RDIを返す
    mov rax, rdi
    ret
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
