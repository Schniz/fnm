import { script } from "./shellcode/script.js"
import { Bash, Fish, PowerShell, WinCmd, Zsh } from "./shellcode/shells.js"
import describe from "./describe.js"

for (const shell of [Bash, Zsh, Fish, PowerShell, WinCmd]) {
  describe(shell, () => {
    test(`current returns the current Node.js version set in fnm`, async () => {
      await script(shell)
        .then(shell.env({}))
        .then(
          shell.hasCommandOutput(
            shell.call("fnm", ["current"]),
            "none",
            "currently activated version"
          )
        )
        .then(shell.call("fnm", ["install", "v8.11.3"]))
        .then(shell.call("fnm", ["install", "v10.10.0"]))
        .then(shell.call("fnm", ["use", "v8.11.3"]))
        .then(
          shell.hasCommandOutput(
            shell.call("fnm", ["current"]),
            "v8.11.3",
            "currently activated version"
          )
        )
        .then(shell.call("fnm", ["use", "v10.10.0"]))
        .then(
          shell.hasCommandOutput(
            shell.call("fnm", ["current"]),
            "v10.10.0",
            "currently activated version"
          )
        )
        .then(shell.call("fnm", ["use", "system"]))
        .then(
          shell.hasCommandOutput(
            shell.call("fnm", ["current"]),
            "system",
            "currently activated version"
          )
        )
        .takeSnapshot(shell)
        .execute(shell)
    })
  })
}
