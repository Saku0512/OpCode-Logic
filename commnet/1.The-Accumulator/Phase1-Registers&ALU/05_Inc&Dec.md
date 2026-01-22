グランドステージ: 1.The Accumulator

フェーズ: Phase 1 - Registers & ALU

ステージ名: Inc & Dec

ステージ内容: RDI（値A）を読み取り、A+1 と A-1 を順番に出力する（OUT命令を使用）

学習ポイント: INC命令、DEC命令、OUT命令によるストリーム出力

## 解説

このステージでは、INC/DEC命令とOUT命令を使います。

- **RDI**: 入力値（値A）
- **OUT命令**: 値を出力ストリームに送る
- **INC**: 値を1増やす
- **DEC**: 値を1減らす

## 正解コード

```asm
section .text
    global _start

_start:
    ; RAX に RDI の値をコピー
    mov rax, rdi
    
    ; A+1 を計算して出力
    inc rax              ; RAX = A + 1
    out rax              ; A+1 を出力
    
    ; A-1 を計算して出力
    mov rax, rdi         ; RAX = A に戻す
    dec rax              ; RAX = A - 1
    out rax              ; A-1 を出力
    
    ; 終了
    ret
```

## コード解説

1. `mov rax, rdi` - RDIの値をRAXにコピー
2. `inc rax` - RAXを1増やす（A+1）
3. `out rax` - RAXの値を出力ストリームに送る
4. `mov rax, rdi` - 再度RDIの値をRAXにコピー
5. `dec rax` - RAXを1減らす（A-1）
6. `out rax` - RAXの値を出力ストリームに送る

## ポイント

- INC/DECは1増減する専用命令で、ADD/SUBより効率的です
- OUT命令は値を出力キューに追加します（複数の値を順番に出力可能）
- このステージでは戻り値（RAX）ではなく、出力ストリームが評価されます
