#!/bin/bash

set -e

export PATH=$PATH_ADDITION:$PATH

GAL_PROMPT_PREFIX="\e[34m✡\e[m  "

function type() {
  printf $GAL_PROMPT_PREFIX
  echo -n " "
  echo $* | node .ci/type-letters.js
}

type 'eval "$(fnm env)"'
eval "$(fnm env)"

type 'fnm --version'
fnm --version

type 'cat .node-version'
cat .node-version

type 'fnm install'
fnm install

type 'fnm use'
fnm use

type 'node -v'
node -v

sleep 2
echo ""
