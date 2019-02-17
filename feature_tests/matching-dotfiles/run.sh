#!/bin/bash

eval $(fnm env)
fnm install
fnm use

if [ "$(node --version)" != "11.10.0" ]; then
  echo "Expected Node version is not v11.10.0!"
  exit 1
fi
