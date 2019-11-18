#!/bin/bash

set -e

DIRECTORY="$(dirname "$0")"
BINARY="$1"
SPECIFIC_TEST="$2"
TEMP_DIR_BASE=$(pwd)/$DIRECTORY/.tmp
TEMP_BINARY_PATH=$TEMP_DIR_BASE/bin
TEMP_FNM_DIR=$TEMP_DIR_BASE/.fnm

if [ "$BINARY" == "" ]; then
  echo "No binary supplied!"
  exit 1
fi

echo "using fnm=$BINARY"

rm -rf "$TEMP_DIR_BASE"
mkdir "$TEMP_DIR_BASE" "$TEMP_BINARY_PATH"
cp "$BINARY" "$TEMP_BINARY_PATH/fnm"

run_test() {
  test_file="$1"
  rm -rf "$TEMP_FNM_DIR"

  TEST_BASENAME="$(basename "$test_file")"
  TEST_DIRNAME="$(dirname "$test_file")"

  echo "Running test in $test_file"
  echo "Running test in $test_file" | sed "s/./-/g"
  (cd "$TEST_DIRNAME" && FNM_DIR="$TEMP_FNM_DIR" PATH="$TEMP_BINARY_PATH:$PATH" bash "$TEST_BASENAME")
  echo ""
  echo " -> Finished!"

  rm -rf "$TEMP_FNM_DIR"
}

if [ "$SPECIFIC_TEST" == "" ]; then
  for test_file in "$DIRECTORY"/*/run.sh; do
    run_test "$test_file"
  done
else
  run_test "$SPECIFIC_TEST"
fi
