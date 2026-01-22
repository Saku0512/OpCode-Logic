グランドステージ: 1.The Accumulator

フェーズ: Phase 2 - Flags & Jumps

ステージ名: Unconditional

ステージ内容: 入力を読み、そのまま出力する（jmp命令の練習）

学習ポイント: jmp命令、無条件ジャンプ

ユーザーに初期状態で表示するコード

```asm
section .bss
    buf resb 16

section .text
    global _start

_start:
    ; MISSION: Unconditional
    ; read from stdin, write to stdout using jmp
    
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
    ; MISSION: Unconditional
    ; read from stdin, write to stdout using jmp

    jmp .read_input

.read_input:
    ; read(0, buf, 16)
    mov rax, 0          ; syscall: read
    mov rdi, 0          ; stdin
    mov rsi, buf        ; buffer
    mov rdx, 16         ; size
    syscall

    jmp .write_output

.write_output:
    ; write(1, buf, rax)
    mov rdx, rax        ; number of bytes read
    mov rax, 1          ; syscall: write
    mov rdi, 1          ; stdout
    mov rsi, buf
    syscall
    
    jmp .exit

.exit:
    ; exit(0)
    mov rax, 60
    xor rdi, rdi
    syscall
```
