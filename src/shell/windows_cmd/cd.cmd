@echo off
cd %1
if exist .nvmrc (
    fnm use
) else (
    if exist .node-version (
        fnm use
    )
)
@echo on