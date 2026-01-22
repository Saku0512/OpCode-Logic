グランドステージ: 1.The Accumulator

フェーズ: Phase 2 - Flags & Jumps

ステージ名: Comparison

ステージ内容: RDI（値A）とRSI（値B）を読み取り、大きい方の値を返す

学習ポイント: CMP命令、条件付きジャンプ（JG/JL/JE）、値の比較

## 解説

このステージでは、2つの値を比較して大きい方を返します。

- **CMP命令**: 2つの値を比較してフラグを設定
- **JG**: 第1オペランドが第2オペランドより大きいときにジャンプ（Jump if Greater）
- **JL**: 第1オペランドが第2オペランドより小さいときにジャンプ（Jump if Less）
- **JE**: 2つの値が等しいときにジャンプ（Jump if Equal）

## 正解コード

```asm
section .bss
    buf resb 16
    out resb 16

section .text
    global _start

_start:
    ; MISSION: Comparison
    ; read from stdin, compare consecutive bytes, output +/=/- markers

    ; read(0, buf, 16)
    mov rax, 0          ; syscall: read
    mov rdi, 0          ; stdin
    mov rsi, buf        ; buffer
    mov rdx, 16         ; size
    syscall

    ; compare consecutive bytes
    mov rcx, rax        ; number of bytes read
    xor r8, r8          ; index = 0
    mov al, byte [buf]  ; prev = first byte
    xor r9, r9          ; output index = 0

.loop:
    cmp r8, rcx
    jge .done_compare
    inc r8
    cmp r8, rcx
    jge .done_compare
    
    mov bl, byte [buf + r8] ; current = buf[index]
    cmp bl, al
    je .is_equal
    jg .is_greater
    
.is_less:
    mov byte [out + r9], '-'
    jmp .next_iter
    
.is_equal:
    mov byte [out + r9], '='
    jmp .next_iter
    
.is_greater:
    mov byte [out + r9], '+'
    
.next_iter:
    mov al, bl          ; prev = current
    inc r9
    jmp .loop

.done_compare:
    ; write(1, out, r9)
    mov rdx, r9         ; number of bytes to write
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

### ループ処理の準備
```asm
mov rcx, rax        ; number of bytes read
xor r8, r8          ; index = 0
mov al, byte [buf]  ; prev = first byte
xor r9, r9          ; output index = 0
```

1. **`mov rcx, rax`** - 読み込んだバイト数をRCXに保存
2. **`xor r8, r8`** - R8を0で初期化（読み取りインデックス）
3. **`mov al, byte [buf]`** - 最初のバイトをALに読み込む（前の値として保存）
4. **`xor r9, r9`** - R9を0で初期化（出力インデックス）

### 連続するバイトを比較するループ
```asm
.loop:
    cmp r8, rcx          ; 読み取りインデックスとバイト数を比較
    jge .done_compare    ; インデックス >= バイト数なら終了
    inc r8               ; インデックスを1増やす（次のバイトへ）
    cmp r8, rcx          ; 再度範囲チェック
    jge .done_compare    ; 範囲外なら終了
    
    mov bl, byte [buf + r8]  ; 現在のバイトをBLに読み込む
    cmp bl, al          ; 現在のバイトと前のバイトを比較
    je .is_equal        ; 等しいなら.is_equalにジャンプ
    jg .is_greater      ; 現在のバイト > 前のバイトなら.is_greaterにジャンプ
    
.is_less:
    mov byte [out + r9], '-'  ; 現在のバイト < 前のバイト: '-'を出力
    jmp .next_iter
    
.is_equal:
    mov byte [out + r9], '='  ; 現在のバイト == 前のバイト: '='を出力
    jmp .next_iter
    
.is_greater:
    mov byte [out + r9], '+'  ; 現在のバイト > 前のバイト: '+'を出力
    
.next_iter:
    mov al, bl          ; 前の値 = 現在の値（次の比較のため）
    inc r9              ; 出力インデックスを1増やす
    jmp .loop           ; ループの先頭に戻る
```

5. **`cmp r8, rcx`** - 読み取りインデックスとバイト数を比較
6. **`jge .done_compare`** - インデックスがバイト数以上ならループ終了
7. **`inc r8`** - インデックスを1増やす（次のバイトへ移動）
8. **`cmp r8, rcx`** - 再度範囲チェック（最後のバイトの場合は比較できないため）
9. **`mov bl, byte [buf + r8]`** - 現在のバイトをBLに読み込む
10. **`cmp bl, al`** - 現在のバイト（BL）と前のバイト（AL）を比較
    - `bl - al`を計算してフラグを設定
11. **`je .is_equal`** - 等しい（ZFセット）なら.is_equalにジャンプ
12. **`jg .is_greater`** - 現在のバイト > 前のバイト（SF=OF=0, ZF=0）なら.is_greaterにジャンプ
13. **`.is_less:`** - 現在のバイト < 前のバイトの場合
    - **`mov byte [out + r9], '-'`** - 出力バッファに'-'を書き込む
14. **`.is_equal:`** - 現在のバイト == 前のバイトの場合
    - **`mov byte [out + r9], '='`** - 出力バッファに'='を書き込む
15. **`.is_greater:`** - 現在のバイト > 前のバイトの場合
    - **`mov byte [out + r9], '+'`** - 出力バッファに'+'を書き込む
16. **`.next_iter:`** - 共通処理
    - **`mov al, bl`** - 前の値 = 現在の値（次の比較のため）
    - **`inc r9`** - 出力インデックスを1増やす
    - **`jmp .loop`** - ループの先頭に戻る

### 結果の出力
```asm
.done_compare:
    ; write(1, out, r9)
    mov rdx, r9         ; number of bytes to write
    mov rax, 1          ; syscall: write
    mov rdi, 1          ; stdout
    mov rsi, out
    syscall
```
比較結果のマーカー（'+'、'='、'-'）を標準出力に書き出します。

## ポイント

- **連続する要素の比較**: 配列の隣接する要素を比較するパターン
- **CMP命令と条件ジャンプ**:
  - `cmp bl, al`で`bl - al`を計算
  - `je`（Jump if Equal）: ZFがセットされているとき
  - `jg`（Jump if Greater）: SF=OF=0かつZF=0のとき（符号付き比較で「より大きい」）
  - どちらでもない場合（`jl`相当）: 現在のバイト < 前のバイト
- **比較の順序**: JE → JG → それ以外（Less）の順で判定
- **前の値の保持**: ALレジスタに前のバイト値を保持し、次の比較に使用
- **出力バッファ**: 比較結果を別のバッファ（`out`）に書き込む
