# テスト結果

## フロントエンドのテスト ✅

全てのテストが正常に通過しました！

```
✓ src/lib/components/Editor.test.ts (3 tests) 29ms
✓ src/lib/components/IOView.test.ts (6 tests) 54ms
✓ src/lib/components/RegisterView.test.ts (4 tests) 65ms

Test Files  3 passed (3)
Tests  13 passed (13)
```

### テスト内容

1. **Editor.test.ts** - Editor コンポーネントのテスト
   - プレースホルダーの表示
   - 初期コード値の表示
   - スタイリングクラスの確認

2. **RegisterView.test.ts** - RegisterView コンポーネントのテスト
   - 全レジスタの表示
   - レジスタ値の表示
   - 未定義レジスタの処理
   - ヘッダーの確認

3. **IOView.test.ts** - IOView コンポーネントのテスト
   - 全セクションの表示
   - 入力値の表示
   - 出力値の表示
   - 期待値の表示
   - 空配列の処理
   - 負の数の表示

## Rust のテスト

Rust のテストコードは追加されていますが、実行にはシステムライブラリ（GTK、glib など）のインストールが必要です。

### テストを実行するには

システムライブラリをインストールしてください：

```bash
# Ubuntu/Debian
sudo apt-get install libgtk-3-dev libwebkit2gtk-4.0-dev libappindicator3-dev librsvg2-dev patchelf

# その後、テストを実行
cd src-tauri
cargo test
```

### 追加されたテスト

#### VM のテスト (`src-tauri/src/vm.rs`)
- 基本命令のテスト（MOV, ADD, SUB, INC, DEC, XOR）
- フラグのテスト（Zero Flag, Sign Flag）
- ジャンプ命令のテスト（JMP, JZ, JNZ, JS）
- スタック操作のテスト（PUSH, POP）
- 入出力命令のテスト（IN, OUT）
- メモリ操作のテスト
- パーサーのテスト（オペランド、ラベル、コメントなど）
- エッジケースのテスト

#### レベル検証のテスト (`src-tauri/src/levels.rs`)
- 全レベルの取得
- ID によるレベル取得
- 各レベルのテストケース確認
- 全レベルにテストケースがあることの確認

## テストの実行方法

### フロントエンドのテスト
```bash
npm test
```

### Rust のテスト
```bash
cd src-tauri
cargo test
```

### 特定のテストのみ実行
```bash
# フロントエンド
npm test -- Editor.test.ts

# Rust
cd src-tauri
cargo test vm::tests::test_mov_instructions
```
