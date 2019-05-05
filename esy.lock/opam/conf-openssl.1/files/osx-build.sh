#!/bin/sh

# Try just executing pkg-config openssl before looking
# explicitly for Nix -> Homebrew -> MacPorts.
# This handles the case where the user has set
# PKG_CONFIG_PATH themselves.
res=$(pkg-config openssl)
if [ $? -eq 0 ]; then
    echo $res
    exit 0
fi

if [ -e "$HOME/.nix-profile/lib/pkgconfig/openssl.pc" ]; then
  # Nix on macOS
  res=$(env PKG_CONFIG_PATH=$HOME/.nix-profile/lib/pkgconfig pkg-config openssl)
  if [ $? -eq 0 ]; then
    echo $res
    exit 0
  fi
fi

if [ -e "/usr/local/opt/openssl/lib/pkgconfig/openssl.pc" ]; then
  # Homebrew
  res=$(env PKG_CONFIG_PATH=/usr/local/opt/openssl/lib/pkgconfig pkg-config openssl)
  if [ $? -eq 0 ]; then
    echo $res
    exit 0
  fi
fi

# MacPorts
PKG_CONFIG_PATH=/opt/local/lib/pkgconfig pkg-config openssl
