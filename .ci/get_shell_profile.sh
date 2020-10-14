#!/bin/bash

case $1 in
  "fish")
    echo "$HOME/.config/fish/conf.d/fnm.fish"
    ;;
  "zsh")
    echo "$HOME/.zshrc"
    ;;
  "bash")
    OS="$(uname -s)"
    if [ "$OS" = "Darwin" ]; then
      echo "$HOME/.profile"
    else
      echo "$HOME/.bashrc"
    fi
    ;;
  *)
    exit 1
    ;;
esac
