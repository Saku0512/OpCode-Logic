グランドステージ: 2.The Stack

フェーズ: Phase 1 - Stack Basics

ステージ名: Swap Two

ステージ内容: 2つの入力(A, B)を読み、出力をB→Aの順にする（sys_write）

学習ポイント: in, push, pop, sys_write, スタックを使った入れ替え

ユーザーに初期状態で表示するコード

```asm
section .bss
    buf resb 16

section .text
    global _start

_start:
    ; MISSION: Swap Two
    ; INで A, B を読み、sys_writeで B, A の順に出力する
    
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
    ; read A
    in rax
    push rax

    ; read B
    in rax
    push rax

    ; output B then A
    mov rsi, buf
    pop rax
    mov [rsi], rax
    inc rsi
    pop rax
    mov [rsi], rax

    ; write(1, buf, 2)
    mov rax, 1
    mov rdi, 1
    mov rsi, buf
    mov rdx, 2
    syscall

    ; exit(0)
    mov rax, 60
    xor rdi, rdi
    syscall
```
