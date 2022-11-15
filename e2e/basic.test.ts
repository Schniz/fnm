import { writeFile, mkdir } from "node:fs/promises"
import { join } from "node:path"
import { script } from "./shellcode/script"
import { Bash, Fish, PowerShell, WinCmd, Zsh } from "./shellcode/shells"
import testCwd from "./shellcode/test-cwd"
import testNodeVersion from "./shellcode/test-node-version"
import describe from "./describe"

for (const shell of [Bash, Zsh, Fish, PowerShell, WinCmd]) {
  describe(shell, () => {
    test(`basic usage`, async () => {
      await script(shell)
        .then(shell.env({}))
        .then(shell.call("fnm", ["install", "v8.11.3"]))
        .then(shell.call("fnm", ["use", "v8.11.3"]))
        .then(testNodeVersion(shell, "v8.11.3"))
        .takeSnapshot(shell)
        .execute(shell)
    })

    test(`.nvmrc`, async () => {
      await writeFile(join(testCwd(), ".nvmrc"), "v8.11.3")
      await script(shell)
        .then(shell.env({}))
        .then(shell.call("fnm", ["install"]))
        .then(shell.call("fnm", ["use"]))
        .then(testNodeVersion(shell, "v8.11.3"))
        .takeSnapshot(shell)
        .execute(shell)
    })

    test(`.node-version`, async () => {
      await writeFile(join(testCwd(), ".node-version"), "v8.11.3")
      await script(shell)
        .then(shell.env({}))
        .then(shell.call("fnm", ["install"]))
        .then(shell.call("fnm", ["use"]))
        .then(testNodeVersion(shell, "v8.11.3"))
        .takeSnapshot(shell)
        .execute(shell)
    })

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

    test(`resolves partial semver`, async () => {
      await script(shell)
        .then(shell.env({}))
        .then(shell.call("fnm", ["install", "6"]))
        .then(shell.call("fnm", ["use", "6"]))
        .then(testNodeVersion(shell, "v6.17.1"))
        .takeSnapshot(shell)
        .execute(shell)
    })

    test("`fnm ls` with nothing installed", async () => {
      await script(shell)
        .then(shell.env({}))
        .then(
          shell.hasCommandOutput(
            shell.call("fnm", ["ls"]),
            "* system",
            "fnm ls"
          )
        )
        .takeSnapshot(shell)
        .execute(shell)
    })
  })
}
