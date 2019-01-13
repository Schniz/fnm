#!/bin/bash

eval $(nsw env)
nsw install
nsw use

if [ "$(node --version)" != "v10.9.0" ]; then
  echo "Node version is not v10.9.0!"
  exit 1
fi