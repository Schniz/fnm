import { lstat, readFile, realpath } from "node:fs/promises"
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
          })
        )
        .takeSnapshot(shell)
        .execute(shell)

      if (shell.currentlySupported()) {
        const file = await readFile(join(testCwd(), filename), "utf8")
        const json = JSON.parse(file)
        expect(json).toEqual({
          FNM_ARCH: expect.any(String),
          FNM_DIR: expect.any(String),
          FNM_LOGLEVEL: "info",
          FNM_MULTISHELL_PATH: expect.any(String),
          FNM_NODE_DIST_MIRROR: expect.any(String),
          FNM_VERSION_FILE_STRATEGY: "local",
        })
      }
    })

    test(`deletes the multishell upon shell exit`, async () => {
      const filename = `multishell_path`
      await script(shell)
        .then(shell.env({ args: ["--delete-on-exit"] }))
        .then(
          shell.redirectOutput(
            shell.call("echo", [shell.getEnvVar("FNM_MULTISHELL_PATH")]),
            { output: filename }
          )
        )
        .takeSnapshot(shell)
        .execute(shell)

      if (shell.currentlySupported()) {
        const multishell = await readFile(
          join(testCwd(), filename),
          "utf8"
        ).then((x) => x.trim())
        await expect(lstat(multishell)).rejects.toThrowError(
          /no such file or directory/
        )
      }
    })
  })
}
