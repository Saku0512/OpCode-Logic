# OpCode Logic

アセンブリ言語を学ぶためのインタラクティブなデスクトップアプリケーション。仮想マシン上でアセンブリコードを実行し、段階的に学習できるパズルゲームです。

## 概要

OpCode Logicは、x86-64アセンブリ言語を楽しく学ぶための教育用デスクトップアプリケーションです。12のステージを通じて、レジスタ操作、算術演算、フラグ、ジャンプ、ループなどの基本的なアセンブリ概念を学習できます。

## 主な機能

- **仮想マシン実行**: Intel構文とAT&T構文の両方に対応したアセンブリコードの実行
- **12のステージ**: 段階的に難易度が上がる学習ステージ
- **リアルタイム実行**: レジスタ、フラグ、メモリの状態をリアルタイムで確認
- **進捗管理**: クリアしたステージは自動保存され、次のステージがアンロック
- **詳細な解説**: 各ステージクリア後に、コードの解説を読むことが可能
- **テストケース検証**: 複数のテストケースでコードの正確性を検証

## 対応プラットフォーム

- **Windows** (x64)
- **Linux** (x64)
- **macOS** (x64, ARM64)

## 技術スタック

- **フロントエンド**: SvelteKit 5, TypeScript, Vite
- **バックエンド**: Rust, Tauri 2
- **テスト**: Vitest, Testing Library

## セットアップ

### 必要な環境

- Node.js 18以上
- Rust 1.70以上
- Tauri CLI 2.x
- （Linux）追加で必要な可能性: `cmake`, `clang`, `llvm-dev`, `libclang-dev`（Unicorn/Keystoneのビルド用）

### インストール

```bash
# 依存関係のインストール
npm install

# 開発サーバーの起動
npm run tauri dev
```

### ビルド

```bash
# 本番ビルド（全プラットフォーム）
npm run tauri build

# Windows用のみ
npm run tauri build -- --target x86_64-pc-windows-msvc

# Linux用のみ
npm run tauri build -- --target x86_64-unknown-linux-gnu

# macOS用のみ
npm run tauri build -- --target x86_64-apple-darwin
npm run tauri build -- --target aarch64-apple-darwin  # Apple Silicon
```

## リリース

### ローカルでリリースビルド

```bash
# 全プラットフォーム用のビルドを実行
./scripts/build-release.sh [version]

# 例: バージョン0.3.0でビルド
./scripts/build-release.sh 0.3.0
```

ビルドファイルは `release/` ディレクトリに生成されます。

### GitHub Actionsで自動リリース

#### 方法1: タグをプッシュ

```bash
# リリースタグを作成してプッシュ
git tag -a v0.3.0 -m "Release v0.3.0"
git push origin v0.3.0
```

#### 方法2: スクリプトを使用

```bash
# リリーススクリプトを実行
./scripts/create-release.sh 0.3.0 "Release v0.3.0"
```

#### 方法3: GitHub UIから手動実行

1. GitHubリポジトリの「Actions」タブを開く
2. 「Release」ワークフローを選択
3. 「Run workflow」をクリック
4. バージョン番号を入力して実行

GitHub Actionsが自動的に以下を実行します：
- Windows用ビルド（.exe, .msi）
- Linux用ビルド（.AppImage, .deb）
- macOS用ビルド（Intel/ARM64用 .dmg）
- GitHubリリースの作成とアップロード

## 使い方

1. **レベル選択**: サイドバーから学習したいステージを選択
2. **コード記述**: エディタにアセンブリコードを記述（Intel構文またはAT&T構文）
3. **実行**: "RUN & VERIFY"ボタンをクリックしてコードを実行
4. **確認**: レジスタの状態、I/Oストリーム、期待値との比較を確認
5. **クリア**: すべてのテストケースを通過するとステージクリア
6. **解説**: クリアしたステージでは「解説を読む」ボタンから詳細な解説を閲覧可能

## ステージ構成

### Phase 1: Registers & ALU
- 01: Mov & Call - 基本的なMOV命令とシステムコール
- 02: Addition - ADD命令による加算
- 03: Subtraction - SUB命令による減算
- 04: The XOR Trick - XOR命令の活用
- 05: Inc & Dec - INC/DEC命令とストリーム出力

### Phase 2: Flags & Jumps
- 06: Unconditional - JMP命令による無条件ジャンプ
- 07: Zero Flag - Zero Flagと条件分岐
- 08: Sign Flag - Sign Flagと符号判定
- 09: Comparison - CMP命令と値の比較

### Phase 3: Loop Structures
- 10: Countdown - ループ構造とカウントダウン
- 11: Accumulate 3 - 累積処理

### BOSS Stage
- 12: The Accumulator - 総合チャレンジ

## 開発

### テストの実行

```bash
# 全テスト実行
npm test

# テストUI
npm run test:ui

# カバレッジ
npm run test:coverage
```

### 型チェック

```bash
npm run check
```

## ライセンス

GPL-2.0-only

## 推奨IDE設定

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
