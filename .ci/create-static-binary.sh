#!/bin/bash

echo "Building binary in docker"

docker build . -t schlez/fnm-static-binary

echo "Copying to ./fnm"

docker run --rm -v $(pwd):$(pwd) --workdir $(pwd) schlez/fnm-static-binary cp /app/_esy/default/build/default/executable/FnmApp.exe ./fnm
strip ./fnm
