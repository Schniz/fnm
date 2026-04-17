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
      const pkgDir = path.join(testTmpDir(), "default-packages-pkg")
      fs.mkdirSync(pkgDir, { recursive: true })
      fs.writeFileSync(
        path.join(pkgDir, "package.json"),
        JSON.stringify(
          {
            name: "fnm-default-packages-test",
            version: "1.0.0",
          },
          null,
          2
        )
      )
      fs.writeFileSync(path.join(fnmDir, "default-packages"), `${pkgDir}\n`)

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
            "'fnm-default-packages-test'"
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
