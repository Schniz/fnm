#!/bin/bash

set -e

eval $(fnm env --log-level=error)
ALIAS="$(fnm install 8.11.3 && (fnm alias 123 abc 2>&1 || true))"

if [ "$ALIAS" == "" ]; then
  echo "Expected the output to contain errors"
  exit 1
fi
