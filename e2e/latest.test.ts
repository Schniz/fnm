import { script } from "./shellcode/script.js"
import { Bash, Fish, PowerShell, Zsh } from "./shellcode/shells.js"
import describe from "./describe.js"

for (const shell of [Bash, Zsh, Fish, PowerShell]) {
  describe(shell, () => {
    test(`installs latest`, async () => {
      await script(shell)
        .then(shell.env({}))
        .then(shell.call("fnm", ["install", "--latest"]))
        .then(
          shell.scriptOutputContains(shell.call("fnm", ["ls"]), "latest")
        )
        .then(shell.call("fnm", ["use", "'latest'"]))
        .takeSnapshot(shell)
        .execute(shell)
    })
  })
}
