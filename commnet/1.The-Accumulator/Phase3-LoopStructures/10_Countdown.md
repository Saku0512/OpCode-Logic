グランドステージ: 1.The Accumulator

フェーズ: Phase 3 - Loop Structures

ステージ名: Countdown

ステージ内容: RDI（値N）を読み取り、N, N-1, N-2, ..., 1 を順番に出力する（ストリーム出力）

学習ポイント: ループ構造、DEC命令、条件付きジャンプ、OUT命令

## 解説

このステージでは、カウントダウンループを実装します。

- **ループ**: 条件が満たされるまで繰り返し実行
- **DEC命令**: 値を1減らす
- **JNZ**: Zero Flagがセットされていないときにジャンプ（ループ継続）

## 正解コード

```asm
section .bss
    buf resb 16

section .text
    global _start

_start:
    ; MISSION: Countdown
    ; read a digit, count down from it to 0

    ; read(0, buf, 16)
    mov rax, 0          ; syscall: read
    mov rdi, 0          ; stdin
    mov rsi, buf        ; buffer
    mov rdx, 16         ; size
    syscall

    ; get the digit and convert to number
    mov al, byte [buf]
    sub al, '0'         ; convert ASCII to number
    
    ; setup for countdown
    xor rcx, rcx
    mov cl, al          ; rcx = number
    xor rsi, rsi        ; buffer index = 0

.loop:
    ; write current number
    mov al, cl
    add al, '0'         ; convert back to ASCII
    mov byte [buf + rsi], al
    inc rsi
    
    loop .loop

    ; write(1, buf, rsi)
    mov rdx, rsi        ; number of bytes
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

### ASCII文字から数値への変換
```asm
mov al, byte [buf]  ; 最初のバイト（ASCII文字）をALに読み込む
sub al, '0'         ; ASCII '0'（0x30）を引いて数値に変換
```
1. **`mov al, byte [buf]`** - バッファの最初のバイト（ASCII文字の数字）をALに読み込む
2. **`sub al, '0'`** - ASCII '0'（0x30）を引いて数値に変換
   - '0' = 0x30 → 0
   - '1' = 0x31 → 1
   - '2' = 0x32 → 2
   - など

### カウントダウンループの準備
```asm
xor rcx, rcx        ; RCXを0でクリア
mov cl, al          ; RCXの下位8ビットに数値を設定（カウンタ）
xor rsi, rsi        ; RSIを0で初期化（バッファのインデックス）
```

3. **`xor rcx, rcx`** - RCXを0でクリア
4. **`mov cl, al`** - ALの値をCL（RCXの下位8ビット）にコピー
   - RCXがカウンタとして使用される
5. **`xor rsi, rsi`** - RSIを0で初期化（出力バッファのインデックス）

### カウントダウンループ
```asm
.loop:
    ; write current number
    mov al, cl          ; 現在のカウンタ値をALにコピー
    add al, '0'         ; 数値に'0'を足してASCII文字に変換
    mov byte [buf + rsi], al  ; バッファにASCII文字を書き込む
    inc rsi             ; バッファインデックスを1増やす
    
    loop .loop          ; RCXを1減らし、0でなければ.loopに戻る
```

6. **`.loop:`** - ループの開始ラベル
7. **`mov al, cl`** - 現在のカウンタ値（CL）をALにコピー
8. **`add al, '0'`** - 数値に'0'（0x30）を足してASCII文字に変換
   - 0 → '0' (0x30)
   - 1 → '1' (0x31)
   - 2 → '2' (0x32)
   - など
9. **`mov byte [buf + rsi], al`** - 変換したASCII文字をバッファに書き込む
10. **`inc rsi`** - バッファインデックスを1増やす（次の位置へ）
11. **`loop .loop`** - LOOP命令の実行
    - RCXを1減らす（`dec rcx`と同じ）
    - RCXが0でなければ`.loop`にジャンプ
    - RCXが0なら次の命令に進む

### 結果の出力
```asm
; write(1, buf, rsi)
mov rdx, rsi        ; number of bytes
mov rax, 1          ; syscall: write
mov rdi, 1          ; stdout
mov rsi, buf
syscall
```
カウントダウンの結果を標準出力に書き出します。

## ポイント

- **LOOP命令**: カウンタ（RCX）を自動的に減らし、0でなければ指定したラベルにジャンプ
  - `loop .loop`は`dec rcx; jnz .loop`と同じ意味
  - ループ専用の便利な命令
- **ASCII変換**: 
  - 数値 → ASCII: `add al, '0'`
  - ASCII → 数値: `sub al, '0'`
- **カウントダウンの動作**:
  - 入力が'3'の場合: RCX = 3
  - 1回目: RCX=3 → '3'を出力 → RCX=2
  - 2回目: RCX=2 → '2'を出力 → RCX=1
  - 3回目: RCX=1 → '1'を出力 → RCX=0
  - 4回目: RCX=0 → ループ終了
- **出力順序**: N, N-1, N-2, ..., 1（降順）
