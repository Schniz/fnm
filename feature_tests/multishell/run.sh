#!/bin/bash

set -e

eval $(fnm env)

fnm install v8.11.3
fnm install v11.9.0

fnm use v8.11.3

bash -c '
  set -e
  eval $(fnm env --multi)
  fnm use v11.9.0
  echo "> verifying version v11.9.0 for child bash"
  if [ "$(node -v)" == "v11.9.0" ]; then
    echo "Okay!"
  else
    echo "Node version should be v11.9.0 in the bash fork"
    exit 1
  fi
'

echo "> verifying version v8.11.3 for parent bash"
if [ "$(node -v)" == "v8.11.3" ]; then
  echo "Okay!"
else
  echo "Node version should be v8.11.3 in the base bash"
  exit 1
fi
