グランドステージ: 1.The Accumulator

フェーズ: Phase 1 - Registers & ALU

ステージ名: Mov & Call

ステージ内容: 入力レジスタ RDI の値を読み取り、そのまま RAX に返す

学習ポイント: MOV命令、レジスタ間の値の移動、RDI（入力）とRAX（戻り値）の役割

## 解説

このステージでは、MOV命令を使ってレジスタ間で値を移動させます。

- **RDI**: 入力値が格納されるレジスタ
- **RAX**: 戻り値を格納するレジスタ

MOV命令の基本的な構文は `mov 宛先, 元` です。

## 正解コード

```asm
section .text
    global _start

_start:
    ; RDI の値を RAX に移動
    mov rax, rdi
    
    ; 終了
    ret
```

## コード解説

1. `mov rax, rdi` - RDIレジスタの値をRAXレジスタにコピーします
2. `ret` - 関数から戻ります（RAXの値が戻り値として使用されます）

## ポイント

- MOV命令は値をコピーするだけで、元のレジスタの値は変更されません
- RDIとRAXは異なる用途で使われるレジスタです
  - RDI: 関数の第1引数として使用
  - RAX: 関数の戻り値として使用
