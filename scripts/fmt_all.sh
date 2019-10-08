#!/bin/bash

DIRECTORY=$(dirname "$0")

"$DIRECTORY/fmt_base.sh" --in-place ./*/*.re
