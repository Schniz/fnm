import { cmdCall } from "./cmdCall"
import { cmdEnv } from "./cmdEnv"
import { cmdExpectCommandOutput } from "./expect-command-output"
import { redirectOutput } from "./redirect-output"
import { define, Shell } from "./types"

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
}

export const Zsh = {
  ...define<Shell>({
    binaryName: () => "zsh",
    currentlySupported: () => true,
    name: () => "Zsh",
    launchArgs: () => [],
    escapeText: (x) => x,
    dieOnErrors: () => `set -e`,
  }),
  ...cmdEnv.bash,
  ...cmdCall.all,
  ...cmdExpectCommandOutput.bash,
}

export const Fish = {
  ...define<Shell>({
    binaryName: () => "fish",
    currentlySupported: () => true,
    name: () => "Fish",
    launchArgs: () => [],
    escapeText: (x) => x,
    forceFile: true,
  }),
  ...cmdEnv.fish,
  ...cmdCall.all,
  ...redirectOutput.bash,
  ...cmdExpectCommandOutput.fish,
}

export const PowerShell = {
  ...define<Shell>({
    binaryName: () => {
      if (process.platform === "win32") {
        return "powershell.exe"
      } else {
        return "pwsh"
      }
    },
    currentlySupported: () => true,
    name: () => "PowerShell",
    launchArgs: () => ["-NoProfile"],
    escapeText: (x) => x,
    dieOnErrors: () => `$ErrorActionPreference = "Stop"`,
  }),
  ...cmdEnv.powershell,
  ...cmdCall.all,
  ...cmdExpectCommandOutput.powershell,
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
}
