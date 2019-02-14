#!/bin/bash

eval `fnm env`

fnm install v8.11.3
fnm install v8.11.3 | grep "already installed"

if [ "$?" != "0" ]; then
  echo "The second installation should say it is already installed"
  exit 1
fi
