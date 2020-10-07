#!/bin/bash

if [ "$1" = "" ]; then
  echo "No version provided, using 'Unreleased'" >&2
  NEXT_VERSION="Unreleased"
else
  NEXT_VERSION="$1"
fi

echo "Generating changelog for $NEXT_VERSION"

lerna-changelog --from=v1.0.0 "--next-version=$NEXT_VERSION" >CHANGELOG.md
prettier --write CHANGELOG.md
