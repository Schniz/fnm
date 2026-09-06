import { writeFile } from "node:fs/promises"
import { join } from "node:path"
import describe from "../describe.js"
import { script } from "../shellcode/script.js"
import { Bash, Fish, PowerShell, Zsh } from "../shellcode/shells.js"
import testCwd from "../shellcode/test-cwd.js"

for (const shell of [Bash, Zsh, Fish, PowerShell]) {
  describe(shell, () => {
    test("issue #1461: repeated installation respects log levels", async () => {
      await writeFile(
        join(testCwd(), "check-install.cjs"),
        `
          const assert = require("assert")
          const { spawnSync } = require("child_process")
          for (const level of ["info", "all", "error", "quiet"]) {
            const installed = spawnSync("fnm", ["--log-level", level, "install", "v8.11.3"], { encoding: "utf8" })
            assert.strictEqual(installed.status, 0, installed.stderr)
            assert.strictEqual(installed.stderr, "")
            if (level === "info" || level === "all") {
              assert.ok(/already installed/.test(installed.stdout))
            } else {
              assert.strictEqual(installed.stdout, "")
            }

            const failed = spawnSync("fnm", ["--log-level", level, "alias", "abcd", "efg"], { encoding: "utf8" })
            assert.strictEqual(failed.status, 1)
            assert.strictEqual(failed.stdout, "")
            if (level === "quiet") {
              assert.strictEqual(failed.stderr, "")
            } else {
              assert.ok(/find requested version/.test(failed.stderr))
            }
          }
          console.log("logging checks passed")
        `,
      )
      await script(shell)
        .then(shell.env({}))
        .then(shell.call("fnm", ["install", "v8.11.3"]))
        .then(
          shell.hasCommandOutput(
            shell.call("node", ["check-install.cjs"]),
            "logging checks passed",
            "installation logging checks",
          ),
        )
        .execute(shell)
    })
  })
}
