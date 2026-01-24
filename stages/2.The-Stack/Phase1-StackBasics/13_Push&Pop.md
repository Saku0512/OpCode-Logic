グランドステージ: 2.The Stack

フェーズ: Phase 1 - Stack Basics

ステージ名: Push & Pop

ステージ内容: 入力を読み、スタックに退避してから復元し、その値をsys_writeで出力する

学習ポイント: in, push, pop, sys_write, レジスタ退避

ユーザーに初期状態で表示するコード

```asm
section .bss
    buf resb 16

section .text
    global _start

_start:
    ; MISSION: Push & Pop
    ; INで値を1つ読み、pushで退避し、popで復元してsys_writeで出力する
    
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

    ; save A on stack
    push rax

    ; clobber register (prove we can restore)
    xor rax, rax

    ; restore A
    pop rax
    
    ; write(1, buf, 1)
    mov rsi, buf
    mov [rsi], rax
    mov rax, 1
    mov rdi, 1
    mov rsi, buf
    mov rdx, 1
    syscall

    ; exit(0)
    mov rax, 60
    xor rdi, rdi
    syscall
```
