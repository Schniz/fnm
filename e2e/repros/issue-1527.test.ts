import { script } from "../shellcode/script.js"
import { Bash, Fish, PowerShell, Zsh } from "../shellcode/shells.js"
import describe from "../describe.js"

for (const shell of [Bash, Zsh, Fish, PowerShell]) {
  describe(shell, () => {
    test(`issue #1527: global package remains available after uninstall on older node`, async () => {
      await script(shell)
        .then(shell.env({}))
        .then(shell.call("fnm", ["install", "16"]))
        .then(shell.call("fnm", ["install", "18"]))
        .then(shell.call("fnm", ["use", "16"]))
        .then(shell.call("npm", ["install", "-g", "cowsay"]))
        .then(shell.call("fnm", ["use", "18"]))
        .then(shell.call("npm", ["install", "-g", "cowsay"]))
        .then(shell.call("fnm", ["use", "16"]))
        .then(shell.call("npm", ["uninstall", "-g", "cowsay"]))
        .then(
          shell.scriptOutputContains(shell.call("cowsay", ["hello"]), "hello"),
        )
        .execute(shell)
    })
  })
}
