#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

failures=0
failed_list=()

shopt -s nullglob
tests=( "$ROOT_DIR"/stages/*/*/*/test.sh "$ROOT_DIR"/stages/*/*/*/*/test.sh )

if [ ${#tests[@]} -eq 0 ]; then
  echo "No stage test.sh found under stages/."
  exit 1
fi

for t in "${tests[@]}"; do
  rel="${t#"$ROOT_DIR"/}"
  echo "==> $rel"
  if bash "$t"; then
    echo "PASS: $rel"
  else
    echo "FAIL: $rel" >&2
    failures=$((failures + 1))
    failed_list+=( "$rel" )
  fi
  echo
done

if [ "$failures" -ne 0 ]; then
  echo "FAILED ($failures):" >&2
  for f in "${failed_list[@]}"; do
    echo " - $f" >&2
  done
  exit 1
fi

echo "All stage tests passed (${#tests[@]})."

