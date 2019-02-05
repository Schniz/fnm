#!/bin/bash

set -e

DIRECTORY=`dirname $0`

if hash fish 2>/dev/null; then
  fish $DIRECTORY/run.fish
else
  echo "Skipping: \`fish\` is not installed"
fi
