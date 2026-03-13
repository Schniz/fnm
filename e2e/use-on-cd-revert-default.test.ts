import { writeFile, mkdir } from "node:fs/promises"
import { join } from "node:path"
import { script } from "./shellcode/script.js"
import { Bash, Fish, PowerShell, Zsh } from "./shellcode/shells.js"
import testCwd from "./shellcode/test-cwd.js"
import testNodeVersion from "./shellcode/test-node-version.js"
import describe from "./describe.js"

const defaultVersion = "v8.11.3"
const projectVersion = "v12.22.12"

for (const shell of [Bash, Zsh, Fish, PowerShell]) {
  describe(shell, () => {
    test(`reverts to default when leaving versioned directory (local strategy)`, async () => {
      await mkdir(join(testCwd(), "subdir"), { recursive: true })
      await writeFile(join(testCwd(), "subdir", ".node-version"), projectVersion)

      await script(shell)
        .then(shell.env({ useOnCd: true }))
        .then(shell.call("fnm", ["install", defaultVersion]))
        .then(shell.call("fnm", ["install", projectVersion]))
        .then(shell.call("fnm", ["default", defaultVersion]))
        .then(shell.call("cd", ["subdir"]))
        .then(testNodeVersion(shell, projectVersion))
        .then(shell.call("cd", [".."]))
        .then(testNodeVersion(shell, defaultVersion))
        .execute(shell)
    })

    test(`stays on project version while inside project (local strategy)`, async () => {
      await mkdir(join(testCwd(), "subdir", "nested"), { recursive: true })
      await writeFile(join(testCwd(), "subdir", ".node-version"), projectVersion)

      await script(shell)
        .then(shell.env({ useOnCd: true }))
        .then(shell.call("fnm", ["install", defaultVersion]))
        .then(shell.call("fnm", ["install", projectVersion]))
        .then(shell.call("fnm", ["default", defaultVersion]))
        .then(shell.call("cd", ["subdir"]))
        .then(testNodeVersion(shell, projectVersion))
        .then(shell.call("cd", ["nested"]))
        .then(testNodeVersion(shell, defaultVersion))
        .execute(shell)
    })

    test(`no default set - graceful no-op`, async () => {
      await mkdir(join(testCwd(), "subdir"), { recursive: true })
      await writeFile(join(testCwd(), "subdir", ".node-version"), projectVersion)

      await script(shell)
        .then(shell.env({ useOnCd: true }))
        .then(shell.call("fnm", ["install", projectVersion]))
        .then(shell.call("cd", ["subdir"]))
        .then(testNodeVersion(shell, projectVersion))
        .then(shell.call("cd", [".."]))
        .execute(shell)
    })

    test(`already on default - no unnecessary switch`, async () => {
      await mkdir(join(testCwd(), "noversion"), { recursive: true })

      const captureCdOutput = (() => {
        const outputFile = "cd-output.txt"
        if (shell === Fish) {
          return `begin\n  cd noversion\n  cd ..\nend > ${outputFile} 2>&1`
        }
        if (shell === PowerShell) {
          return `& { cd noversion; cd .. } *> ${outputFile}`
        }
        return `{ cd noversion; cd ..; } > ${outputFile} 2>&1`
      })()

      await script(shell)
        .then(shell.env({ useOnCd: true }))
        .then(shell.call("fnm", ["install", defaultVersion]))
        .then(shell.call("fnm", ["default", defaultVersion]))
        .then(shell.call("fnm", ["use", "default", "--silent-if-unchanged"]))
        .then(testNodeVersion(shell, defaultVersion))
        .then(captureCdOutput)
        .then(shell.hasCommandOutput(shell.call("cat", ["cd-output.txt"]), "", "cd output"))
        .execute(shell)
    })
  })
}
