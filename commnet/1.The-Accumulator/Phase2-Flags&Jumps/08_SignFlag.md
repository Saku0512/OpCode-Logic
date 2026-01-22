グランドステージ: 1.The Accumulator

フェーズ: Phase 2 - Flags & Jumps

ステージ名: Sign Flag

ステージ内容: RDI（値A）を読み取り、A < 0 なら1を、そうでなければ0を返す

学習ポイント: Sign Flag（SF）、条件付きジャンプ（JS）、符号判定

## 解説

このステージでは、Sign Flag（SF）と条件付きジャンプを使います。

- **Sign Flag (SF)**: 演算結果の最上位ビット（符号ビット）が1のときにセットされるフラグ
- **JS**: Sign Flagがセットされているときにジャンプ（負の値）
- **CMP命令**: 減算を行い、結果の符号でSFを設定

## 正解コード

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

### 最初のバイトの符号チェック
```asm
mov al, byte [buf]  ; 最初のバイトをALに読み込む
test al, al         ; ALとALをAND演算（フラグを設定）
js .is_negative     ; Sign Flagがセットされていれば（負の値）.is_negativeにジャンプ
```

1. **`mov al, byte [buf]`** - バッファの最初のバイト（インデックス0）をALに読み込む
2. **`test al, al`** - ALとALをAND演算してフラグを設定
   - `test`命令はAND演算を行い、結果を捨ててフラグだけを設定
   - `al & al`は`al`と同じなので、ALの値に基づいてフラグが設定される
   - ALの最上位ビット（符号ビット）が1ならSign Flag (SF) がセットされる
3. **`js .is_negative`** - Sign Flagがセットされていれば（AL < 0）.is_negativeにジャンプ
   - JSは"Jump if Sign"の略
   - 符号付き整数では、最上位ビットが1の値は負の数として解釈される

### 正の値の場合
```asm
; positive
mov byte [buf], '+'  ; バッファに'+'を書き込む
mov rdx, 1           ; 出力するバイト数（1バイト）
jmp .write_result    ; .write_resultにジャンプ
```

4. **`mov byte [buf], '+'`** - バッファの最初の位置に'+'文字（0x2B）を書き込む
5. **`mov rdx, 1`** - 出力するバイト数を1に設定
6. **`jmp .write_result`** - 出力処理にジャンプ

### 負の値の場合
```asm
.is_negative:
    mov byte [buf], '-'  ; バッファに'-'を書き込む
    mov rdx, 1           ; 出力するバイト数（1バイト）
```

7. **`.is_negative:`** - 負の値の場合のラベル
8. **`mov byte [buf], '-'`** - バッファの最初の位置に'-'文字（0x2D）を書き込む
9. **`mov rdx, 1`** - 出力するバイト数を1に設定

### 結果の出力
```asm
.write_result:
    ; write(1, buf, rdx)
    mov rax, 1          ; syscall: write
    mov rdi, 1          ; stdout
    mov rsi, buf
    syscall
```
10. **`.write_result:`** - 出力処理のラベル
11. **システムコールwrite** - バッファの内容（'+'または'-'）を標準出力に書き出し

## ポイント

- **TEST命令**: AND演算を行い、結果を捨ててフラグだけを設定
  - `test al, al`は`al & al`を計算し、ALの値に基づいてフラグを設定
  - 最上位ビットが1ならSFがセットされる
- **JS命令**: "Jump if Sign"の略
  - Sign Flagがセットされている（結果が負）ときにジャンプ
  - 符号付き整数では、最上位ビットが1の値は負の数として解釈される
- **符号判定**: 
  - バイト値の範囲は0x00-0xFF
  - 符号付きとして解釈すると、0x80-0xFFは負の値（-128～-1）
  - 0x00-0x7Fは正の値（0～127）
- **条件分岐**: JSで符号を判定し、'+'または'-'を出力
