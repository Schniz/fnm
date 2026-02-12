import fs from "fs"
import path from "path"
import { script } from "./shellcode/script.js"
import { Bash, PowerShell } from "./shellcode/shells.js"
import describe from "./describe.js"
import testTmpDir from "./shellcode/test-tmp-dir.js"

for (const shell of [Bash, PowerShell]) {
  describe(shell, () => {
    test(`installs default packages`, async () => {
      const fnmDir = path.join(testTmpDir(), "fnm")
      fs.mkdirSync(fnmDir, { recursive: true })
      fs.writeFileSync(path.join(fnmDir, "default-packages"), "is-odd\n")

      await script(shell)
        .then(shell.env({}))
        .then(shell.call("fnm", ["install", "18"]))
        .then(
          shell.scriptOutputContains(
            shell.call("fnm", [
              "exec",
              "--using=18",
              "npm",
              "list",
              "-g",
              "--depth=0",
            ]),
            "'is-odd'"
          )
        )
        .takeSnapshot(shell)
        .execute(shell)
    })

    test(`missing default-packages file does not error`, async () => {
      await script(shell)
        .then(shell.env({}))
        .then(shell.call("fnm", ["install", "18"]))
        .takeSnapshot(shell)
        .execute(shell)
    })
  })
}

