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
section .bss
    buf resb 16

section .text
    global _start

_start:
    ; MISSION: Mov & Call
    ; read from stdin (syscall 0), write to stdout (syscall 1)

    ; read(0, buf, 16)
    mov rax, 0          ; syscall: read
    mov rdi, 0          ; stdin
    mov rsi, buf        ; buffer
    mov rdx, 16         ; size
    syscall

    ; write(1, buf, rax)
    mov rdx, rax        ; number of bytes read
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

1. `mov rax, rdi` - RDIレジスタの値をRAXレジスタにコピーします
2. `ret` - 関数から戻ります（RAXの値が戻り値として使用されます）

## ポイント

- MOV命令は値をコピーするだけで、元のレジスタの値は変更されません
- RDIとRAXは異なる用途で使われるレジスタです
  - RDI: 関数の第1引数として使用
  - RAX: 関数の戻り値として使用
