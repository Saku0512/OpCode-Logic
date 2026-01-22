グランドステージ: 1.The Accumulator

フェーズ: Phase 3 - Loop Structures

ステージ名: Countdown

ステージ内容: RDI（値N）を読み取り、N, N-1, N-2, ..., 1 を順番に出力する（ストリーム出力）

学習ポイント: ループ構造、DEC命令、条件付きジャンプ、OUT命令

## 解説

このステージでは、カウントダウンループを実装します。

- **ループ**: 条件が満たされるまで繰り返し実行
- **DEC命令**: 値を1減らす
- **JNZ**: Zero Flagがセットされていないときにジャンプ（ループ継続）

## 正解コード

```asm
section .text
    global _start

_start:
    ; RCX に RDI の値をコピー（カウンタ）
    mov rcx, rdi

loop_start:
    ; 現在の値を出力
    mov rax, rcx
    out rax
    
    ; カウンタを1減らす
    dec rcx
    
    ; カウンタが0でなければループを継続
    cmp rcx, 0
    jnz loop_start
    
    ; 終了
    ret
```

## コード解説

1. `mov rcx, rdi` - RDIの値をRCXにコピー（カウンタとして使用）
2. `loop_start:` - ループの開始ラベル
3. `mov rax, rcx` - 現在のカウンタ値をRAXにコピー
4. `out rax` - 現在の値を出力
5. `dec rcx` - カウンタを1減らす
6. `cmp rcx, 0` - カウンタが0かチェック
7. `jnz loop_start` - カウンタが0でなければループを継続

## ポイント

- ループはカウンタが0になるまで続きます
- DEC命令でカウンタを減らし、CMPとJNZでループ継続を判定します
- 出力は N, N-1, N-2, ..., 1 の順になります
