import { rmSync } from "node:fs"
import { join } from "node:path"
import { script } from "./shellcode/script.js"
import { Bash, Fish, PowerShell, Zsh } from "./shellcode/shells.js"
import testNodeVersion from "./shellcode/test-node-version.js"
import getStderr from "./shellcode/get-stderr.js"
import testTmpDir from "./shellcode/test-tmp-dir.js"
import describe from "./describe.js"

// Test-only prefixes of the proxy server in tests/proxy-server/index.mjs.
const mirror = (mode: "corrupt" | "no-signature" | "bare") =>
  `--node-dist-mirror=http://localhost:8080/__test__/${mode}`

// Each test checks what `fnm install` does, which a previous run's install would skip.
const freshScript = (shell: Parameters<typeof script>[0]) => {
  rmSync(join(testTmpDir(), "fnm"), { recursive: true, force: true })
  return script(shell)
}

for (const shell of [Bash, Zsh, Fish, PowerShell]) {
  describe(shell, () => {
    test(`verifies the signature of a download`, async () => {
      await freshScript(shell)
        .then(shell.env({}))
        .then(
          shell.scriptOutputContains(
            shell.call("fnm", ["install", "v8.11.3"]),
            "'Verified signature by'",
          ),
        )
        .then(shell.call("fnm", ["use", "v8.11.3"]))
        .then(testNodeVersion(shell, "v8.11.3"))
        .takeSnapshot(shell)
        .execute(shell)
    })

    test(`does not install a download that doesn't match its checksum`, async () => {
      await freshScript(shell)
        .then(shell.env({}))
        .then(
          shell.scriptOutputContains(
            getStderr(
              shell.call("fnm", [mirror("corrupt"), "install", "v8.11.3"]),
            ),
            "'match its published checksum'",
          ),
        )
        .then(
          shell.hasCommandOutput(
            shell.call("fnm", ["ls"]),
            "* system",
            "fnm ls",
          ),
        )
        .takeSnapshot(shell)
        .execute(shell)
    })

    test(`does not install from a mirror without signatures`, async () => {
      await freshScript(shell)
        .then(shell.env({}))
        .then(
          shell.scriptOutputContains(
            getStderr(
              shell.call("fnm", [mirror("no-signature"), "install", "v8.11.3"]),
            ),
            "'FNM_DOWNLOAD_VERIFICATION=checksum'",
          ),
        )
        .then(
          shell.hasCommandOutput(
            shell.call("fnm", ["ls"]),
            "* system",
            "fnm ls",
          ),
        )
        .takeSnapshot(shell)
        .execute(shell)
    })

    test(`installs from a mirror without signatures with checksum verification`, async () => {
      await freshScript(shell)
        .then(shell.env({}))
        .then(
          shell.scriptOutputContains(
            getStderr(
              shell.call("fnm", [
                mirror("no-signature"),
                "--download-verification=checksum",
                "install",
                "v8.11.3",
              ]),
            ),
            "'no signature is checked'",
          ),
        )
        .then(shell.call("fnm", ["use", "v8.11.3"]))
        .then(testNodeVersion(shell, "v8.11.3"))
        .takeSnapshot(shell)
        .execute(shell)
    })

    test(`fails when the release keys file is missing`, async () => {
      await freshScript(shell)
        .then(shell.env({}))
        .then(
          shell.scriptOutputContains(
            getStderr(shell.call("fnm", ["install", "v8.11.3"])),
            "'missing-release-keys.asc can.t be used'",
          ),
        )
        .then(
          shell.hasCommandOutput(
            shell.call("fnm", ["ls"]),
            "* system",
            "fnm ls",
          ),
        )
        .addExtraEnvVar(
          "FNM_NODE_RELEASE_KEYS",
          join(testTmpDir(), "missing-release-keys.asc"),
        )
        .takeSnapshot(shell)
        .execute(shell)
    })

    test(`installs from a mirror without checksums with verification off`, async () => {
      await freshScript(shell)
        .then(shell.env({}))
        .then(
          shell.scriptOutputContains(
            getStderr(
              shell.call("fnm", [
                mirror("bare"),
                "--download-verification=none",
                "install",
                "v8.11.3",
              ]),
            ),
            "'the download is not checked'",
          ),
        )
        .then(shell.call("fnm", ["use", "v8.11.3"]))
        .then(testNodeVersion(shell, "v8.11.3"))
        .takeSnapshot(shell)
        .execute(shell)
    })
  })
}
