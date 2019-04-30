#!/bin/bash

set -e

eval $(fnm env --log-level=quiet)
INSTALL="$(fnm install v8.11.3 && fnm use v8.11.3 && fnm alias v8.11.3 something)"

OUTPUT="$INSTALL"
if [ "$OUTPUT" != "" ]; then
  echo "Expected the output to be empty, instead got:"
  echo $OUTPUT
  exit 1
fi
