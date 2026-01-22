グランドステージ: 1.The Accumulator

フェーズ: Phase 3 - Loop Structures

ステージ名: Accumulate 3

ステージ内容: RDI、RSI、RDXの3つの値を読み取り、合計をRAXに返す

学習ポイント: 複数のレジスタの値を加算、累積処理

## 解説

このステージでは、3つのレジスタの値を合計します。

- **RDI**: 第1引数（値A）
- **RSI**: 第2引数（値B）
- **RDX**: 第3引数（値C）
- **RAX**: 戻り値（A + B + C）

## 正解コード

```asm
section .bss
    buf resb 16
    out resb 16

section .text
    global _start

_start:
    ; MISSION: Accumulate 3
    ; read input, sum the first 3 bytes, write result

    ; read(0, buf, 16)
    mov rax, 0          ; syscall: read
    mov rdi, 0          ; stdin
    mov rsi, buf        ; buffer
    mov rdx, 16         ; size
    syscall

    ; accumulate first 3 bytes
    xor rax, rax        ; sum = 0
    xor rcx, rcx        ; index = 0
    
.loop:
    cmp rcx, 3
    jge .done_accumulate
    
    mov bl, byte [buf + rcx]
    movzx rbx, bl       ; zero-extend to 64-bit
    add rax, rbx
    inc rcx
    jmp .loop

.done_accumulate:
    ; convert sum to decimal string
    ; for simplicity, assume single digit result
    add al, '0'
    mov byte [out], al
    mov rdx, 1

    ; write(1, out, rdx)
    mov rax, 1          ; syscall: write
    mov rdi, 1          ; stdout
    mov rsi, out
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

### 累積処理の準備
```asm
xor rax, rax        ; sum = 0
xor rcx, rcx        ; index = 0
```

1. **`xor rax, rax`** - RAXを0で初期化（合計の初期値）
2. **`xor rcx, rcx`** - RCXを0で初期化（配列のインデックス）

### 最初の3バイトを累積するループ
```asm
.loop:
    cmp rcx, 3          ; インデックスが3かチェック
    jge .done_accumulate ; インデックス >= 3なら終了
    
    mov bl, byte [buf + rcx]  ; buf[index]の値をBLに読み込む
    movzx rbx, bl       ; BLを64ビットにゼロ拡張してRBXに格納
    add rax, rbx        ; 合計に値を加算
    inc rcx             ; インデックスを1増やす
    jmp .loop           ; ループの先頭に戻る
```

3. **`cmp rcx, 3`** - インデックスが3かチェック
4. **`jge .done_accumulate`** - インデックスが3以上ならループ終了
   - 最初の3バイトのみを処理するため
5. **`mov bl, byte [buf + rcx]`** - バッファの現在位置のバイト値をBLに読み込む
   - BLはRBXの下位8ビット
6. **`movzx rbx, bl`** - BLを64ビットにゼロ拡張してRBXに格納
   - `movzx`は"Move with Zero-Extend"の略
   - 符号なし拡張: 上位ビットを0で埋める
   - 例: BL=0xFF → RBX=0x00000000000000FF
7. **`add rax, rbx`** - 合計（RAX）に現在の値（RBX）を加算
   - RAX = RAX + RBX
8. **`inc rcx`** - インデックスを1増やす
9. **`jmp .loop`** - ループの先頭に戻る

### 結果をASCII文字に変換して出力
```asm
.done_accumulate:
    ; convert sum to decimal string
    ; for simplicity, assume single digit result
    add al, '0'         ; 合計値（AL）に'0'を足してASCII文字に変換
    mov byte [out], al  ; 出力バッファにASCII文字を書き込む
    mov rdx, 1          ; 出力するバイト数（1バイト）

    ; write(1, out, rdx)
    mov rax, 1          ; syscall: write
    mov rdi, 1          ; stdout
    mov rsi, out
    syscall
```

10. **`.done_accumulate:`** - ループ終了後の処理
11. **`add al, '0'`** - 合計値（AL、RAXの下位8ビット）に'0'を足してASCII文字に変換
    - このコードは合計が1桁（0-9）であることを前提としている
12. **`mov byte [out], al`** - 変換したASCII文字を出力バッファに書き込む
13. **`mov rdx, 1`** - 出力するバイト数を1に設定
14. **システムコールwrite** - 結果を標準出力に書き出し

## ポイント

- **MOVZX命令**: 符号なし拡張（Zero-Extend）
  - 8ビット（BL）を64ビット（RBX）に拡張
  - 上位ビットを0で埋める
  - 符号付き拡張の場合は`movsx`（Sign-Extend）を使用
- **累積処理**: ループで値を1つずつ加算していく
  - 初期値を0に設定
  - 各反復で現在の値を合計に加算
- **範囲制限**: `cmp rcx, 3`で最初の3バイトのみを処理
- **ASCII変換**: 数値結果をASCII文字に変換して出力
  - このコードは1桁の結果を前提としている
  - 複数桁の場合は除算と剰余演算が必要
