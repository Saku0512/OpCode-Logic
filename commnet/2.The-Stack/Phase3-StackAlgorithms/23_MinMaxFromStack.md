グランドステージ: 2.The Stack

フェーズ: Phase 3 - Stack Algorithms

ステージ名: Min & Max From Stack

ステージ内容: 0が来るまで入力をpushし、popしながら最小値と最大値を求めて出力する

学習ポイント: スタックでのバッファリング, cmp/jsによる大小判定, ループ, sys_write

入力仕様:
- 0: 終了
- それ以外: 対象データ（少なくとも1つは与えられる）

出力仕様:
- OUTで「最小値→最大値」の順に出力する

ユーザーに初期状態で表示するコード

```asm
section .bss
    buf resb 16

section .text
    global _start

_start:
    ; MISSION: Min & Max From Stack
    ; 0が来るまで値を読み、最後に min と max を sys_write する
    
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
    ; init min/max with first value
    pop rbx
    mov r8, rbx           ; min
    mov r9, rbx           ; max
    dec rcx

.loop:
    cmp rcx, 0
    jz .do_write

    pop rbx

    ; if rbx < min then min = rbx
    mov rax, rbx
    sub rax, r8
    js .update_min
    jmp .check_max

.update_min:
    mov r8, rbx

.check_max:
    ; if rbx > max then max = rbx
    mov rax, rbx
    sub rax, r9
    js .next
    jz .next
    mov r9, rbx

.next:
    dec rcx
    jmp .loop

.do_write:
    mov rsi, buf
    mov rax, r8
    mov [rsi], rax
    inc rsi
    mov rax, r9
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
