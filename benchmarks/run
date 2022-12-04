#!/bin/bash

set -e

export FNM_DIR

BASE_DIR="$(dirname "$(realpath "$0")")"
cd "$BASE_DIR" || exit 1

FNM_DIR="$(mktemp -d)"
export PATH="$BASE_DIR/../target/release:$PATH"

mkdir results 2>/dev/null || :

if [ ! -f "$BASE_DIR/../target/release/fnm" ]; then
  echo "Can't access the release version of fnm.rs"
  exit 1
fi

if ! command -v hyperfine >/dev/null 2>&1; then
  echo "Can't access Hyperfine. Are you sure it is installed?"
  echo "  if not, visit https://github.com/sharkdp/hyperfine"
  exit 1
fi

# Running it with warmup means we're going to have the versions
# pre-installed. I think it is good because you open your shell more times
# than you install Node versions.
hyperfine \
  --warmup=2 \
  --min-runs=40 \
  --time-unit=millisecond \
  --export-json="./results/basic.json" \
  --export-markdown="./results/basic.md" \
  "basic/nvm" \
  "basic/fnm"
