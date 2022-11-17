import { script } from "./shellcode/script"
import { Bash, Fish, PowerShell, WinCmd, Zsh } from "./shellcode/shells"
import fs from "node:fs/promises"
import path from "node:path"
import describe from "./describe"
import testNodeVersion from "./shellcode/test-node-version"
import testBinDir from "./shellcode/test-bin-dir"

for (const shell of [Bash, Fish, PowerShell, WinCmd, Zsh]) {
  describe(shell, () => {
    // latest bash breaks this as it seems. gotta find a solution.
    const t = process.platform === "darwin" && shell === Bash ? test.skip : test

    t(`switches to system node`, async () => {
      const customNode = path.join(testBinDir(), "node")

      if (
        process.platform === "win32" &&
        [WinCmd, PowerShell].includes(shell)
      ) {
        await fs.writeFile(customNode + ".cmd", '@echo "custom"')
      } else {
        await fs.writeFile(customNode, `#!/bin/bash\n\necho "custom"\n`)
        // set executable
        await fs.chmod(customNode, 0o766)
      }

      await script(shell)
        .then(shell.env({}))
        .then(shell.call("fnm", ["install", "v10.10.0"]))
        .then(shell.call("fnm", ["use", "v10"]))
        .then(testNodeVersion(shell, "v10.10.0"))
        .then(shell.call("fnm", ["use", "system"]))
        .then(testNodeVersion(shell, "custom"))
        .execute(shell)
    })
  })
}
