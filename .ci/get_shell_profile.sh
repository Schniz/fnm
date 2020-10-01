#!/bin/bash

OS="$(uname -s)"

case $1 in
  "fish")
    CONFIG_DIR="$(fish -c 'echo -n $__fish_config_dir')"
    echo "${CONFIG_DIR-"$HOME/.config/fish"}/fish.config"
    ;;
  "zsh")
    echo "$HOME/.zshrc"
    ;;
  "bash")
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
