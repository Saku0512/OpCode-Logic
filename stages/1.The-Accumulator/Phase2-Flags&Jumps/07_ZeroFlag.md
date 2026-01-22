グランドステージ: 1.The Accumulator

フェーズ: Phase 2 - Flags & Jumps

ステージ名: Zero Flag

ステージ内容: 入力を読み、ヌル文字（0x00）をスペース（0x20）に変換して出力する

学習ポイント: ZFフラグ、条件付きジャンプ（jz）、フラグの仕組み

ユーザーに初期状態で表示するコード

```asm
section .bss
    buf resb 16

section .text
    global _start

_start:
    ; MISSION: Zero Flag
    ; read from stdin, replace null bytes with spaces, write to stdout
    
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
    ; MISSION: Zero Flag
    ; read from stdin, replace null bytes with spaces, write to stdout

    ; read(0, buf, 16)
    mov rax, 0          ; syscall: read
    mov rdi, 0          ; stdin
    mov rsi, buf        ; buffer
    mov rdx, 16         ; size
    syscall

    ; replace null bytes with spaces
    mov rcx, rax        ; number of bytes read
    xor r8, r8          ; index = 0
.loop:
    cmp r8, rcx
    jge .done_replace
    mov al, byte [buf + r8]
    cmp al, 0           ; check if byte is zero (sets ZF)
    jnz .next_byte
    mov byte [buf + r8], 0x20 ; replace with space
.next_byte:
    inc r8
    jmp .loop

.done_replace:
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
