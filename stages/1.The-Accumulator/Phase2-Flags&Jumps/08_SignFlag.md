グランドステージ: 1.The Accumulator

フェーズ: Phase 2 - Flags & Jumps

ステージ名: Sign Flag

ステージ内容: 入力を読み、最初の文字が負（0x80以上）かどうかで処理を分岐

学習ポイント: SFフラグ、条件付きジャンプ（js/jns）、符号判定

ユーザーに初期状態で表示するコード

```asm
section .bss
    buf resb 16

section .text
    global _start

_start:
    ; MISSION: Sign Flag
    ; read from stdin, check if first byte is negative, write result
    
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
    ; MISSION: Sign Flag
    ; read from stdin, check if first byte is negative, write result

    ; read(0, buf, 16)
    mov rax, 0          ; syscall: read
    mov rdi, 0          ; stdin
    mov rsi, buf        ; buffer
    mov rdx, 16         ; size
    syscall

    ; check if first byte is negative
    mov al, byte [buf]
    test al, al         ; sets SF based on MSB
    js .is_negative
    
    ; positive
    mov byte [buf], '+'
    mov rdx, 1
    jmp .write_result

.is_negative:
    mov byte [buf], '-'
    mov rdx, 1

.write_result:
    ; write(1, buf, rdx)
    mov rax, 1          ; syscall: write
    mov rdi, 1          ; stdout
    mov rsi, buf
    syscall
    
    ; exit(0)
    mov rax, 60
    xor rdi, rdi
    syscall
```
