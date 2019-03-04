#!/bin/bash

set -e

fnm install 6.11.3
fnm install 8.11.3

fnm alias 8.11.3 oldie
fnm alias 6.11.3 older

VERSIONS_INSTALLED=$(fnm ls)

echo "$VERSIONS_INSTALLED" | grep 8.11.3 | grep oldie
echo "$VERSIONS_INSTALLED" | grep 6.11.3 | grep older
