#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(git rev-parse --show-toplevel)"
STAGE_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

cargo run --quiet --manifest-path "$ROOT_DIR/src-tauri/Cargo.toml" --bin stage_runner -- \
  --level-id "02_Addition" \
  --syntax Intel \
  --asm "$STAGE_DIR/collect.asm"

cargo run --quiet --manifest-path "$ROOT_DIR/src-tauri/Cargo.toml" --bin stage_runner -- \
  --level-id "02_Addition" \
  --syntax Att \
  --asm "$STAGE_DIR/collect_Att.asm"

