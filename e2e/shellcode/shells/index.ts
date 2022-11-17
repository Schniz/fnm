import { cmdCall } from "./cmdCall.js"
import { cmdEnv } from "./cmdEnv.js"
import { cmdExpectCommandOutput } from "./expect-command-output.js"
import { cmdHasOutputContains } from "./output-contains.js"
import { redirectOutput } from "./redirect-output.js"
import { cmdInSubShell } from "./sub-shell.js"
import { define, Shell } from "./types.js"

export const Bash = {
  ...define<Shell>({
    binaryName: () => "bash",
    currentlySupported: () => true,
    name: () => "Bash",
    launchArgs: () => ["-i"],
    escapeText: (x) => x,
    dieOnErrors: () => `set -e`,
  }),
  ...cmdEnv.bash,
  ...cmdCall.all,
  ...redirectOutput.bash,
  ...cmdExpectCommandOutput.bash,
  ...cmdHasOutputContains.bash,
  ...cmdInSubShell.bash,
}

export const Zsh = {
  ...define<Shell>({
    binaryName: () => "zsh",
    currentlySupported: () => process.platform !== "win32",
    name: () => "Zsh",
    launchArgs: () => [],
    escapeText: (x) => x,
    dieOnErrors: () => `set -e`,
  }),
  ...cmdEnv.bash,
  ...cmdCall.all,
  ...redirectOutput.bash,
  ...cmdExpectCommandOutput.bash,
  ...cmdHasOutputContains.bash,
  ...cmdInSubShell.zsh,
}

export const Fish = {
  ...define<Shell>({
    binaryName: () => "fish",
    currentlySupported: () => process.platform !== "win32",
    name: () => "Fish",
    launchArgs: () => [],
    escapeText: (x) => x,
    forceFile: true,
  }),
  ...cmdEnv.fish,
  ...cmdCall.all,
  ...redirectOutput.bash,
  ...cmdExpectCommandOutput.fish,
  ...cmdHasOutputContains.fish,
  ...cmdInSubShell.fish,
}

export const PowerShell = {
  ...define<Shell>({
    binaryName: () => "pwsh",
    forceFile: ".ps1",
    currentlySupported: () => true,
    name: () => "PowerShell",
    launchArgs: () => ["-NoProfile"],
    escapeText: (x) => x,
    dieOnErrors: () => `$ErrorActionPreference = "Stop"`,
  }),
  ...cmdEnv.powershell,
  ...cmdCall.all,
  ...redirectOutput.powershell,
  ...cmdExpectCommandOutput.powershell,
  ...cmdHasOutputContains.powershell,
  ...cmdInSubShell.powershell,
}

export const WinCmd = {
  ...define<Shell>({
    binaryName: () => "cmd.exe",
    currentlySupported: () => process.platform === "win32",
    name: () => "Windows Command Prompt",
    launchArgs: () => [],
    escapeText: (str) =>
      str
        .replace(/\r/g, "")
        .replace(/\n/g, "^\n\n")
        .replace(/\%/g, "%%")
        .replace(/\|/g, "^|")
        .replace(/\(/g, "^(")
        .replace(/\)/g, "^)"),
  }),
  ...cmdEnv.wincmd,
  ...cmdCall.all,
  ...cmdExpectCommandOutput.wincmd,
  ...redirectOutput.bash,
}
