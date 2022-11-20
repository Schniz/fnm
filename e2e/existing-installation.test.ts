import getStderr from "./shellcode/get-stderr.js"
import { script } from "./shellcode/script.js"
import { Bash, Fish, PowerShell, Zsh } from "./shellcode/shells.js"
import describe from "./describe.js"

for (const shell of [Bash, Zsh, Fish, PowerShell]) {
  describe(shell, () => {
    test(`warns about an existing installation`, async () => {
      await script(shell)
        .then(shell.env({}))
        .then(shell.call("fnm", ["install", "v8.11.3"]))
        .then(
          shell.scriptOutputContains(
            getStderr(shell.call("fnm", ["install", "v8.11.3"])),
            "'already installed'"
          )
        )
        .takeSnapshot(shell)
        .execute(shell)
    })
  })
}
