#!/bin/bash

set -e

PATH="$(pwd)":$PATH # simulating a custom `node`

eval "$(fnm env --multi)"
fnm install 10
fnm use 10
fnm use system

NVER="$(node -v)"
if [ "$NVER" != "custom build" ]; then
  echo "Expected \`node -v\` to be resolved to the system Node.js, but got $NVER"
  exit 1
fi
