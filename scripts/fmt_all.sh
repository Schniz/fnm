#!/bin/bash

DIRECTORY=$(basename "$0")

"$DIRECTORY/fmt_base.sh" --in-place ./*/*.re
