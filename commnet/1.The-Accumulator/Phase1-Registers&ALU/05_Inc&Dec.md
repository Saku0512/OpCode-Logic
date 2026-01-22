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

### インデックスに応じてINC/DECするループ
```asm
.loop:
    cmp r8, rcx          ; インデックスと読み込んだバイト数を比較
    jge .done_inc_dec    ; インデックス >= バイト数なら終了
    mov al, byte [buf + r8]  ; buf[index]の値をALに読み込む
    test r8, 1          ; インデックスが奇数かチェック
    jnz .do_dec         ; 奇数なら.decにジャンプ
.do_inc:
    inc al              ; 偶数インデックス: 値を1増やす
    jmp .next           ; .nextにジャンプ
.do_dec:
    dec al              ; 奇数インデックス: 値を1減らす
.next:
    mov byte [buf + r8], al  ; 計算結果をbuf[index]に書き戻す
    inc r8              ; インデックスを1増やす
    jmp .loop           ; ループの先頭に戻る
```

1. **`cmp r8, rcx`** - 現在のインデックスと読み込んだバイト数を比較
2. **`jge .done_inc_dec`** - インデックスがバイト数以上ならループ終了
3. **`mov al, byte [buf + r8]`** - バッファの現在位置のバイト値をALに読み込む
4. **`test r8, 1`** - R8と1をAND演算してフラグを設定
   - `test`命令はAND演算を行い、結果を捨ててフラグだけを設定
   - `r8 & 1`で最下位ビットをチェック（奇数なら1、偶数なら0）
   - 結果が0でなければZFがクリアされる
5. **`jnz .do_dec`** - Zero Flagがセットされていなければ（奇数なら）.do_decにジャンプ
6. **`.do_inc:`** - 偶数インデックスの場合
   - **`inc al`** - ALの値を1増やす
7. **`.do_dec:`** - 奇数インデックスの場合
   - **`dec al`** - ALの値を1減らす
8. **`.next:`** - 共通処理
   - **`mov byte [buf + r8], al`** - 計算結果を元の位置に書き戻す
   - **`inc r8`** - インデックスを1増やす
   - **`jmp .loop`** - ループの先頭に戻る

### 結果の出力
```asm
.done_inc_dec:
    mov rdx, rcx        ; number of bytes to write
    mov rax, 1          ; syscall: write
    mov rdi, 1          ; stdout
    mov rsi, buf
    syscall
```
処理済みのバッファを標準出力に書き出します。

## ポイント

- **TEST命令**: AND演算を行い、結果を捨ててフラグだけを設定
  - `test r8, 1`は「r8 & 1」を計算し、最下位ビットをチェック
  - 結果が0ならZFがセット、0でなければZFがクリア
- **条件分岐**: JNZ（Jump if Not Zero）で奇数判定
  - 偶数: 最下位ビットが0 → ZFセット → JNZでジャンプしない → INC実行
  - 奇数: 最下位ビットが1 → ZFクリア → JNZでジャンプ → DEC実行
- **INC/DEC命令**: 1増減する専用命令
  - `inc al`は`add al, 1`と同じだが、より効率的
  - `dec al`は`sub al, 1`と同じだが、より効率的
- **ラベル**: `.do_inc`、`.do_dec`、`.next`はローカルラベル（`.`で始まる）
