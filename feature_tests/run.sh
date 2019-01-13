#!/bin/bash

set -e

DIRECTORY=`dirname $0`
BINARY=$1
TEMP_DIR_BASE=$(pwd)/$DIRECTORY/.tmp
TEMP_BINARY_PATH=$TEMP_DIR_BASE/bin
TEMP_NSW_DIR=$TEMP_DIR_BASE/.nsw

if [ "$BINARY" == "" ]; then
  echo "No binary supplied!"
  exit 1
fi

echo "using nvm=$BINARY"

rm -rf $TEMP_DIR_BASE
mkdir $TEMP_DIR_BASE $TEMP_BINARY_PATH
cp $BINARY $TEMP_BINARY_PATH/nsw

for test_file in $DIRECTORY/*/run.sh; do
  rm -rf $TEMP_NSW_DIR

  echo "Running test in $test_file"
  echo "Running test in $test_file" | sed "s/./-/g"
  (cd $(dirname $test_file) && NSW_DIR=$TEMP_NSW_DIR PATH=$TEMP_BINARY_PATH:$PATH bash $(basename $test_file))
  echo ""
  echo " -> Finished!"

  rm -rf $TEMP_NSW_DIR
done
