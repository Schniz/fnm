#!/bin/bash

set -e

eval "$(fnm env --multi)"
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

    cd nvmrc

    NODE_VERSION=$(node -v)
    if [ "$NODE_VERSION" != "v8.11.3" ]; then
      echo "Failed: Node version ($NODE_VERSION) is not v8.11.3"
      exit 1
    fi

    fnm use 6.11.3
    cd ../dot\ node\ version

    NODE_VERSION=$(node -v)
    if [ "$NODE_VERSION" != "v8.11.3" ]; then
      echo "Failed: Node version ($NODE_VERSION) is not v8.11.3"
      exit 1
    fi
  '
else
  echo "Skipping zsh test: \`zsh\` is not installed"
fi

if hash fish 2>/dev/null; then
  echo ' > Running test on Fish'

  fish -c '
    fnm env --multi --use-on-cd | source

    fnm use 6.11.3

    set NODE_VERSION (node -v)
    if test "$NODE_VERSION" != "v6.11.3"
      echo "Failed: Node version ($NODE_VERSION) is not v6.11.3"
      exit 1
    end

    cd nvmrc

    set NODE_VERSION (node -v)
    if test "$NODE_VERSION" != "v8.11.3"
      echo "Failed: Node version ($NODE_VERSION) is not v8.11.3"
      exit 1
    end

    fnm use 6.11.3
    cd ../dot\ node\ version

    set NODE_VERSION (node -v)
    if test "$NODE_VERSION" != "v8.11.3"
      echo "Failed: Node version ($NODE_VERSION) is not v8.11.3"
      exit 1
    end
  '
else
  echo "Skipping fish test: \`zsh\` is not installed"
fi

echo " > Running test on Bash..."
bash -c '
  set -e
  shopt -s expand_aliases
  eval "`fnm env --multi --use-on-cd`"
  fnm use 6.11.3
  NODE_VERSION=$(node -v)
  if [ "$NODE_VERSION" != "v6.11.3" ]; then
    echo "Failed: Node version ($NODE_VERSION) is not v6.11.3"
    exit 1
  fi
  cd nvmrc
  NODE_VERSION=$(node -v)
  if [ "$NODE_VERSION" != "v8.11.3" ]; then
    echo "Failed: Node version ($NODE_VERSION) is not v8.11.3"
    exit 1
  fi
  fnm use 6.11.3
  cd ../dot\ node\ version
  NODE_VERSION=$(node -v)
  if [ "$NODE_VERSION" != "v8.11.3" ]; then
    echo "Failed: Node version ($NODE_VERSION) is not v8.11.3"
    exit 1
  fi
'
