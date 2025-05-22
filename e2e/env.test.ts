import { readFile } from "node:fs/promises"
import { join } from "node:path"
import { script } from "./shellcode/script.js"
import { Bash, Fish, PowerShell, WinCmd, Zsh } from "./shellcode/shells.js"
import testCwd from "./shellcode/test-cwd.js"
import describe from "./describe.js"

for (const shell of [Bash, Zsh, Fish, PowerShell, WinCmd]) {
  describe(shell, () => {
    test(`outputs json`, async () => {
      const filename = `file.json`
      await script(shell)
        .then(
          shell.redirectOutput(shell.call("fnm", ["env", "--json"]), {
            output: filename,
          }),
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
          FNM_RESOLVE_ENGINES: "true",
          FNM_COREPACK_ENABLED: "false",
          FNM_VERSION_FILE_STRATEGY: "local",
        })
      }
    })
  })
}
