グランドステージ: 1.The Accumulator

フェーズ: Phase 2 - Flags & Jumps

ステージ名: Zero Flag

ステージ内容: RDI（値A）を読み取り、Aが0なら1を、そうでなければ0を返す

学習ポイント: Zero Flag（ZF）、条件付きジャンプ（JZ/JNZ）、CMP命令

## 解説

このステージでは、Zero Flag（ZF）と条件付きジャンプを使います。

- **Zero Flag (ZF)**: 演算結果が0のときにセットされるフラグ
- **JZ**: Zero Flagがセットされているときにジャンプ
- **JNZ**: Zero Flagがセットされていないときにジャンプ
- **CMP命令**: 2つの値を比較してフラグを設定（実際には減算を行い、結果を捨てる）

## 正解コード

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

### ヌルバイトをスペースに置換するループ
```asm
.loop:
    cmp r8, rcx          ; インデックスと読み込んだバイト数を比較
    jge .done_replace    ; インデックス >= バイト数なら終了
    mov al, byte [buf + r8]  ; buf[index]の値をALに読み込む
    cmp al, 0           ; ALが0かチェック（ZFを設定）
    jnz .next_byte      ; 0でなければ.next_byteにジャンプ
    mov byte [buf + r8], 0x20 ; 0なら0x20（スペース）に置換
.next_byte:
    inc r8              ; インデックスを1増やす
    jmp .loop           ; ループの先頭に戻る
```

1. **`cmp r8, rcx`** - 現在のインデックスと読み込んだバイト数を比較
2. **`jge .done_replace`** - インデックスがバイト数以上ならループ終了
3. **`mov al, byte [buf + r8]`** - バッファの現在位置のバイト値をALに読み込む
4. **`cmp al, 0`** - ALと0を比較（AL - 0を計算してフラグを設定）
   - ALが0なら結果は0 → Zero Flag (ZF) がセットされる
   - ALが0でなければ結果は0以外 → ZFがクリアされる
5. **`jnz .next_byte`** - Zero Flagがセットされていなければ（AL != 0）.next_byteにジャンプ
   - つまり、ALが0でない場合は置換せずに次へ
6. **`mov byte [buf + r8], 0x20`** - ALが0の場合のみ実行される
   - 0x20（32、スペース文字）に置換
7. **`.next_byte:`** - 共通処理
   - **`inc r8`** - インデックスを1増やす
   - **`jmp .loop`** - ループの先頭に戻る

### 結果の出力
```asm
.done_replace:
    mov rdx, rcx        ; number of bytes to write
    mov rax, 1          ; syscall: write
    mov rdi, 1          ; stdout
    mov rsi, buf
    syscall
```
処理済みのバッファを標準出力に書き出します。

## ポイント

- **CMP命令**: 値を変更せず、フラグだけを設定
  - `cmp al, 0`は`al - 0`を計算し、結果を捨ててフラグだけを設定
  - 結果が0ならZFがセット、0でなければZFがクリア
- **JNZ命令**: "Jump if Not Zero"の略
  - ZFがセットされていない（結果が0でない）ときにジャンプ
  - このコードでは「AL != 0」のときにジャンプ
- **条件分岐の流れ**:
  - AL == 0: ZFセット → JNZでジャンプしない → 置換処理実行
  - AL != 0: ZFクリア → JNZでジャンプ → 置換処理スキップ
- **0x20**: スペース文字（ASCII 32）
