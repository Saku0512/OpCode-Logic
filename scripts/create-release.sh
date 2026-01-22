#!/bin/bash

# GitHubリリース作成スクリプト
# 使用方法: ./scripts/create-release.sh [version] [message]

set -e

VERSION=${1:-"0.1.0"}
MESSAGE=${2:-"Release v$VERSION"}

echo "🚀 GitHubリリースを作成します"
echo "バージョン: $VERSION"
echo "メッセージ: $MESSAGE"
echo ""

# Gitの状態を確認
if [ -n "$(git status --porcelain)" ]; then
    echo "⚠️  未コミットの変更があります。コミットしてください。"
    exit 1
fi

# タグが既に存在するか確認
if git rev-parse "v$VERSION" >/dev/null 2>&1; then
    echo "⚠️  タグ v$VERSION は既に存在します。"
    read -p "続行しますか？ (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# タグを作成
echo "🏷️  タグ v$VERSION を作成中..."
git tag -a "v$VERSION" -m "$MESSAGE"

# タグをプッシュ
echo "📤 タグをプッシュ中..."
git push origin "v$VERSION"

echo ""
echo "✅ リリースタグを作成しました: v$VERSION"
echo "📋 GitHub Actionsが自動的にリリースをビルドします"
echo ""
echo "リリースの確認: https://github.com/$(git config --get remote.origin.url | sed 's/.*github.com[:/]\(.*\)\.git/\1/')/releases"
