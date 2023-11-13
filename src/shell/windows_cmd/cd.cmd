@echo off
cd %*
if "%FNM_VERSION_FILE_STRATEGY%" == "recursive" (
  fnm use --silent-if-unchanged
) else (
  if exist .nvmrc (
    fnm use --silent-if-unchanged
  ) else (
    if exist .node-version (
      fnm use --silent-if-unchanged
    )
  )
)
@echo on
