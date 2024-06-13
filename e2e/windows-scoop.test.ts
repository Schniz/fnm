import { script } from "./shellcode/script.js"
import { Bash, Fish, PowerShell, WinCmd, Zsh } from "./shellcode/shells.js"
import testNodeVersion from "./shellcode/test-node-version.js"
import describe from "./describe.js"
import os from "node:os"
import { execa } from "execa"

const testIf = (b: boolean) => (b ? test : test.skip)

if (os.platform() === "win32") {
  beforeAll(async () => {
    // Create a scoop shim for tests
    await execa(`scoop`, [
      "shim",
      "add",
      "fnm_release",
      "target/release/fnm.exe",
    ])
  })

  for (const shell of [Bash, Zsh, Fish, PowerShell, WinCmd]) {
    describe(shell, () => {
      testIf(os.platform() === "win32")(
        `scoop shims infer the shell`,
        async () => {
          await script(shell)
            .then(shell.env({ executableName: "fnm_release" }))
            .then(shell.call("fnm_release", ["install", "v20.14.0"]))
            .then(shell.call("fnm_release", ["use", "v20.14.0"]))
            .then(testNodeVersion(shell, "v20.14.0"))
            .execute(shell)
        },
      )
    })
  }
}
