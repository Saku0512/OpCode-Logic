グランドステージ: 1.The Accumulator

フェーズ: Phase 2 - Flags & Jumps

ステージ名: Unconditional

ステージ内容: 1を無限に出力し続ける（ストリーム出力、最初の5つの1がチェックされる）

学習ポイント: JMP命令、無条件ジャンプ、ラベル、無限ループ

## 解説

このステージでは、JMP命令を使って無限ループを作成します。

- **JMP命令**: 無条件で指定したラベルにジャンプ
- **ラベル**: コード内の位置を表すマーカー
- **無限ループ**: 終了条件のないループ

## 正解コード

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

## コード解説

### JMP命令による制御フローの変更
```asm
jmp .read_input
```
1. **`jmp .read_input`** - `.read_input`ラベルに無条件ジャンプ
   - プログラムの実行順序を変更
   - この時点では何も実行せず、直接`.read_input`に飛ぶ

### 入力の読み込み
```asm
.read_input:
    ; read(0, buf, 16)
    mov rax, 0          ; syscall: read
    mov rdi, 0          ; stdin
    mov rsi, buf        ; buffer
    mov rdx, 16         ; size
    syscall
```
2. **`.read_input:`** - ラベルの定義（入力処理の開始地点）
3. **システムコールread** - 標準入力から最大16バイトを読み込み

### 出力処理へのジャンプ
```asm
jmp .write_output
```
4. **`jmp .write_output`** - `.write_output`ラベルに無条件ジャンプ
   - 入力処理の後、直接出力処理に移る

### 出力の書き込み
```asm
.write_output:
    ; write(1, buf, rax)
    mov rdx, rax        ; number of bytes read
    mov rax, 1          ; syscall: write
    mov rdi, 1          ; stdout
    mov rsi, buf
    syscall
```
5. **`.write_output:`** - ラベルの定義（出力処理の開始地点）
6. **システムコールwrite** - 読み込んだデータを標準出力に書き出し

### 終了処理へのジャンプ
```asm
jmp .exit
```
7. **`jmp .exit`** - `.exit`ラベルに無条件ジャンプ

### プログラムの終了
```asm
.exit:
    ; exit(0)
    mov rax, 60
    xor rdi, rdi
    syscall
```
8. **`.exit:`** - ラベルの定義（終了処理の開始地点）
9. **システムコールexit** - プログラムを終了

## ポイント

- **JMP命令**: 無条件で指定したラベルにジャンプ
  - 条件チェックなしで常にジャンプ
  - プログラムの実行順序を自由に変更可能
- **ラベル**: コード内の位置を表すマーカー
  - ラベル名の後に `:` を付けて定義
  - `.`で始まるラベルはローカルラベル（スコープ内でのみ有効）
- **制御フロー**: JMPを使うことで、コードを順番に実行せずに任意の位置に飛べる
- **このコードの流れ**: 
  1. `_start` → `jmp .read_input`
  2. `.read_input` → 入力処理 → `jmp .write_output`
  3. `.write_output` → 出力処理 → `jmp .exit`
  4. `.exit` → 終了処理
