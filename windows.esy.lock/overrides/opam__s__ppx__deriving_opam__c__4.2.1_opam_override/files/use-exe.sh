#!/bin/bash

if [ "$1" == "windows" ]; then
    sed -i"" 's/ppx_deriving\/ppx_deriving/ppx_deriving\/ppx_deriving.exe/' $2;
else
    true
fi