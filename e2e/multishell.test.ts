import { script } from "./shellcode/script"
import { Bash, Fish, PowerShell, Zsh } from "./shellcode/shells"
import testNodeVersion from "./shellcode/test-node-version"

for (const shell of [Bash, Zsh, Fish, PowerShell]) {
  describe(shell.name(), () => {
    test(`multishell changes don't affect parent`, async () => {
      await script(shell)
        .then(shell.env({}))
        .then(shell.call("fnm", ["install", "v8.11.3"]))
        .then(shell.call("fnm", ["install", "v11.9.0"]))
        .then(
          shell.inSubShell(
            script(shell)
              .then(shell.env({}))
              .then(shell.call("fnm", ["use", "v11"]))
              .then(testNodeVersion(shell, "v11.9.0"))
              .asLine()
          )
        )
        .then(testNodeVersion(shell, "v8.11.3"))
        .takeSnapshot(shell)
        .execute(shell)
    })
  })
}
