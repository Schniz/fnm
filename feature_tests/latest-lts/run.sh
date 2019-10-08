#!/bin/bash

set -e

eval "$(fnm env --multi)"

fnm install
fnm use

fnm ls | grep latest-
