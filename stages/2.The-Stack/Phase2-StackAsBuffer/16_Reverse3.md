グランドステージ: 2.The Stack

フェーズ: Phase 2 - Stack as Buffer

ステージ名: Reverse 3

ステージ内容: 3つの入力(A, B, C)を読み、逆順(C, B, A)で出力する（sys_write）

学習ポイント: push, pop, sys_write, LIFO（後入れ先出し）

ユーザーに初期状態で表示するコード

```asm
section .bss
    buf resb 16

section .text
    global _start

_start:
    ; MISSION: Reverse 3
    ; INで A, B, C を読み、sys_writeで C, B, A を出力する
    
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
    in rax
    push rax
    in rax
    push rax

    mov rsi, buf
    pop rax
    mov [rsi], rax
    inc rsi
    pop rax
    mov [rsi], rax
    inc rsi
    pop rax
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
