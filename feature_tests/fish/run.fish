#!/usr/bin/env fish

fnm env --fish | source

fnm install v8.11.3
fnm use v8.11.3

if test (node --version) != "v8.11.3"
  echo "Node version is not v8.11.3!"
  exit 1
end
