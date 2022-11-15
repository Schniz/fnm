import getStderr from "./shellcode/get-stderr"
import { script } from "./shellcode/script"
import { Bash, Fish, PowerShell, Zsh } from "./shellcode/shells"

for (const shell of [Bash, Zsh, Fish, PowerShell]) {
  describe(shell.name(), () => {
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
