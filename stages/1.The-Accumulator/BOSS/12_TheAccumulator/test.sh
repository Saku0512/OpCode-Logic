#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(git rev-parse --show-toplevel)"
STAGE_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

cargo run --quiet --manifest-path "$ROOT_DIR/src-tauri/Cargo.toml" --bin stage_runner -- \
  --level-id "12_TheAccumulator" \
  --asm "$STAGE_DIR/collect.asm"

