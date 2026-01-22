グランドステージ: 1.The Accumulator

フェーズ: Phase 1 - Registers & ALU

ステージ名: Mov & Call

ステージ内容: 入力レジスタ RDI の値を読み取り、そのまま RAX に返す

学習ポイント: MOV命令、レジスタ間の値の移動、RDI（入力）とRAX（戻り値）の役割

## 解説

このステージでは、MOV命令を使ってレジスタ間で値を移動させます。

- **RDI**: 入力値が格納されるレジスタ
- **RAX**: 戻り値を格納するレジスタ

MOV命令の基本的な構文は `mov 宛先, 元` です。

## 正解コード

```asm
section .bss
    buf resb 16

section .text
    global _start

_start:
    ; MISSION: Mov & Call
    ; read from stdin (syscall 0), write to stdout (syscall 1)

    ; read(0, buf, 16)
    mov rax, 0          ; syscall: read
    mov rdi, 0          ; stdin
    mov rsi, buf        ; buffer
    mov rdx, 16         ; size
    syscall

    ; write(1, buf, rax)
    mov rdx, rax        ; number of bytes read
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

### セクション定義
- `section .bss` - 未初期化データセクション（バッファ用）
- `buf resb 16` - 16バイトのバッファを確保

### システムコール: read (syscall 0)
```asm
mov rax, 0          ; syscall: read
mov rdi, 0          ; stdin
mov rsi, buf        ; buffer
mov rdx, 16         ; size
syscall
```

1. **`mov rax, 0`** - システムコール番号0（read）をRAXに設定
2. **`mov rdi, 0`** - ファイルディスクリプタ0（標準入力）をRDIに設定
3. **`mov rsi, buf`** - 読み込み先バッファのアドレスをRSIに設定
4. **`mov rdx, 16`** - 読み込む最大バイト数（16）をRDXに設定
5. **`syscall`** - システムコールを実行（実際の入力を読み込む）
   - 戻り値: RAXに読み込んだバイト数が格納される

### システムコール: write (syscall 1)
```asm
mov rdx, rax        ; number of bytes read
mov rax, 1          ; syscall: write
mov rdi, 1          ; stdout
mov rsi, buf
syscall
```

1. **`mov rdx, rax`** - 読み込んだバイト数（readの戻り値）をRDXに設定
2. **`mov rax, 1`** - システムコール番号1（write）をRAXに設定
3. **`mov rdi, 1`** - ファイルディスクリプタ1（標準出力）をRDIに設定
4. **`mov rsi, buf`** - 書き込み元バッファのアドレスをRSIに設定
5. **`syscall`** - システムコールを実行（バッファの内容を出力）

### システムコール: exit (syscall 60)
```asm
mov rax, 60
xor rdi, rdi
syscall
```

1. **`mov rax, 60`** - システムコール番号60（exit）をRAXに設定
2. **`xor rdi, rdi`** - RDIを0にクリア（終了コード0）
3. **`syscall`** - プログラムを終了

## ポイント

- **システムコール**: OSの機能を呼び出すためのメカニズム
- **x86_64のシステムコール規約**:
  - RAX: システムコール番号
  - RDI: 第1引数
  - RSI: 第2引数
  - RDX: 第3引数
  - 戻り値はRAXに格納される
- **read/write**: バイト単位で入出力を行う
- **buf**: メモリ上の一時的なデータ保存領域
