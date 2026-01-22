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
section .text
    global _start

_start:
    ; RAX に RDI の値をコピー（合計の初期値）
    mov rax, rdi
    
    ; RSI の値を加算
    add rax, rsi
    
    ; RDX の値を加算
    add rax, rdx
    
    ; 終了（RAX = RDI + RSI + RDX）
    ret
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
