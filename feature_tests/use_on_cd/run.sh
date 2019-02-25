#!/bin/bash

set -e

DIRECTORY=`dirname $0`

eval "`fnm env --multi`"
fnm install 6.11.3
fnm install 8.11.3
fnm use 6.11.3

if hash zsh 2>/dev/null; then
  echo ' > Running test on Zsh'

  zsh -c '
    set -e

    eval "`fnm env --multi --use-on-cd`"

    fnm use 6.11.3

    NODE_VERSION=$(node -v)
    if [ "$NODE_VERSION" != "v6.11.3" ]; then
      echo "Failed: Node version ($NODE_VERSION) is not v6.11.3"
      exit 1
    fi

    echo "$ cd app"
    cd app

    NODE_VERSION=$(node -v)
    if [ "$NODE_VERSION" != "v8.11.3" ]; then
      echo "Failed: Node version ($NODE_VERSION) is not v8.11.3"
      exit 1
    fi
  '
else
  echo "Skipping zsh test: \`zsh\` is not installed"
fi

echo " > Running test on Bash..."
bash -c '
  shopt -s expand_aliases
  eval "`fnm env --multi --use-on-cd`"
  fnm use 6.11.3
  NODE_VERSION=$(node -v)
  if [ "$NODE_VERSION" != "v6.11.3" ]; then
    echo "Failed: Node version ($NODE_VERSION) is not v6.11.3"
    exit 1
  fi
  cd app
  NODE_VERSION=$(node -v)
  if [ "$NODE_VERSION" != "v8.11.3" ]; then
    echo "Failed: Node version ($NODE_VERSION) is not v8.11.3"
    exit 1
  fi
'
