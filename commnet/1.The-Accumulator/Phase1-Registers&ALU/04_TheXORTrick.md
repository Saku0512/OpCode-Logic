グランドステージ: 1.The Accumulator

フェーズ: Phase 1 - Registers & ALU

ステージ名: The XOR Trick

ステージ内容: RDIの値に関係なく、常に0を返す（XORトリックを使用）

学習ポイント: XOR命令、同じ値同士のXORで0になる性質

## 解説

このステージでは、XOR命令の特殊な性質を利用します。

**XORの重要な性質**: 同じ値同士をXORすると0になります
- `A XOR A = 0`
- これは、レジスタを0にクリアする効率的な方法です

## 正解コード

```asm
section .bss
    buf resb 16

section .text
    global _start

_start:
    ; MISSION: The XOR Trick
    ; read from stdin, XOR each byte with 0x20, write to stdout

    ; read(0, buf, 16)
    mov rax, 0          ; syscall: read
    mov rdi, 0          ; stdin
    mov rsi, buf        ; buffer
    mov rdx, 16         ; size
    syscall

    ; XOR each byte with 0x20
    mov rcx, rax        ; number of bytes read
    xor r8, r8          ; index = 0
.loop:
    cmp r8, rcx
    jge .done_xor
    mov al, byte [buf + r8]
    xor al, 0x20
    mov byte [buf + r8], al
    inc r8
    jmp .loop

.done_xor:
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
2. **`xor r8, r8`** - R8を0で初期化（XORトリックを使用）

### 各バイトと0x20をXORするループ
```asm
.loop:
    cmp r8, rcx          ; インデックスと読み込んだバイト数を比較
    jge .done_xor        ; インデックス >= バイト数なら終了
    mov al, byte [buf + r8]  ; buf[index]の値をALに読み込む
    xor al, 0x20         ; ALと0x20をXOR演算
    mov byte [buf + r8], al  ; 計算結果をbuf[index]に書き戻す
    inc r8               ; インデックスを1増やす
    jmp .loop            ; ループの先頭に戻る
```

1. **`cmp r8, rcx`** - 現在のインデックスと読み込んだバイト数を比較
2. **`jge .done_xor`** - インデックスがバイト数以上ならループ終了
3. **`mov al, byte [buf + r8]`** - バッファの現在位置のバイト値をALに読み込む
4. **`xor al, 0x20`** - ALと0x20（32、2進数で00100000）をXOR演算
   - XORの性質: 同じビット位置で異なる値なら1、同じ値なら0
   - 0x20は6ビット目（0から数えて5番目）だけが1
   - この操作で6ビット目を反転（大文字↔小文字の変換などに使用）
5. **`mov byte [buf + r8], al`** - 計算結果を元の位置に書き戻す
6. **`inc r8`** - インデックスを1増やす
7. **`jmp .loop`** - ループの先頭に戻る

### 結果の出力
```asm
.done_xor:
    mov rdx, rcx        ; number of bytes to write
    mov rax, 1          ; syscall: write
    mov rdi, 1          ; stdout
    mov rsi, buf
    syscall
```
処理済みのバッファを標準出力に書き出します。

## ポイント

- **XOR演算**: ビット単位の排他的論理和
  - `0 XOR 0 = 0`
  - `0 XOR 1 = 1`
  - `1 XOR 0 = 1`
  - `1 XOR 1 = 0`
- **0x20の意味**: 32（10進数）、2進数で`00100000`
  - 6ビット目だけを反転させる
  - ASCII文字では大文字と小文字の変換に使用（例: 'A'=0x41, 'a'=0x61）
- **XORの性質**: 
  - 同じ値同士をXORすると0になる（`A XOR A = 0`）
  - XORは可逆演算（`(A XOR B) XOR B = A`）
- **即値の指定**: `0x20`は16進数表記（`$32`でも可）
