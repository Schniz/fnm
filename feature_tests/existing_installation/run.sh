#!/bin/bash

set -e

eval `fnm env`

echo "> Installing for the first time..."
fnm install v8.11.3
echo "> Installing the second time..."
fnm install v8.11.3 2>&1 >/dev/null | grep "already installed"

if [ "$?" != "0" ]; then
  echo "The second installation should say it is already installed"
  exit 1
fi
