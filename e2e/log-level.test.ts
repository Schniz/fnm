import { script } from "./shellcode/script.js"
import { Bash, Fish, PowerShell, Zsh } from "./shellcode/shells.js"
import describe from "./describe.js"
import getStderr from "./shellcode/get-stderr.js"

for (const shell of [Bash, Zsh, Fish, PowerShell]) {
  describe(shell, () => {
    test(`"quiet" log level`, async () => {
      await script(shell)
        .then(shell.env({ logLevel: "quiet" }))
        .then(
          shell.hasCommandOutput(
            shell.call("fnm", ["install", "v8.11.3"]),
            "",
            "fnm install"
          )
        )
        .then(
          shell.hasCommandOutput(
            shell.call("fnm", ["use", "v8.11.3"]),
            "",
            "fnm use"
          )
        )
        .then(
          shell.hasCommandOutput(
            shell.call("fnm", ["alias", "v8.11.3", "something"]),
            "",
            "fnm alias"
          )
        )
        .takeSnapshot(shell)
        .execute(shell)
    })

    test("error log level", async () => {
      await script(shell)
        .then(shell.env({ logLevel: "error" }))
        .then(
          shell.hasCommandOutput(
            shell.call("fnm", ["install", "v8.11.3"]),
            "",
            "fnm install"
          )
        )
        .then(
          shell.hasCommandOutput(
            shell.call("fnm", ["use", "v8.11.3"]),
            "",
            "fnm use"
          )
        )
        .then(
          shell.hasCommandOutput(
            shell.call("fnm", ["alias", "v8.11.3", "something"]),
            "",
            "fnm alias"
          )
        )
        .then(
          shell.scriptOutputContains(
            getStderr(shell.call("fnm", ["alias", "abcd", "efg"])),
            `"find requested version"`
          )
        )

        .takeSnapshot(shell)
        .execute(shell)
    })
  })
}
