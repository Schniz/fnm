#!/bin/bash

set -e

eval $(fnm env)
fnm install
fnm use

if [ "$(node --version)" != "v11.10.0" ]; then
  echo "Node version is not v11.10.0!"
  exit 1
fi
