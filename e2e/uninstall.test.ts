import { script } from "./shellcode/script.js"
import { Bash, Fish, PowerShell, Zsh } from "./shellcode/shells.js"
import describe from "./describe.js"

for (const shell of [Bash, Zsh, Fish, PowerShell]) {
  describe(shell, () => {
    test(`uninstalls a version`, async () => {
      await script(shell)
        .then(shell.call("fnm", ["i", "12.0.0"]))
        .then(shell.call("fnm", ["alias", "12.0.0", "hello"]))
        .then(
          shell.scriptOutputContains(
            shell.scriptOutputContains(shell.call("fnm", ["ls"]), "v12.0.0"),
            "hello"
          )
        )
        .then(shell.call("fnm", ["uni", "hello"]))
        .then(
          shell.hasCommandOutput(
            shell.call("fnm", ["ls"]),
            "* system",
            "fnm ls"
          )
        )
        .takeSnapshot(shell)
        .execute(shell)
    })
  })
}
