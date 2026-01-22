グランドステージ: 1.The Accumulator

フェーズ: Phase 1 - Registers & ALU

ステージ名: Subtraction

ステージ内容: RDI（値A）とRSI（値B）を読み取り、A - B を計算してRAXに返す

学習ポイント: SUB命令、2つのレジスタの値を減算

## 解説

このステージでは、SUB命令を使って2つの値を減算します。

- **RDI**: 第1引数（値A）
- **RSI**: 第2引数（値B）
- **RAX**: 戻り値（A - B）

SUB命令の構文は `sub 宛先, 元` で、`宛先 = 宛先 - 元` を実行します。

## 正解コード

```asm
section .bss
    buf resb 16

section .text
    global _start

_start:
    ; MISSION: Subtraction
    ; read from stdin, subtract 1 from each byte, write to stdout

    ; read(0, buf, 16)
    mov rax, 0          ; syscall: read
    mov rdi, 0          ; stdin
    mov rsi, buf        ; buffer
    mov rdx, 16         ; size
    syscall

    ; subtract 1 from each byte in the buffer
    mov rcx, rax        ; number of bytes read
    xor r8, r8          ; index = 0
.loop:
    cmp r8, rcx
    jge .done_sub
    mov al, byte [buf + r8]
    sub al, 1
    mov byte [buf + r8], al
    inc r8
    jmp .loop

.done_sub:
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

### 入力の読み込み
```asm
mov rax, 0          ; syscall: read
mov rdi, 0          ; stdin
mov rsi, buf        ; buffer
mov rdx, 16         ; size
syscall
```
標準入力から最大16バイトを読み込み、`buf`に格納します。

### ループ処理の準備
```asm
mov rcx, rax        ; number of bytes read
xor r8, r8          ; index = 0
```
1. **`mov rcx, rax`** - 読み込んだバイト数をRCXに保存
2. **`xor r8, r8`** - R8を0で初期化（配列のインデックス）

### 各バイトから1を減算するループ
```asm
.loop:
    cmp r8, rcx          ; インデックスと読み込んだバイト数を比較
    jge .done_sub        ; インデックス >= バイト数なら終了
    mov al, byte [buf + r8]  ; buf[index]の値をALに読み込む
    sub al, 1            ; ALから1を減算
    mov byte [buf + r8], al  ; 計算結果をbuf[index]に書き戻す
    inc r8               ; インデックスを1増やす
    jmp .loop            ; ループの先頭に戻る
```

1. **`cmp r8, rcx`** - 現在のインデックスと読み込んだバイト数を比較
2. **`jge .done_sub`** - インデックスがバイト数以上ならループ終了
3. **`mov al, byte [buf + r8]`** - バッファの現在位置のバイト値をALに読み込む
4. **`sub al, 1`** - ALレジスタの値から1を減算
5. **`mov byte [buf + r8], al`** - 計算結果を元の位置に書き戻す
6. **`inc r8`** - インデックスを1増やす
7. **`jmp .loop`** - ループの先頭に戻る

### 結果の出力
```asm
.done_sub:
    mov rdx, rcx        ; number of bytes to write
    mov rax, 1          ; syscall: write
    mov rdi, 1          ; stdout
    mov rsi, buf
    syscall
```
処理済みのバッファを標準出力に書き出します。

## ポイント

- **SUB命令**: 値から1を減算（DEC命令でも可能だが、SUBは任意の値から減算可能）
- **符号付き演算**: 結果が負になる場合も2の補数表現で正しく処理される
- **ループ構造**: 02_Additionと同様の構造で、ADDの代わりにSUBを使用
- **バイト単位処理**: 各バイトを個別に処理するため、ループが必要
