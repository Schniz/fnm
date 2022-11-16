import { script } from "./shellcode/script"
import { Bash, Fish, PowerShell, Zsh } from "./shellcode/shells"
import fs from "node:fs/promises"
import path from "node:path"
import describe from "./describe"
import testCwd from "./shellcode/test-cwd"

for (const shell of [Bash, Fish, PowerShell, Zsh]) {
  describe(shell, () => {
    test(`uses .nvmrc with lts definition`, async () => {
      await fs.writeFile(path.join(testCwd(), ".nvmrc"), `lts/dubnium`)

      await script(shell)
        .then(shell.env({}))
        .then(shell.call("fnm", ["install"]))
        .then(shell.call("fnm", ["use"]))
        .then(
          shell.scriptOutputContains(shell.call("fnm", ["ls"]), "lts-dubnium")
        )
        .takeSnapshot(shell)
        .execute(shell)
    })
  })
}
