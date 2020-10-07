#!/bin/bash

if [ "$1" = "" ]; then
  echo "Taking version from binary" >&2
  NEXT_VERSION="$(cargo run --quiet -- --version)"
else
  NEXT_VERSION="$1"
fi

echo "Generating changelog for $NEXT_VERSION"

lerna-changelog --from=v1.0.0 "--next-version=$NEXT_VERSION" >CHANGELOG.md
prettier --write CHANGELOG.md
