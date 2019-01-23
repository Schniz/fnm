#!/bin/zsh

set -e

GAL_PROMPT_PREFIX='\e[34m✡ \e[0m'

function type() {
  printf $GAL_PROMPT_PREFIX
  echo $* | pv -qL $[10+(-2 + RANDOM%5)]
}

cd ./feature_tests/nvmrc

type 'eval `fnm env`'
eval `fnm env`

type 'fnm --version'
fnm --version

type 'cat .nvmrc'
cat .nvmrc

type 'fnm install'
fnm install

type 'fnm use'
fnm use

type 'node -v'
node -v

sleep 2
echo ""