#!/bin/bash

set -e

eval $(fnm env)
fnm install
fnm use

if [ "$(node --version)" != "v10.9.0" ]; then
  echo "Node version is not v10.9.0!"
  exit 1
fi
