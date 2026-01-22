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
section .text
    global _start

_start:
    ; RDI と 0 を比較（ZFを設定）
    cmp rdi, 0
    
    ; ZFがセットされていれば（RDI == 0）zero_labelにジャンプ
    jz zero_label
    
    ; RDI != 0 の場合
    mov rax, 0          ; 0を返す
    ret

zero_label:
    ; RDI == 0 の場合
    mov rax, 1          ; 1を返す
    ret
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
