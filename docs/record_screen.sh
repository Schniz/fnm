#!/bin/bash

DIRECTORY=`dirname $0`

function setup_binary() {
  export TEMP_DIR=$(mktemp -d -t fnm)
  cp _esy/default/build/default/executable/FnmApp.exe $TEMP_DIR/fnm
  export PATH=$TEMP_DIR:$PATH
  export FNM_DIR=$TEMP_DIR/.fnm
}

setup_binary

RECORDING_PATH=$DIRECTORY/screen_recording

(rm -rf $RECORDING_PATH &> /dev/null || true)

asciinema rec -c $DIRECTORY/recorded_screen_script.sh $RECORDING_PATH
cat $RECORDING_PATH | sed "s@$TEMP_DIR@~@g" | svg-term --window --out $DIRECTORY/fnm.svg --height=17 --width=70
