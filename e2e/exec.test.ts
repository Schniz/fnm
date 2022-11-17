import { script } from "./shellcode/script.js"
import { Bash, Fish, PowerShell, WinCmd, Zsh } from "./shellcode/shells.js"
import testCwd from "./shellcode/test-cwd.js"
import fs from "node:fs/promises"
import path from "node:path"
import describe from "./describe.js"

for (const shell of [Bash, Zsh, Fish, PowerShell, WinCmd]) {
  describe(shell, () => {
    test("`exec` usage", async () => {
      await fs.writeFile(path.join(testCwd(), ".nvmrc"), "v8.10.0")

      await script(shell)
        .then(shell.call("fnm", ["install"]))
        .then(shell.call("fnm", ["install", "v6.10.0"]))
        .then(shell.call("fnm", ["install", "v10.10.0"]))
        .then(
          shell.hasCommandOutput(
            shell.call("fnm", ["exec", "--", "node", "-v"]),
            "v8.10.0",
            "version file exec"
          )
        )
        .then(
          shell.hasCommandOutput(
            shell.call("fnm", ["exec", "--using=6", "--", "node", "-v"]),
            "v6.10.0",
            "exec:6 node -v"
          )
        )
        .then(
          shell.hasCommandOutput(
            shell.call("fnm", ["exec", "--using=10", "--", "node", "-v"]),
            "v10.10.0",
            "exec:6 node -v"
          )
        )
        .takeSnapshot(shell)
        .execute(shell)
    })
  })
}
