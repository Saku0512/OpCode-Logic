グランドステージ: 1.The Accumulator

フェーズ: Phase 1 - Registers & ALU

ステージ名: Addition

ステージ内容: 入力を読み、各文字を1増やして出力する

学習ポイント: add命令、rdxレジスタの使用、バイト操作

ユーザーに初期状態で表示するコード

```asm
section .bss
    buf resb 16

section .text
    global _start

_start:
    ; MISSION: Addition
    ; read from stdin, add 1 to each byte, write to stdout
    
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
    ; MISSION: Addition
    ; read from stdin, add 1 to each byte, write to stdout

    ; read(0, buf, 16)
    mov rax, 0          ; syscall: read
    mov rdi, 0          ; stdin
    mov rsi, buf        ; buffer
    mov rdx, 16         ; size
    syscall

    ; add 1 to each byte in the buffer
    mov rcx, rax        ; number of bytes read
    xor r8, r8          ; index = 0
.loop:
    cmp r8, rcx
    jge .done_add
    mov al, byte [buf + r8]
    add al, 1
    mov byte [buf + r8], al
    inc r8
    jmp .loop

.done_add:
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
