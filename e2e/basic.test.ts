import { writeFile } from "node:fs/promises"
import { join } from "node:path"
import { script } from "./shellcode/script"
import { Bash, Fish, PowerShell, Zsh } from "./shellcode/shells"
import testCwd from "./shellcode/test-cwd"
import testNodeVersion from "./shellcode/test-node-version"

for (const shell of [Bash, Zsh, Fish, PowerShell]) {
  describe(shell.name(), () => {
    test(`basic usage`, async () => {
      await script(shell)
        .then(shell.env({}))
        .then(shell.call("fnm", ["install", "v8.11.3"]))
        .then(shell.call("fnm", ["use", "v8.11.3"]))
        .then(testNodeVersion(shell, "v8.11.3"))
        .takeSnapshot(shell)
        .execute(shell)
    })

    test(`nvmrc`, async () => {
      await writeFile(join(testCwd(), ".nvmrc"), "v8.11.3")
      await script(shell)
        .then(shell.env({}))
        .then(shell.call("fnm", ["install"]))
        .then(shell.call("fnm", ["use"]))
        .then(testNodeVersion(shell, "v8.11.3"))
        .takeSnapshot(shell)
        .execute(shell)
    })
  })
}
