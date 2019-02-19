#!/bin/bash

eval `fnm env --multi`

fnm install 6 # no new versions would be issued for this unsupported version
fnm install 8.11.3

fnm use 6
if [ "$(node -v)" != "v6.16.0" ]; then
  echo "Node version mismatch: $(node -v). Expected: v6.16.0"
fi

fnm use 8
if [ "$(node -v)" != "v8.11.3" ]; then
  echo "Node version mismatch: $(node -v). Expected: v8.11.3"
fi
