import { writeFile, mkdir } from "node:fs/promises"
import { join } from "node:path"
import { script } from "./shellcode/script.js"
import { Bash, Fish, PowerShell, WinCmd, Zsh } from "./shellcode/shells.js"
import testCwd from "./shellcode/test-cwd.js"
import testNodeVersion from "./shellcode/test-node-version.js"
import describe from "./describe.js"

for (const shell of [Bash, Zsh, Fish, PowerShell, WinCmd]) {
  describe(shell, () => {
    test(`use on cd`, async () => {
      await mkdir(join(testCwd(), "subdir"), { recursive: true })
      await writeFile(join(testCwd(), "subdir", ".node-version"), "v12.22.12")
      await script(shell)
        .then(shell.env({ useOnCd: true }))
        .then(shell.call("fnm", ["install", "v8.11.3"]))
        .then(shell.call("fnm", ["install", "v12.22.12"]))
        .then(shell.call("cd", ["subdir"]))
        .then(testNodeVersion(shell, "v12.22.12"))
        .takeSnapshot(shell)
        .execute(shell)
    })

    test(`with resolve-engines`, async () => {
      await mkdir(join(testCwd(), "subdir"), { recursive: true })
      await writeFile(
        join(testCwd(), "subdir", "package.json"),
        JSON.stringify({
          name: "hello",
          engines: {
            node: "v12.22.12",
          },
        }),
      )
      await script(shell)
        .then(shell.env({ useOnCd: true, resolveEngines: true }))
        .then(shell.call("fnm", ["install", "v8.11.3"]))
        .then(shell.call("fnm", ["install", "v12.22.12"]))
        .then(shell.call("cd", ["subdir"]))
        .then(testNodeVersion(shell, "v12.22.12"))
        .execute(shell)
    })
  })
}
