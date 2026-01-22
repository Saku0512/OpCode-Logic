グランドステージ: 1.The Accumulator

フェーズ: Phase 1 - Registers & ALU

ステージ名: Addition

ステージ内容: RDI（値A）とRSI（値B）を読み取り、A + B を計算してRAXに返す

学習ポイント: ADD命令、2つのレジスタの値を加算、RSIレジスタの使用

## 解説

このステージでは、ADD命令を使って2つの値を加算します。

- **RDI**: 第1引数（値A）
- **RSI**: 第2引数（値B）
- **RAX**: 戻り値（A + B）

ADD命令の構文は `add 宛先, 元` で、`宛先 = 宛先 + 元` を実行します。

## 正解コード

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

## コード解説

### 入力の読み込み
```asm
mov rax, 0          ; syscall: read
mov rdi, 0          ; stdin
mov rsi, buf        ; buffer
mov rdx, 16         ; size
syscall
```
標準入力から最大16バイトを読み込み、`buf`に格納します。RAXには実際に読み込んだバイト数が返されます。

### ループ処理の準備
```asm
mov rcx, rax        ; number of bytes read
xor r8, r8          ; index = 0
```
1. **`mov rcx, rax`** - 読み込んだバイト数をRCXに保存（ループの終了条件）
2. **`xor r8, r8`** - R8を0で初期化（配列のインデックス）

### 各バイトに1を加算するループ
```asm
.loop:
    cmp r8, rcx          ; インデックスと読み込んだバイト数を比較
    jge .done_add        ; インデックス >= バイト数なら終了
    mov al, byte [buf + r8]  ; buf[index]の値をALに読み込む
    add al, 1            ; ALに1を加算
    mov byte [buf + r8], al  ; 計算結果をbuf[index]に書き戻す
    inc r8               ; インデックスを1増やす
    jmp .loop            ; ループの先頭に戻る
```

1. **`cmp r8, rcx`** - 現在のインデックスと読み込んだバイト数を比較
2. **`jge .done_add`** - インデックスがバイト数以上ならループ終了
3. **`mov al, byte [buf + r8]`** - バッファの現在位置のバイト値をALレジスタに読み込む
   - `[buf + r8]`は「bufのアドレス + r8」の位置を表す
   - `byte`は1バイト単位でのアクセスを指定
4. **`add al, 1`** - ALレジスタの値に1を加算
5. **`mov byte [buf + r8], al`** - 計算結果を元の位置に書き戻す
6. **`inc r8`** - インデックスを1増やして次のバイトへ
7. **`jmp .loop`** - ループの先頭に戻る

### 結果の出力
```asm
.done_add:
    mov rdx, rcx        ; number of bytes to write
    mov rax, 1          ; syscall: write
    mov rdi, 1          ; stdout
    mov rsi, buf
    syscall
```
処理済みのバッファを標準出力に書き出します。

## ポイント

- **メモリアクセス**: `[buf + r8]`で配列の要素にアクセス
- **バイト単位処理**: `byte`修飾子で1バイト単位で操作
- **ループ制御**: CMPとJGEでループの終了条件を判定
- **ADD命令**: 値に1を加算（INC命令でも可能だが、ADDは任意の値に加算可能）
- **インデックス**: R8レジスタを配列のインデックスとして使用
