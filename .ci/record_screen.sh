#!/bin/bash

DIRECTORY="$(dirname "$0")"

function setup_binary() {
  TEMP_DIR="/tmp/fnm-$(date '+%s')"
  mkdir "$TEMP_DIR"
  cp ./target/release/fnm "$TEMP_DIR/fnm"
  export PATH=$TEMP_DIR:$PATH
  export FNM_DIR=$TEMP_DIR/.fnm

  # First run of the binary might be slower due to anti-virus software
  echo "Using $(which fnm)"
  echo "  with version $(fnm --version)"
}

setup_binary

RECORDING_PATH=$DIRECTORY/screen_recording

(rm -rf "$RECORDING_PATH" &> /dev/null || true)

asciinema rec -c "$DIRECTORY/recorded_screen_script.sh" "$RECORDING_PATH"
sed "s@$TEMP_DIR@~@g" "$RECORDING_PATH" | \
  svg-term \
    --window \
    --out "docs/fnm.svg" \
    --height=17 \
    --width=70
