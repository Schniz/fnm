#!/bin/bash

set -e

eval $(fnm env)
fnm install
fnm use

echo node --version
if [ "$(node --version)" != "v11.10.0" ]; then
  echo "Expected Node version is not v11.10.0!"
  exit 1
fi
