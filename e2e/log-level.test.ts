import { script } from "./shellcode/script"
import { Bash, Fish, PowerShell, Zsh } from "./shellcode/shells"
import describe from "./describe"

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
  })
}
