import { script } from "./shellcode/script"
import { Bash, Fish, PowerShell, Zsh } from "./shellcode/shells"
import describe from "./describe"
import { writeFile } from "node:fs/promises"
import path from "node:path"
import testCwd from "./shellcode/test-cwd"

for (const shell of [Bash, Zsh, Fish, PowerShell]) {
  describe(shell, () => {
    test(`allows to install an lts if version missing`, async () => {
      await writeFile(path.join(testCwd(), ".node-version"), "lts/*")
      await script(shell)
        .then(shell.env({}))
        .then(shell.call("fnm", ["use", "--install-if-missing"]))
        .then(
          shell.scriptOutputContains(shell.call("fnm", ["ls"]), "lts-latest")
        )
        .takeSnapshot(shell)
        .execute(shell)
    })
  })
}
