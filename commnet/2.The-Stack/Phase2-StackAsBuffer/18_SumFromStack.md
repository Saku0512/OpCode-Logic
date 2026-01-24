グランドステージ: 2.The Stack

フェーズ: Phase 2 - Stack as Buffer

ステージ名: Sum From Stack

ステージ内容: 0が来るまで入力をpushし、最後にpopしながら合計してsys_writeで出力する

学習ポイント: in, push, pop, add, sys_write, ループ, 「読み取りフェーズ」と「計算フェーズ」の分離

ユーザーに初期状態で表示するコード

```asm
section .bss
    buf resb 16

section .text
    global _start

_start:
    ; MISSION: Sum From Stack
    ; INで 0 が来るまで読み、全てをpush
    ; その後 pop しながら合計し、sys_writeで出力する（0は含めない）
    
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
    xor rbx, rbx          ; sum = 0

.sum_loop:
    cmp rcx, 0
    jz .done
    pop rax
    add rbx, rax
    dec rcx
    jmp .sum_loop

.done:
    mov rax, rbx
    
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
