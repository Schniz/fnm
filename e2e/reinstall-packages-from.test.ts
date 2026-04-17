import getStderr from "./shellcode/get-stderr.js"
import { script } from "./shellcode/script.js"
import { Bash, Fish, PowerShell, Zsh } from "./shellcode/shells.js"
import describe from "./describe.js"

const SOURCE_VERSION = "v18.20.0"
const TARGET_VERSION = "v20.11.0"

function captureAndVerifyReinstallOutput(
  shell: typeof Bash | typeof Zsh | typeof Fish | typeof PowerShell,
): string {
  const installCmd = `fnm install ${TARGET_VERSION} --reinstall-packages-from=${SOURCE_VERSION}`

  if (shell === PowerShell) {
    return `$__out__ = ${installCmd} 2>&1 | Out-String
if ($__out__ -notmatch "is-odd@") { exit 1 }
if ($__out__ -match "  - npm@") { exit 1 }
if ($__out__ -match "  - corepack@") { exit 1 }
if ($__out__ -notmatch "Successfully reinstalled") { exit 1 }
`
  }

  if (shell === Fish) {
    return `set __out__ (${installCmd} 2>&1)
echo $__out__ | grep 'is-odd@'; or begin; echo "Expected output to contain 'is-odd@'"; exit 1; end
echo $__out__ | grep '  - npm@'; and begin; echo "Expected output to not contain 'npm@'"; exit 1; end
echo $__out__ | grep '  - corepack@'; and begin; echo "Expected output to not contain 'corepack@'"; exit 1; end
echo $__out__ | grep 'Successfully reinstalled'; or begin; echo "Expected output to contain 'Successfully reinstalled'"; exit 1; end
`
  }

  // Bash and Zsh share syntax
  return `__out__="$(${installCmd} 2>&1)"
echo "$__out__" | grep 'is-odd@' || (echo "Expected output to contain 'is-odd@'" && exit 1)
if echo "$__out__" | grep -q '  - npm@'; then
  echo "Expected output to not contain 'npm@'"
  exit 1
fi
if echo "$__out__" | grep -q '  - corepack@'; then
  echo "Expected output to not contain 'corepack@'"
  exit 1
fi
echo "$__out__" | grep 'Successfully reinstalled' || (echo "Expected output to contain 'Successfully reinstalled'" && exit 1)
`
}

for (const shell of [Bash, Zsh, Fish, PowerShell]) {
  describe(shell, () => {
    test(`reinstall packages from another version`, async () => {
      await script(shell)
        .then(shell.env({}))
        .then(shell.call("fnm", ["install", SOURCE_VERSION]))
        .then(shell.call("fnm", ["use", SOURCE_VERSION]))
        .then(shell.call("npm", ["install", "-g", "is-odd"]))
        .then(
          shell.scriptOutputContains(
            shell.call("npm", ["list", "-g", "--depth=0"]),
            "'is-odd'",
          ),
        )
        .then(captureAndVerifyReinstallOutput(shell))
        .then(shell.call("fnm", ["use", TARGET_VERSION]))
        .then(
          shell.scriptOutputContains(
            shell.call("npm", ["list", "-g", "--depth=0"]),
            "'is-odd'",
          ),
        )
        .takeSnapshot(shell)
        .execute(shell)
    })

    test(`errors when source version is not installed`, async () => {
      await script(shell)
        .then(shell.env({ logLevel: "error" }))
        .then(
          shell.scriptOutputContains(
            getStderr(
              shell.call("fnm", [
                "install",
                TARGET_VERSION,
                `--reinstall-packages-from=${SOURCE_VERSION}`,
              ]),
            ),
            "'Version v18.20.0 is not installed'",
          ),
        )
        .takeSnapshot(shell)
        .execute(shell)
    })

    test(`skips reinstall when source and target are the same version`, async () => {
      await script(shell)
        .then(shell.env({}))
        .then(shell.call("fnm", ["install", SOURCE_VERSION]))
        .then(
          shell.scriptOutputContains(
            shell.call("fnm", [
              "install",
              SOURCE_VERSION,
              `--reinstall-packages-from=${SOURCE_VERSION}`,
            ]),
            "'Skipping package reinstallation'",
          ),
        )
        .takeSnapshot(shell)
        .execute(shell)
    })

    test(`source has no global packages`, async () => {
      await script(shell)
        .then(shell.env({}))
        .then(shell.call("fnm", ["install", SOURCE_VERSION]))
        .then(
          shell.scriptOutputContains(
            shell.call("fnm", [
              "install",
              TARGET_VERSION,
              `--reinstall-packages-from=${SOURCE_VERSION}`,
            ]),
            "'No global packages found in'",
          ),
        )
        .takeSnapshot(shell)
        .execute(shell)
    })
  })
}
