#!/bin/bash

echo "Building binary in docker"

docker build . -t schlez/nsw-static-binary

echo "Copying to ./nsw-linux"

docker run --rm -v $(pwd):$(pwd) --workdir $(pwd) schlez/nsw-static-binary cp /app/_build/default/executable/NswApp.exe ./nsw-linux
