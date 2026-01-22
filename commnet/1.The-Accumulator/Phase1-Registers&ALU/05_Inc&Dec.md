グランドステージ: 1.The Accumulator

フェーズ: Phase 1 - Registers & ALU

ステージ名: Inc & Dec

ステージ内容: RDI（値A）を読み取り、A+1 と A-1 を順番に出力する（OUT命令を使用）

学習ポイント: INC命令、DEC命令、OUT命令によるストリーム出力

## 解説

このステージでは、INC/DEC命令とOUT命令を使います。

- **RDI**: 入力値（値A）
- **OUT命令**: 値を出力ストリームに送る
- **INC**: 値を1増やす
- **DEC**: 値を1減らす

## 正解コード

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

## コード解説

1. `mov rax, rdi` - RDIの値をRAXにコピー
2. `inc rax` - RAXを1増やす（A+1）
3. `out rax` - RAXの値を出力ストリームに送る
4. `mov rax, rdi` - 再度RDIの値をRAXにコピー
5. `dec rax` - RAXを1減らす（A-1）
6. `out rax` - RAXの値を出力ストリームに送る

## ポイント

- INC/DECは1増減する専用命令で、ADD/SUBより効率的です
- OUT命令は値を出力キューに追加します（複数の値を順番に出力可能）
- このステージでは戻り値（RAX）ではなく、出力ストリームが評価されます
