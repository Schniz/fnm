import { script } from "./shellcode/script.js"
import { Bash, Fish, PowerShell, Zsh } from "./shellcode/shells.js"
import describe from "./describe.js"
import { writeFile } from "node:fs/promises"
import path from "node:path"
import testCwd from "./shellcode/test-cwd.js"
import getStderr from "./shellcode/get-stderr.js"
import testNodeVersion from "./shellcode/test-node-version.js"

for (const shell of [Bash, Zsh, Fish, PowerShell]) {
  describe(shell, () => {
    test(`installs aliases when corepack is enabled`, async () => {
      await writeFile(path.join(testCwd(), ".node-version"), "lts/*")
      await script(shell)
        .then(shell.env({ corepackEnabled: true }))
        .then(shell.call("fnm", ["use", "--install-if-missing"]))
        .then(
          shell.scriptOutputContains(shell.call("fnm", ["ls"]), "lts-latest"),
        )
        .takeSnapshot(shell)
        .execute(shell)
    })

    test(`allows to install an lts if version missing`, async () => {
      await writeFile(path.join(testCwd(), ".node-version"), "lts/*")
      await script(shell)
        .then(shell.env({}))
        .then(shell.call("fnm", ["use", "--install-if-missing"]))
        .then(
          shell.scriptOutputContains(shell.call("fnm", ["ls"]), "lts-latest"),
        )
        .takeSnapshot(shell)
        .execute(shell)
    })

    test(`errors when alias is not found`, async () => {
      await writeFile(path.join(testCwd(), ".node-version"), "lts/*")
      await script(shell)
        .then(shell.env({ logLevel: "error" }))
        .then(
          shell.scriptOutputContains(
            getStderr(shell.call("fnm", ["use"])),
            "'Requested version lts-latest is not'",
          ),
        )
        .takeSnapshot(shell)
        .execute(shell)
    })

    test(`unalias a version`, async () => {
      await script(shell)
        .then(shell.env({}))
        .then(shell.call("fnm", ["install", "11.10.0"]))
        .then(shell.call("fnm", ["install", "8.11.3"]))
        .then(shell.call("fnm", ["alias", "8.11.3", "version8"]))
        .then(shell.scriptOutputContains(shell.call("fnm", ["ls"]), "version8"))
        .then(shell.call("fnm", ["unalias", "version8"]))
        .then(
          shell.scriptOutputContains(
            getStderr(shell.call("fnm", ["use", "version8"])),
            "'Requested version version8 is not currently installed'",
          ),
        )
        .takeSnapshot(shell)
        .execute(shell)
    })

    test(`unalias errors if alias not found`, async () => {
      await script(shell)
        .then(shell.env({ logLevel: "error" }))
        .then(
          shell.scriptOutputContains(
            getStderr(shell.call("fnm", ["unalias", "lts"])),
            "'Requested alias lts not found'",
          ),
        )
        .takeSnapshot(shell)
        .execute(shell)
    })

    test(`can alias the system node`, async () => {
      await script(shell)
        .then(shell.env({}))
        .then(shell.call("fnm", ["alias", "system", "my_system"]))
        .then(
          shell.scriptOutputContains(shell.call("fnm", ["ls"]), "my_system"),
        )
        .then(shell.call("fnm", ["alias", "system", "default"]))
        .then(shell.call("fnm", ["alias", "my_system", "my_system2"]))
        .then(
          shell.scriptOutputContains(shell.call("fnm", ["ls"]), "my_system2"),
        )
        .then(
          shell.scriptOutputContains(
            shell.call("fnm", ["use", "my_system"]),
            "'Bypassing fnm'",
          ),
        )
        .then(shell.call("fnm", ["unalias", "my_system"]))
        .then(
          shell.scriptOutputContains(
            getStderr(shell.call("fnm", ["use", "my_system"])),
            "'Requested version my_system is not currently installed'",
          ),
        )
        .takeSnapshot(shell)
        .execute(shell)
    })

    test(`aliasing versions`, async () => {
      const installedVersions = shell.call("fnm", ["ls"])
      await script(shell)
        .then(shell.env({}))
        .then(shell.call("fnm", ["install", "6.11.3"]))
        .then(shell.call("fnm", ["install", "8.11.3"]))
        .then(shell.call("fnm", ["alias", "8.11", "oldie"]))
        .then(shell.call("fnm", ["alias", "6", "older"]))
        .then(shell.call("fnm", ["default", "older"]))
        .then(
          shell.scriptOutputContains(
            shell.scriptOutputContains(installedVersions, "8.11.3"),
            "oldie",
          ),
        )
        .then(
          shell.scriptOutputContains(
            shell.scriptOutputContains(installedVersions, "6.11.3"),
            "older",
          ),
        )
        .then(shell.call("fnm", ["use", "older"]))
        .then(testNodeVersion(shell, "v6.11.3"))
        .then(shell.call("fnm", ["use", "oldie"]))
        .then(testNodeVersion(shell, "v8.11.3"))
        .then(shell.call("fnm", ["use", "default"]))
        .then(testNodeVersion(shell, "v6.11.3"))
        .takeSnapshot(shell)
        .execute(shell)
    })
  })
}
