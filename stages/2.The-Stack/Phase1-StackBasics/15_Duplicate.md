グランドステージ: 2.The Stack

フェーズ: Phase 1 - Stack Basics

ステージ名: Duplicate

ステージ内容: 入力(A)を読み、同じ値を2回出力する（sys_write）

学習ポイント: in, push, pop, sys_write, 退避して再利用する

ユーザーに初期状態で表示するコード

```asm
section .bss
    buf resb 16

section .text
    global _start

_start:
    ; MISSION: Duplicate
    ; INで A を読み、sys_writeで A, A を出力する
    
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
    in rax
    push rax

    pop rbx
    mov rsi, buf
    mov [rsi], rbx
    inc rsi
    mov [rsi], rbx

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
