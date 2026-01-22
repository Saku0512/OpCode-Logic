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
section .text
    global _start

_start:
    ; RAX に RDI の値をコピー
    mov rax, rdi
    
    ; RAX から RSI の値を減算 (RAX = RAX - RSI)
    sub rax, rsi
    
    ; 終了
    ret
```

## コード解説

1. `mov rax, rdi` - RDIの値をRAXにコピー（RAX = A）
2. `sub rax, rsi` - RAXからRSIの値を減算（RAX = A - B）
3. `ret` - RAXの値（A - B）を戻り値として返す

## ポイント

- SUB命令は第1オペランドから第2オペランドを引きます
- `sub rax, rsi` は `rax = rax - rsi` と同じ意味です
- 結果が負になる場合も正しく処理されます（2の補数表現）
