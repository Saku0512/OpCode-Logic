# OpCode Logic

アセンブリ言語を学ぶためのインタラクティブなデスクトップアプリケーション。仮想マシン上でアセンブリコードを実行し、段階的に学習できるパズルゲームです。

## 概要

OpCode Logicは、x86-64アセンブリ言語を楽しく学ぶための教育用デスクトップアプリケーションです。
Progateのようなスライド形式の学習モードと、パズル形式の実践モードを通じて、レジスタ操作、算術演算、フラグ、ジャンプ、ループなどの基本的なアセンブリ概念を学習できます。

## 推奨使用者

- プログラムそのものへの理解を深めたい人
- AIに頼らないでデバッグしたい人
- プログラムをAIが出力する魔法だと思っている人

## 主な機能

- **インタラクティブな学習**: スライドによる解説と、その場でのコーディング演習を組み合わせた学習体験
- **仮想マシン実行**: Intel構文とAT&T構文の両方に対応したアセンブリコードの実行
- **使いやすいUI**:
  - ドラッグ＆ドロップで操作可能なUI
  - レジスタやI/Oストリームをリアルタイムで確認可能
  - エディタのTabキーインデント対応
- **進捗管理**: 学習の進捗状況を自動保存
- **多言語対応**: 日本語と英語に対応

## カリキュラム

### Phase 1: Basics & I/O

- Intro: アセンブリの基本構造とシステムコール
- IO: 入出力の基本 (`read`, `write`)
- Registers: レジスタ操作とメモリ (`mov`, `add`, `sub`)

### Phase 2: Logic & Control Flow (Coming Soon)

- Flags: 条件分岐の仕組み
- Jumps: ループと制御フロー

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

## 開発

### 型チェック

```bash
npm run check
```

## ライセンス

GPL-2.0-only

## 推奨IDE設定

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
