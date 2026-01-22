グランドステージ: 1.The Accumulator

フェーズ: Phase 1 - Registers & ALU

ステージ名: Inc & Dec

ステージ内容: 入力を読み、各文字の偶数番目をinc、奇数番目をdecして出力する

学習ポイント: inc/dec命令、インデックス操作、条件分岐

ユーザーに初期状態で表示するコード

```asm
section .bss
    buf resb 16

section .text
    global _start

_start:
    ; MISSION: Inc & Dec
    ; read from stdin, inc even-indexed, dec odd-indexed bytes, write to stdout
    
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
    ; MISSION: Inc & Dec
    ; read from stdin, inc even-indexed, dec odd-indexed bytes, write to stdout

    ; read(0, buf, 16)
    mov rax, 0          ; syscall: read
    mov rdi, 0          ; stdin
    mov rsi, buf        ; buffer
    mov rdx, 16         ; size
    syscall

    ; inc/dec based on index
    mov rcx, rax        ; number of bytes read
    xor r8, r8          ; index = 0
.loop:
    cmp r8, rcx
    jge .done_inc_dec
    mov al, byte [buf + r8]
    test r8, 1          ; test if index is odd
    jnz .do_dec
.do_inc:
    inc al
    jmp .next
.do_dec:
    dec al
.next:
    mov byte [buf + r8], al
    inc r8
    jmp .loop

.done_inc_dec:
    ; write(1, buf, rax)
    mov rdx, rcx        ; number of bytes to write
    mov rax, 1          ; syscall: write
    mov rdi, 1          ; stdout
    mov rsi, buf
    syscall
    
    ; exit(0)
    mov rax, 60
    xor rdi, rdi
    syscall
```
