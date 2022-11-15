import { script } from "./shellcode/script"
import { Bash, Fish, PowerShell, Zsh } from "./shellcode/shells"
import describe from "./describe"
import { writeFile } from "node:fs/promises"
import path from "node:path"
import testCwd from "./shellcode/test-cwd"
import getStderr from "./shellcode/get-stderr"

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

    test(`errors when alias is not found`, async () => {
      await writeFile(path.join(testCwd(), ".node-version"), "lts/*")
      await script(shell)
        .then(shell.env({ logLevel: "error" }))
        .then(
          shell.scriptOutputContains(
            getStderr(shell.call("fnm", ["use"])),
            "'Requested version lts-latest is not'"
          )
        )
        .takeSnapshot(shell)
        .execute(shell)
    })

    test(`unalias a version`, async () => {
      await script(shell)
        .then(shell.env({}))
        .then(shell.call("fnm", ["install", "11.10.0"]))
        .then(shell.call("fnm", ["install", "8.11.3"]))
        .then(shell.call("fnm", ["alias", "8.11.3", "version8"]))
        .then(shell.scriptOutputContains(shell.call("fnm", ["ls"]), "version8"))
        .then(shell.call("fnm", ["unalias", "version8"]))
        .then(
          shell.scriptOutputContains(
            getStderr(shell.call("fnm", ["use", "version8"])),
            "'Requested version version8 is not currently installed'"
          )
        )
        .takeSnapshot(shell)
        .execute(shell)
    })

    test(`unalias errors if alias not found`, async () => {
      await script(shell)
        .then(shell.env({ logLevel: "error" }))
        .then(
          shell.scriptOutputContains(
            getStderr(shell.call("fnm", ["unalias", "lts"])),
            "'Requested alias lts not found'"
          )
        )
        .takeSnapshot(shell)
        .execute(shell)
    })
  })
}
