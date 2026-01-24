## Third-party notices

このプロジェクト（OpCode Logic）は **GPL-2.0-only** で配布します。
デスクトップアプリとして **バイナリ配布**（GitHub Releases / 将来はサイト配布）を行う場合、GPLv2（2-only）の条件に従い、受領者が本プロジェクトの対応ソースコードを入手できる状態にします。

- **本プロジェクトのライセンス本文**: `LICENSE`（GPLv2）
- **本プロジェクトの対応ソース**: リポジトリのソース、または配布ページ（Release/サイト）から入手可能にします。

このプロジェクトは以下のサードパーティソフトウェアに依存します。配布形態（静的/動的リンク、同梱物）によっては、それぞれのライセンス条件に従う必要があります。

### Unicorn Engine
- **Project**: Unicorn CPU Emulator
- **Repository**: `https://github.com/unicorn-engine/unicorn`
- **Used via**: Rust crate `unicorn-engine`
- **Crate version**: `unicorn-engine 2.1.5`（`src-tauri/Cargo.toml` / `src-tauri/Cargo.lock`）
- **License**: GPL-2.0-only（詳細は上記リポジトリのLICENSEを参照）

### Keystone Engine
- **Project**: Keystone Assembler Engine
- **Repository**: `https://github.com/keystone-engine/keystone`
- **Used via**: Rust crate `keystone-engine`
- **Crate version**: `keystone-engine 0.1.0`（`src-tauri/Cargo.toml` / `src-tauri/Cargo.lock`）
- **License**: GPL-2.0-only（詳細は上記リポジトリのLICENSEを参照）

