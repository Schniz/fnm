#!/bin/bash

echo "Building binary in docker"

docker build . -t schlez/fnm-static-binary

echo "Copying to ./fnm-linux"

docker run --rm -v $(pwd):$(pwd) --workdir $(pwd) schlez/fnm-static-binary cp /app/_build/default/executable/FnmApp.exe ./fnm-linux
