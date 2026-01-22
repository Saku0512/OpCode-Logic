グランドステージ: 1.The Accumulator

フェーズ: BOSS Stage

ステージ名: The Accumulator

ステージ内容: IN命令を使って入力を読み取り、0が入力されるまで合計を累積し、最終的な合計をRAXに返す

学習ポイント: IN命令、ループ、条件判定、累積処理の組み合わせ

## 解説

このステージはボスステージで、これまで学んだすべての技術を組み合わせます。

- **IN命令**: 入力キューから値を読み取る
- **ループ**: 0が入力されるまで繰り返す
- **累積処理**: 読み取った値を合計に加算

## 正解コード

```asm
section .bss
    buf resb 32
    out resb 32

section .text
    global _start

_start:
    ; MISSION: The Accumulator (BOSS STAGE)
    ; Challenge: Combine all learned skills
    ; Read input, process each byte:
    ; - For uppercase letters: convert to lowercase
    ; - For digits: increment
    ; - For others: pass through
    ; Output result

    ; read(0, buf, 32)
    mov rax, 0          ; syscall: read
    mov rdi, 0          ; stdin
    mov rsi, buf        ; buffer
    mov rdx, 32         ; size
    syscall

    ; process each byte
    mov rcx, rax        ; number of bytes read
    xor r8, r8          ; read index = 0
    xor r9, r9          ; write index = 0

.loop:
    cmp r8, rcx
    jge .done_process
    
    mov al, byte [buf + r8]
    
    ; check if uppercase letter (A-Z: 0x41-0x5A)
    cmp al, 'A'
    jl .check_digit
    cmp al, 'Z'
    jg .check_digit
    
    ; convert to lowercase (add 0x20)
    add al, 0x20
    jmp .write_byte
    
.check_digit:
    ; check if digit (0-9: 0x30-0x39)
    cmp al, '0'
    jl .pass_through
    cmp al, '9'
    jg .pass_through
    
    ; increment digit
    inc al
    cmp al, '9' + 1
    jne .write_byte
    mov al, '0'         ; wrap 9 to 0
    jmp .write_byte
    
.pass_through:
    ; keep as is
    
.write_byte:
    mov byte [out + r9], al
    inc r9
    inc r8
    jmp .loop

.done_process:
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
mov rdx, 32         ; size
syscall
```
標準入力から最大32バイトを読み込み、`buf`に格納します。

### ループ処理の準備
```asm
mov rcx, rax        ; number of bytes read
xor r8, r8          ; read index = 0
xor r9, r9          ; write index = 0
```

1. **`mov rcx, rax`** - 読み込んだバイト数をRCXに保存
2. **`xor r8, r8`** - R8を0で初期化（読み取りインデックス）
3. **`xor r9, r9`** - R9を0で初期化（書き込みインデックス）

### 各バイトを処理するループ
```asm
.loop:
    cmp r8, rcx          ; 読み取りインデックスとバイト数を比較
    jge .done_process    ; インデックス >= バイト数なら終了
    
    mov al, byte [buf + r8]  ; 現在のバイトをALに読み込む
```

4. **`cmp r8, rcx`** - 読み取りインデックスとバイト数を比較
5. **`jge .done_process`** - インデックスがバイト数以上ならループ終了
6. **`mov al, byte [buf + r8]`** - 現在のバイトをALに読み込む

### 大文字判定と変換
```asm
    ; check if uppercase letter (A-Z: 0x41-0x5A)
    cmp al, 'A'         ; ALと'A'（0x41）を比較
    jl .check_digit     ; AL < 'A'なら.check_digitにジャンプ
    cmp al, 'Z'         ; ALと'Z'（0x5A）を比較
    jg .check_digit     ; AL > 'Z'なら.check_digitにジャンプ
    
    ; convert to lowercase (add 0x20)
    add al, 0x20        ; 大文字なら0x20を足して小文字に変換
    jmp .write_byte     ; .write_byteにジャンプ
```

7. **`cmp al, 'A'`** - ALと'A'（0x41）を比較
8. **`jl .check_digit`** - AL < 'A'なら数字チェックへ（大文字ではない）
9. **`cmp al, 'Z'`** - ALと'Z'（0x5A）を比較
10. **`jg .check_digit`** - AL > 'Z'なら数字チェックへ（大文字ではない）
11. **`add al, 0x20`** - 大文字の場合、0x20を足して小文字に変換
    - 'A' (0x41) → 'a' (0x61)
    - 'B' (0x42) → 'b' (0x62)
    - など
12. **`jmp .write_byte`** - 書き込み処理へジャンプ

### 数字判定とインクリメント
```asm
.check_digit:
    ; check if digit (0-9: 0x30-0x39)
    cmp al, '0'         ; ALと'0'（0x30）を比較
    jl .pass_through    ; AL < '0'なら.pass_throughにジャンプ
    cmp al, '9'         ; ALと'9'（0x39）を比較
    jg .pass_through    ; AL > '9'なら.pass_throughにジャンプ
    
    ; increment digit
    inc al              ; 数字なら1増やす
    cmp al, '9' + 1     ; 結果が'9' + 1（':' = 0x3A）かチェック
    jne .write_byte     ; '9' + 1でなければ.write_byteにジャンプ
    mov al, '0'         ; '9' + 1なら'0'にラップ（9→0）
    jmp .write_byte     ; .write_byteにジャンプ
```

13. **`.check_digit:`** - 数字チェックのラベル
14. **`cmp al, '0'`** - ALと'0'（0x30）を比較
15. **`jl .pass_through`** - AL < '0'ならそのまま通過
16. **`cmp al, '9'`** - ALと'9'（0x39）を比較
17. **`jg .pass_through`** - AL > '9'ならそのまま通過
18. **`inc al`** - 数字の場合、1増やす
    - '0' → '1'
    - '1' → '2'
    - ...
    - '9' → ':'（0x3A、数字ではない）
19. **`cmp al, '9' + 1`** - 結果が':'（0x3A）かチェック
20. **`jne .write_byte`** - ':'でなければ書き込み処理へ
21. **`mov al, '0'`** - ':'の場合、'0'にラップ（9→0の循環）
22. **`jmp .write_byte`** - 書き込み処理へジャンプ

### その他の文字はそのまま通過
```asm
.pass_through:
    ; keep as is
```

23. **`.pass_through:`** - そのまま通過するラベル
    - 大文字でも数字でもない文字は何も処理しない

### 処理結果の書き込み
```asm
.write_byte:
    mov byte [out + r9], al  ; 処理済みのバイトを出力バッファに書き込む
    inc r9              ; 書き込みインデックスを1増やす
    inc r8              ; 読み取りインデックスを1増やす
    jmp .loop           ; ループの先頭に戻る
```

24. **`.write_byte:`** - 書き込み処理のラベル
25. **`mov byte [out + r9], al`** - 処理済みのバイトを出力バッファに書き込む
26. **`inc r9`** - 書き込みインデックスを1増やす
27. **`inc r8`** - 読み取りインデックスを1増やす
28. **`jmp .loop`** - ループの先頭に戻る

### 結果の出力
```asm
.done_process:
    ; write(1, out, r9)
    mov rdx, r9         ; number of bytes to write
    mov rax, 1          ; syscall: write
    mov rdi, 1          ; stdout
    mov rsi, out
    syscall
```
処理済みのバッファを標準出力に書き出します。

## ポイント

- **複数の条件分岐**: 大文字 → 数字 → その他の順で判定
- **範囲チェック**: 
  - 大文字: 'A' <= AL <= 'Z'
  - 数字: '0' <= AL <= '9'
- **文字変換**:
  - 大文字→小文字: `add al, 0x20`（XORでも可能）
  - 数字インクリメント: `inc al`（9の場合は'0'にラップ）
- **ラップアラウンド**: 数字'9'をインクリメントすると':'になるため、'0'に戻す
- **複合処理**: これまで学んだすべての技術（MOV、ADD、INC、CMP、条件ジャンプ、ループ）を組み合わせた総合問題
