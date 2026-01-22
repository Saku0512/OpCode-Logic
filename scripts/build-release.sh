#!/bin/bash

# OpCode Logic リリースビルドスクリプト
# 使用方法: ./scripts/build-release.sh [version]

set -e

VERSION=${1:-"0.1.0"}
RELEASE_DIR="release"
BUILD_DIR="build"

echo "🚀 OpCode Logic リリースビルド開始"
echo "バージョン: $VERSION"
echo ""

# リリースディレクトリのクリーンアップ
echo "📁 リリースディレクトリをクリーンアップ..."
rm -rf "$RELEASE_DIR"
mkdir -p "$RELEASE_DIR"

# フロントエンドのビルド
echo "📦 フロントエンドをビルド中..."
npm run build

# Windows用ビルド
echo ""
echo "🪟 Windows用ビルド中..."
npm run tauri build -- --target x86_64-pc-windows-msvc

# Windows用ファイルをリリースディレクトリにコピー
if [ -d "src-tauri/target/x86_64-pc-windows-msvc/release/bundle" ]; then
    mkdir -p "$RELEASE_DIR/windows"
    cp -r src-tauri/target/x86_64-pc-windows-msvc/release/bundle/msi/*.msi "$RELEASE_DIR/windows/" 2>/dev/null || true
    cp -r src-tauri/target/x86_64-pc-windows-msvc/release/bundle/nsis/*.exe "$RELEASE_DIR/windows/" 2>/dev/null || true
    echo "✅ Windows用ビルド完了"
else
    echo "⚠️  Windows用ビルドが見つかりませんでした"
fi

# Linux用ビルド
echo ""
echo "🐧 Linux用ビルド中..."
npm run tauri build -- --target x86_64-unknown-linux-gnu

# Linux用ファイルをリリースディレクトリにコピー
if [ -d "src-tauri/target/x86_64-unknown-linux-gnu/release/bundle" ]; then
    mkdir -p "$RELEASE_DIR/linux"
    cp -r src-tauri/target/x86_64-unknown-linux-gnu/release/bundle/appimage/*.AppImage "$RELEASE_DIR/linux/" 2>/dev/null || true
    cp -r src-tauri/target/x86_64-unknown-linux-gnu/release/bundle/deb/*.deb "$RELEASE_DIR/linux/" 2>/dev/null || true
    echo "✅ Linux用ビルド完了"
else
    echo "⚠️  Linux用ビルドが見つかりませんでした"
fi

# macOS用ビルド（x86_64）
echo ""
echo "🍎 macOS用ビルド中 (x86_64)..."
npm run tauri build -- --target x86_64-apple-darwin

# macOS用ファイルをリリースディレクトリにコピー
if [ -d "src-tauri/target/x86_64-apple-darwin/release/bundle" ]; then
    mkdir -p "$RELEASE_DIR/macos-x64"
    cp -r src-tauri/target/x86_64-apple-darwin/release/bundle/macos/*.dmg "$RELEASE_DIR/macos-x64/" 2>/dev/null || true
    cp -r src-tauri/target/x86_64-apple-darwin/release/bundle/macos/*.app "$RELEASE_DIR/macos-x64/" 2>/dev/null || true
    echo "✅ macOS用ビルド完了 (x86_64)"
else
    echo "⚠️  macOS用ビルドが見つかりませんでした (x86_64)"
fi

# macOS用ビルド（ARM64）
echo ""
echo "🍎 macOS用ビルド中 (ARM64)..."
npm run tauri build -- --target aarch64-apple-darwin

# macOS用ファイルをリリースディレクトリにコピー
if [ -d "src-tauri/target/aarch64-apple-darwin/release/bundle" ]; then
    mkdir -p "$RELEASE_DIR/macos-arm64"
    cp -r src-tauri/target/aarch64-apple-darwin/release/bundle/macos/*.dmg "$RELEASE_DIR/macos-arm64/" 2>/dev/null || true
    cp -r src-tauri/target/aarch64-apple-darwin/release/bundle/macos/*.app "$RELEASE_DIR/macos-arm64/" 2>/dev/null || true
    echo "✅ macOS用ビルド完了 (ARM64)"
else
    echo "⚠️  macOS用ビルドが見つかりませんでした (ARM64)"
fi

# リリース情報ファイルを作成
echo ""
echo "📝 リリース情報を作成中..."
cat > "$RELEASE_DIR/RELEASE_INFO.txt" << EOF
OpCode Logic Release
====================
Version: $VERSION
Build Date: $(date -u +"%Y-%m-%d %H:%M:%S UTC")

Platforms:
- Windows (x86_64): MSI installer and NSIS installer
- Linux (x86_64): AppImage and DEB package
- macOS (x86_64): DMG and App bundle
- macOS (ARM64): DMG and App bundle

Installation:
- Windows: Run the .msi or .exe installer
- Linux: 
  - AppImage: Make executable (chmod +x) and run
  - DEB: Install with 'sudo dpkg -i *.deb'
- macOS: Open the .dmg file and drag the app to Applications

For more information, see README.md
EOF

echo ""
echo "✨ リリースビルド完了！"
echo "📦 リリースファイルは '$RELEASE_DIR' ディレクトリにあります"
echo ""
ls -lh "$RELEASE_DIR"/*/
