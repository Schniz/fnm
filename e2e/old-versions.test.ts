import { script } from "./shellcode/script.js"
import { Bash, Fish, PowerShell, WinCmd, Zsh } from "./shellcode/shells.js"
import testNodeVersion from "./shellcode/test-node-version.js"
import describe from "./describe.js"

for (const shell of [Bash, Zsh, Fish, PowerShell, WinCmd]) {
  describe(shell, () => {
    test(`download old Node.js 0.10.x`, async () => {
      await script(shell)
        .then(shell.env({}))
        .then(shell.call("fnm", ["install", "v0.10.36"]))
        .then(shell.call("fnm", ["use", "v0.10.36"]))
        .then(testNodeVersion(shell, "v0.10.36"))
        .takeSnapshot(shell)
        .execute(shell)
    })
  })
}
