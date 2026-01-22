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
section .text
    global _start

_start:
    ; RAX に RDI の値をコピー
    mov rax, rdi
    
    ; RAX と RAX をXOR（常に0になる）
    xor rax, rax
    
    ; 終了（RAX = 0）
    ret
```

## コード解説

1. `mov rax, rdi` - RDIの値をRAXにコピー
2. `xor rax, rax` - RAXとRAXをXOR（結果は常に0）
3. `ret` - 0を戻り値として返す

## ポイント

- `xor rax, rax` は `mov rax, 0` と同じ結果ですが、より効率的です
- これはx86アーキテクチャでよく使われる最適化テクニックです
- XORはビット単位の排他的論理和演算です
