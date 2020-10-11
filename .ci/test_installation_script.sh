#!/bin/bash

set -e

DIRECTORY="$(dirname "$0")"
SHELL_TO_RUN="$1"
PROFILE_FILE="$("$DIRECTORY/get_shell_profile.sh" "$SHELL_TO_RUN")"

ls -lah ~
echo "---"
echo "Profile is $PROFILE_FILE"
echo "---"
cat "$PROFILE_FILE"
echo "---"
echo "PATH=$PATH"
echo "---"

$SHELL_TO_RUN -c "
  . $PROFILE_FILE
  fnm --version
"

$SHELL_TO_RUN -c "
  . $PROFILE_FILE
  fnm install 12.5.0
  fnm ls | grep 12.5.0

  echo 'fnm ls worked.'
"

$SHELL_TO_RUN -c "
  . $PROFILE_FILE
  fnm use 12.5.0
  node --version | grep 12.5.0

  echo 'node --version worked.'
"
