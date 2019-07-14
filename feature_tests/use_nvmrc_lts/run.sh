#!/bin/bash

set -e

eval "$(fnm env)"
fnm install
fnm use

fnm ls | grep lts-dubnium
