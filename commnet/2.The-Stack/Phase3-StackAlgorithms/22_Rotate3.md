グランドステージ: 2.The Stack

フェーズ: Phase 3 - Stack Algorithms

ステージ名: Rotate 3

ステージ内容: 3つの入力(A, B, C)を読み、B→C→A の順に出力する

学習ポイント: push/popでの並べ替え, 一時退避レジスタ, sys_write

出力仕様:
- OUTで B, C, A の順に出力する

ユーザーに初期状態で表示するコード

```asm
section .bss
    buf resb 16

section .text
    global _start

_start:
    ; MISSION: Rotate 3
    ; INで A,B,C を読み、OUTで B,C,A を出力する
    
    ; exit(0)
    mov rax, 60
    xor rdi, rdi
    syscall
```

正解コード(あくまでも模範解答。ユーザーが出したコードが動けば正解とする)

```asm
section .bss
    buf resb 16

section .text
    global _start

_start:
    ; push A,B,C
    in rax
    push rax
    in rax
    push rax
    in rax
    push rax

    ; pop C to temp
    pop r8

    ; write B, C, A into buf
    mov rsi, buf
    pop rax              ; B
    mov [rsi], rax
    inc rsi
    mov rax, r8          ; C
    mov [rsi], rax
    inc rsi
    pop rax              ; A
    mov [rsi], rax

    ; write(1, buf, 3)
    mov rax, 1
    mov rdi, 1
    mov rsi, buf
    mov rdx, 3
    syscall

    ; exit(0)
    mov rax, 60
    xor rdi, rdi
    syscall
```
