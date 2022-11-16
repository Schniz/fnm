import { rm, readFile } from "node:fs/promises"
import { join } from "node:path"
import { script } from "./shellcode/script"
import { Bash, Fish, PowerShell, WinCmd, Zsh } from "./shellcode/shells"
import testCwd from "./shellcode/test-cwd"
import describe from "./describe"

for (const shell of [Bash, Zsh, Fish, PowerShell, WinCmd]) {
  describe(shell, () => {
    test(`outputs json`, async () => {
      await rm(join(testCwd(), "file.json"), { recursive: true, force: true })

      await script(shell)
        .then(
          shell.redirectOutput(shell.call("fnm", ["env", "--json"]), {
            output: "file.json",
          })
        )
        .takeSnapshot(shell)
        .execute(shell)

      const file = await readFile(join(testCwd(), "file.json"), "utf8")
      expect(JSON.parse(file)).toEqual({
        FNM_ARCH: expect.any(String),
        FNM_DIR: expect.any(String),
        FNM_LOGLEVEL: "info",
        FNM_MULTISHELL_PATH: expect.any(String),
        FNM_NODE_DIST_MIRROR: expect.any(String),
        FNM_VERSION_FILE_STRATEGY: "local",
      })
    })
  })
}
