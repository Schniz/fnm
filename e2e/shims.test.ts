import { readFile } from "node:fs/promises"
import { join } from "node:path"
import { script } from "./shellcode/script.js"
import { Bash, Fish, PowerShell, WinCmd, Zsh } from "./shellcode/shells.js"
import testCwd from "./shellcode/test-cwd.js"
import describe from "./describe.js"
import testNodeVersion from "./shellcode/test-node-version.js"

for (const shell of [Bash, Zsh, Fish, PowerShell, WinCmd]) {
  describe(shell, () => {
    test(`outputs json`, async () => {
      const filename = `file.json`
      await script(shell)
        .then(
          shell.redirectOutput(
            shell.call("fnm", ["env", "--json", "--with-shims"]),
            {
              output: filename,
            }
          )
        )
        .takeSnapshot(shell)
        .execute(shell)

      if (shell.currentlySupported()) {
        const file = await readFile(join(testCwd(), filename), "utf8")
        expect(JSON.parse(file)).toEqual({
          FNM_ARCH: expect.any(String),
          FNM_DIR: expect.any(String),
          FNM_LOGLEVEL: "info",
          FNM_MULTISHELL_PATH: expect.any(String),
          FNM_NODE_DIST_MIRROR: expect.any(String),
          FNM_VERSION_FILE_STRATEGY: "local",
          FNM_VERSION_SWITCH_STRATEGY: "shims",
        })
      }
    })

    test(`runs Node through a shim`, async () => {
      await script(shell)
        .then(shell.env({ args: ["--with-shims"] }))
        .then(shell.call("fnm", ["install", "12.0.0"]))
        .then(shell.call("fnm", ["use", "12.0.0"]))
        .then(testNodeVersion(shell, "v12.0.0"))
        .takeSnapshot(shell)
        .execute(shell)
    })
  })
}
