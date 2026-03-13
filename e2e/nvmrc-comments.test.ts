import { writeFile } from "node:fs/promises"
import { join } from "node:path"
import { script } from "./shellcode/script.js"
import { Bash, Fish, PowerShell, WinCmd, Zsh } from "./shellcode/shells.js"
import testCwd from "./shellcode/test-cwd.js"
import testNodeVersion from "./shellcode/test-node-version.js"
import describe from "./describe.js"

for (const shell of [Bash, Zsh, Fish, PowerShell, WinCmd]) {
  describe(shell, () => {
    test(`.nvmrc with comment on first line`, async () => {
      await writeFile(join(testCwd(), ".nvmrc"), "# comment\nv8.11.3")
      await script(shell)
        .then(shell.env({}))
        .then(shell.call("fnm", ["install"]))
        .then(shell.call("fnm", ["use"]))
        .then(testNodeVersion(shell, "v8.11.3"))
        .takeSnapshot(shell)
        .execute(shell)
    })

    test(`.nvmrc with inline comment after version`, async () => {
      await writeFile(join(testCwd(), ".nvmrc"), "v8.11.3 # this is Node 8 LTS")
      await script(shell)
        .then(shell.env({}))
        .then(shell.call("fnm", ["install"]))
        .then(shell.call("fnm", ["use"]))
        .then(testNodeVersion(shell, "v8.11.3"))
        .takeSnapshot(shell)
        .execute(shell)
    })

    test(`.nvmrc with multiple comment lines`, async () => {
      await writeFile(
        join(testCwd(), ".nvmrc"),
        "# First comment\n# Second comment\n\nv8.11.3\n# Trailing comment",
      )
      await script(shell)
        .then(shell.env({}))
        .then(shell.call("fnm", ["install"]))
        .then(shell.call("fnm", ["use"]))
        .then(testNodeVersion(shell, "v8.11.3"))
        .takeSnapshot(shell)
        .execute(shell)
    })

    test(`.nvmrc with empty lines and comments`, async () => {
      await writeFile(
        join(testCwd(), ".nvmrc"),
        "\n\n# comment\n\nv8.11.3\n\n# another comment",
      )
      await script(shell)
        .then(shell.env({}))
        .then(shell.call("fnm", ["install"]))
        .then(shell.call("fnm", ["use"]))
        .then(testNodeVersion(shell, "v8.11.3"))
        .takeSnapshot(shell)
        .execute(shell)
    })

    test(`.nvmrc with whitespace and inline comments`, async () => {
      await writeFile(
        join(testCwd(), ".nvmrc"),
        "  # comment with spaces  \n  v8.11.3  # inline comment  ",
      )
      await script(shell)
        .then(shell.env({}))
        .then(shell.call("fnm", ["install"]))
        .then(shell.call("fnm", ["use"]))
        .then(testNodeVersion(shell, "v8.11.3"))
        .takeSnapshot(shell)
        .execute(shell)
    })

    test(`.node-version should not strip comments`, async () => {
      await writeFile(join(testCwd(), ".node-version"), "v8.11.3")
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
