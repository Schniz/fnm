#!/usr/bin/env powershell

param(
  [string]$version = "latest",
  [string]$installDir = "$env:USERPROFILE\.fnm",
  [switch]$skipShell = $false
)

$ErrorActionPreference = 'Stop'

# if version is not latest
if (!$version.Equals("latest")) {
  # and it does not start with a v
  if (!$version.StartsWith("v")) {
    # append a v in front
    $version = "v$version"
  }
}
$Target = 'fnm-windows.zip'

$Url = if ($version.Equals("latest")) {
  "https://github.com/Schniz/fnm/releases/latest/download/$Target"
} else {
  "https://github.com/Schniz/fnm/releases/download/$version/$Target"
}

# If installation dir does not exist, create it
if (!(Test-Path $installDir)) {
  New-Item $installDir -ItemType Directory | Out-Nul
}

Write-Output "Downloading from $Url"

Invoke-RestMethod $Url -OutFile "$installDir\fnm.zip"

Write-Output "Extracting file to $installDir"

Expand-Archive "$installDir\fnm.zip" -DestinationPath "$installDir" -Force

# Delete the zip file
Remove-Item "$installDir\fnm.zip"

if (!$skipShell) {
  Write-Output "Appending 'fnm env --use-on-cd | Out-String | Invoke-Expression' to profile"
  Add-Content "$profile" "fnm env --use-on-cd | Out-String | Invoke-Expression"
}