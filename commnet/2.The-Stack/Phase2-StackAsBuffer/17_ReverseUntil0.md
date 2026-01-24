グランドステージ: 2.The Stack

フェーズ: Phase 2 - Stack as Buffer

ステージ名: Reverse Until 0

ステージ内容: 0が来るまで入力をpushし、最後に逆順で全て出力する（sys_write、0は出力しない）

学習ポイント: in, push, pop, sys_write, ループ, カウンタ管理

ユーザーに初期状態で表示するコード

```asm
section .bss
    buf resb 16

section .text
    global _start

_start:
    ; MISSION: Reverse Until 0
    ; INで値を読み続け、0が来たら終了
    ; それまでの値を逆順で sys_write する（0は出力しない）
    
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
    xor rcx, rcx          ; count = 0

.read_loop:
    in rax
    cmp rax, 0
    jz .read_done

    push rax
    inc rcx
    jmp .read_loop

.read_done:
    mov r9, rcx           ; save count
    mov rsi, buf          ; output pointer

.out_loop:
    cmp rcx, 0
    jz .do_write

    pop rax
    mov [rsi], rax
    inc rsi
    dec rcx
    jmp .out_loop

.do_write:
    ; write(1, buf, count)
    mov rax, 1
    mov rdi, 1
    mov rsi, buf
    mov rdx, r9
    syscall

    ; exit(0)
    mov rax, 60
    xor rdi, rdi
    syscall
```
