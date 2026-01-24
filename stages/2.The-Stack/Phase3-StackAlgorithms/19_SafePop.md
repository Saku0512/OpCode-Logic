グランドステージ: 2.The Stack

フェーズ: Phase 3 - Stack Algorithms

ステージ名: Safe Pop

ステージ内容: 操作トークン列を処理し、空のスタックからpopしないように制御する

学習ポイント: cmp, jz, js, ループ, 深さカウンタ（stack depth）, push/popの安全運用, sys_write

入力仕様:
- 0: 終了
- 1: 「次の値」を読み、その値をpushする
- -1: 可能ならpopして捨てる（空なら何もしない）

出力仕様:
- 最終的なスタック深さを sys_write で出力する

ユーザーに初期状態で表示するコード

```asm
section .bss
    buf resb 16

section .text
    global _start

_start:
    ; MISSION: Safe Pop
    ; トークン列を処理して、空popを防ぐ
    ; 最終的な深さを sys_write で出力する
    
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
    xor rcx, rcx          ; depth = 0

.loop:
    in rax
    cmp rax, 0
    jz .done

    cmp rax, 1
    jz .do_push

    cmp rax, -1
    jz .do_pop

    ; unknown token -> ignore
    jmp .loop

.do_push:
    in rax                ; read value
    push rax
    inc rcx
    jmp .loop

.do_pop:
    cmp rcx, 0
    jz .loop              ; nothing to pop
    pop rbx               ; discard
    dec rcx
    jmp .loop

.done:
    mov rax, rcx
    
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
