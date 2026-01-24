グランドステージ: 2.The Stack

フェーズ: BOSS Stage

ステージ名: The Stack Machine

ステージ内容: スタックを使ってトークン列を評価する簡易RPN計算機を作る（総合問題）

学習ポイント: in, push/pop, cmp/test, 条件分岐, ループ, 演算の順序（SUB）, sys_write

入力仕様:
- 0: 終了
- 正の値: push
- 負の値: 命令
  - -1: ADD（a + b）
  - -2: SUB（a - b） ※順序注意（先にpopした方が b）
  - -3: XOR（a ^ b）

出力仕様:
- 終了時点のスタックtopを sys_write で出力する

ユーザーに初期状態で表示するコード

```asm
section .bss
    buf resb 16

section .text
    global _start

_start:
    ; MISSION: The Stack Machine (BOSS)
    ; トークン列を評価し、最後の結果をsys_writeで出力する
    
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
.loop:
    in rax
    cmp rax, 0
    jz .done
    js .op

    ; positive -> push
    push rax
    jmp .loop

.op:
    cmp rax, -1
    jz .do_add
    cmp rax, -2
    jz .do_sub
    cmp rax, -3
    jz .do_xor
    jmp .loop            ; unknown op -> ignore

.do_add:
    pop rbx              ; b
    pop rcx              ; a
    add rcx, rbx         ; a+b
    push rcx
    jmp .loop

.do_sub:
    pop rbx              ; b
    pop rcx              ; a
    sub rcx, rbx         ; a-b
    push rcx
    jmp .loop

.do_xor:
    pop rbx              ; b
    pop rcx              ; a
    xor rcx, rbx         ; a^b
    push rcx
    jmp .loop

.done:
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
