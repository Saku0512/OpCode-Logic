グランドステージ: 2.The Stack

フェーズ: Phase 3 - Stack Algorithms

ステージ名: RPN (Add Only)

ステージ内容: 逆ポーランド記法（RPN）の簡易計算を行う（加算のみ）

学習ポイント: スタック計算, pop順序, ループ, 分岐

入力仕様:
- 0: 終了
- 正の値: push
- -1: ADD（2つpopして足し、結果をpush）

出力仕様:
- 終了時点のスタックtopを sys_write で出力する

ユーザーに初期状態で表示するコード

```asm
section .bss
    buf resb 16

section .text
    global _start

_start:
    ; MISSION: RPN (Add Only)
    ; RPNトークン列を処理し、最後にtopをsys_writeで出力する
    
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

    cmp rax, -1
    jz .add

    ; push number
    push rax
    jmp .loop

.add:
    pop rbx
    pop rcx
    add rcx, rbx
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
