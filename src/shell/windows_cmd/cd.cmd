@echo off
cd %1
if "%FNM_VERSION_FILE_STRATEGY%" == "recursive" (
  fnm use --silent-when-unchanged
) else (
  if exist .nvmrc (
    fnm use --silent-when-unchanged
  ) else (
    if exist .node-version (
      fnm use --silent-when-unchanged
    )
  )
)
@echo on
