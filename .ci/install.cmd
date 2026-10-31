@echo off

setlocal enabledelayedexpansion

set INSTALL_DIR=%userprofile%\.local\bin
set SKIP_SHELL=false
set RELEASE=latest

:parse_args
if "%1"=="" goto :main
if "%1"=="-d" (
    set INSTALL_DIR=%2
    shift 
    shift 
) 
if "%1"=="--install-dir" (
    set INSTALL_DIR=%2
    shift 
    shift 
    goto :parse_args
) 
if "%1"=="-s" (
    set SKIP_SHELL=true
    shift 
    goto :parse_args
) 
if "%1"=="--skip-shell" (
    set SKIP_SHELL=true
    shift 
    goto :parse_args
) 
if "%1"=="-r" (
    set RELEASE=%2
    shift 
    shift 
    goto :parse_args
) 
if "%1"=="--release" (
    set RELEASE=%2
    shift 
    shift 
    goto :parse_args
) 
echo Unrecognised argument: `%1`
goto :eof

:check_dependencies
echo Checking dependencies for the installation script...

echo Checking availability of curl...
where curl 2>&1> NUL
if %errorlevel%==0 (
    echo OK!
) else (
    echo Missing!
    set SHOULD_EXIT=true
)

echo Checking availability of powershell...
where powershell 2>&1> NUL
if %errorlevel%==0 (
    echo OK!
) else (
    echo Missing!
    set SHOULD_EXIT=true
)

if "%SHOULD_EXIT%"=="true" (
    echo Not installing fnm due to missing dependencies.
    goto :eof
) 
exit /b

:download_fnm
if "%RELEASE%"=="latest" (
    set URL="https://github.com/Schniz/fnm/releases/latest/download/fnm-windows.zip"
) else (
    set URL="https://github.com/Schniz/fnm/releases/download/%RELEASE%/fnm-windows.zip"
)
echo Downloading %URL%

if not exist %INSTALL_DIR% mkdir %INSTALL_DIR%

curl --progress-bar --fail -L "%URL%" -o "%temp%\fnm-windows.zip"
if not %errorlevel%==0 (
    echo Download failed.  Check that the release/filename are correct.
    goto :eof
)

echo Extracting zip from %temp%\fnm-windows.zip
powershell Expand-Archive -Path "$env:temp\fnm-windows.zip" -DestinationPath "$env:temp" -Force
if exist %temp%\fnm.exe (
    move "%temp%\fnm.exe" "%INSTALL_DIR%" 2>&1>NUL
) else (
    move "%temp%\fnm-windows\fnm.exe" "%INSTALL_DIR%" 2>&1>NUL
)
exit /b

:setup_shells
echo Adding %userprofile%\profile.cmd as startup script for cmd
reg add "HKCU\Software\Microsoft\Command Processor" /v Autorun /d "call %userprofile%\profile.cmd" /f 

if not exist %userprofile%\profile.cmd (
    echo @echo off > %userprofile%\profile.cmd
    echo We have just created this file for you
)

:: This sets the **USER** path to include ~/.local/bin *persistent* across sessions
powershell "$current = (Get-ItemProperty -Path 'HKCU:\Environment' -Name Path).Path; if (-not $current.Contains($env:userprofile + '\.local\bin')) { [Environment]::SetEnvironmentVariable('Path', $current + ';' + $env:userprofile + '\.local\bin;', 'User') }"

:: We still need to do this for the *current* session, though.
:: The above only takes affect from the next new...
set path=%PATH%;%userprofile%\.local\bin


echo Adding FNM section to %userprofile%\profile.cmd

echo. >> %userprofile%\profile.cmd
echo :: FNM >> %userprofile%\profile.cmd
echo :: for /F will launch a new instance of cmd so we create a guard to prevent an infnite loop >> %userprofile%\profile.cmd
echo if not defined FNM_AUTORUN_GUARD ( >> %userprofile%\profile.cmd
echo     set "FNM_AUTORUN_GUARD=AutorunGuard" >> %userprofile%\profile.cmd
echo     FOR /f "tokens=*" %%%%z IN ('fnm env --use-on-cd') DO CALL %%%%z >> %userprofile%\profile.cmd
echo ) >> %userprofile%\profile.cmd

echo Setting up powershell

:: There's no downside to "force creating" a directory, it does nothing if it already exists...
powershell New-Item -Type Directory "$((Get-Item $PROFILE).Directory.FullName)" -Force 2>&1>NUL
powershell Add-Content -Path $PROFILE -Value '', '# FNM', 'fnm env --use-on-cd --shell powershell ^| Out-String ^| Invoke-Expression' -Encoding UTF8

exit /b

:main
echo Install dir: %INSTALL_DIR%
echo Skip Shell : %SKIP_SHELL%
echo Release    : %RELEASE%
call :check_dependencies
call :download_fnm
if not "%SKIP_SHELL%"=="true" (
    call :setup_shells
)

goto :eof
