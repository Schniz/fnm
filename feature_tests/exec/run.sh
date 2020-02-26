#!/bin/bash

set -e

fnm install v6.10.0
fnm install v8.10.0
fnm install v10.10.0
fnm use v8.10.0

fnm exec -- node -v | grep "v8.10.0"
fnm exec --using 6 -- node -v | grep "v6.10.0"
fnm exec --using 10 -- node -v | grep "v10.10.0"
