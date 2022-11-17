import { script } from "./shellcode/script.js"
import { Bash, Fish, PowerShell, Zsh } from "./shellcode/shells.js"
import describe from "./describe.js"

for (const shell of [Bash, Zsh, Fish, PowerShell]) {
  describe(shell, () => {
    test(`installs latest lts`, async () => {
      await script(shell)
        .then(shell.env({}))
        .then(shell.call("fnm", ["install", "--lts"]))
        .then(
          shell.scriptOutputContains(shell.call("fnm", ["ls"]), "lts-latest")
        )
        .then(shell.call("fnm", ["use", "'lts/*'"]))
        .takeSnapshot(shell)
        .execute(shell)
    })
  })
}
