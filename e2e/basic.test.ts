import { writeFile } from "node:fs/promises"
import { join } from "node:path"
import { script } from "./shellcode/script.js"
import { Bash, Fish, PowerShell, WinCmd, Zsh } from "./shellcode/shells.js"
import testCwd from "./shellcode/test-cwd.js"
import testNodeVersion from "./shellcode/test-node-version.js"
import describe from "./describe.js"

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

    test(`package.json engines.node`, async () => {
      await writeFile(
        join(testCwd(), "package.json"),
        JSON.stringify({ engines: { node: "8.11.3" } }),
      )
      await script(shell)
        .then(shell.env({ resolveEngines: true }))
        .then(shell.call("fnm", ["install"]))
        .then(shell.call("fnm", ["use"]))
        .then(testNodeVersion(shell, "v8.11.3"))
        .takeSnapshot(shell)
        .execute(shell)
    })

    test(`package.json engines.node with semver range`, async () => {
      await writeFile(
        join(testCwd(), "package.json"),
        JSON.stringify({ engines: { node: "^6 < 6.17.1" } }),
      )
      await script(shell)
        .then(shell.env({ resolveEngines: true }))
        .then(shell.call("fnm", ["install"]))
        .then(shell.call("fnm", ["use"]))
        .then(testNodeVersion(shell, "v6.17.0"))
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
            "fnm ls",
          ),
        )
        .takeSnapshot(shell)
        .execute(shell)
    })

    test(`when .node-version and .nvmrc are in sync, it throws no error`, async () => {
      await writeFile(join(testCwd(), ".nvmrc"), "v11.10.0")
      await writeFile(join(testCwd(), ".node-version"), "v11.10.0")

      await script(shell)
        .then(shell.env({}))
        .then(shell.call("fnm", ["install"]))
        .then(shell.call("fnm", ["use"]))
        .then(testNodeVersion(shell, "v11.10.0"))
        .takeSnapshot(shell)
        .execute(shell)
    })
  })
}
