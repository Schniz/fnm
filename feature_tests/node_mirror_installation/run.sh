#!/bin/bash

eval `fnm env --node-dist-mirror="https://npm.taobao.org/dist"`

fnm install v8.11.3
fnm use v8.11.3

if [ "$(node -v)" != "v8.11.3" ]; then
  echo "Node version is not v8.11.3!"
  exit 1
fi
