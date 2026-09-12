import { copyFile, mkdir, writeFile } from "node:fs/promises"
import { join, relative } from "node:path"
import describe from "../describe.js"
import { script } from "../shellcode/script.js"
import { Bash, Fish, PowerShell, Zsh } from "../shellcode/shells.js"
import testCwd from "../shellcode/test-cwd.js"
import testNodeVersion from "../shellcode/test-node-version.js"
import testTmpDir from "../shellcode/test-tmp-dir.js"

for (const shell of [Bash, Zsh, Fish, PowerShell]) {
  describe(shell, () => {
    test("issue 1210 keeps shell wrappers available to the hook", async () => {
      await mkdir(join(testCwd(), "project"), { recursive: true })
      await writeFile(join(testCwd(), "project", ".node-version"), "12.22.12")
      const wrapper =
        shell === PowerShell
          ? 'function fnm { $env:FNM_WRAPPER_CALLED = "yes" }'
          : shell === Fish
            ? "function fnm; set -gx FNM_WRAPPER_CALLED yes; end"
            : "fnm() { export FNM_WRAPPER_CALLED=yes; }"
      await script(shell)
        .then(shell.env({ useOnCd: true }))
        .then(wrapper)
        .then(shell.call("cd", ["project"]))
        .then(
          shell.hasCommandOutput(
            shell === PowerShell
              ? "echo $env:FNM_WRAPPER_CALLED"
              : "echo $FNM_WRAPPER_CALLED",
            "yes",
            "wrapper invocation",
          ),
        )
        .execute(shell)
    })
    for (const strategy of ["local", "recursive"]) {
      for (const invocation of ["absolute", "relative"]) {
        test(`issue 1210 ${strategy} ${invocation} executable outside PATH`, async () => {
          const directory = join(
            testTmpDir(),
            "fnm caf\u00e9'\u{10400} space ' $dollar `tick & %FNM_TEST% !",
          )
          await mkdir(directory, { recursive: true })
          const binary = process.platform === "win32" ? "fnm.exe" : "fnm"
          const executable = join(directory, binary)
          await copyFile(
            join(
              process.cwd(),
              "target",
              process.env.FNM_TARGET_NAME ?? "debug",
              binary,
            ),
            executable,
          )
          await mkdir(join(testCwd(), "project", "child"), { recursive: true })
          await writeFile(
            join(testCwd(), "project", ".node-version"),
            "12.22.12",
          )
          const path = (
            invocation === "absolute"
              ? executable
              : relative(testCwd(), executable)
          ).replaceAll("\\", "/")
          const quoted =
            shell === PowerShell
              ? `& '${path.replaceAll("'", "''")}'`
              : shell === Fish
                ? `'${path.replaceAll("\\", "\\\\").replaceAll("'", "\\'")}'`
                : `'${path.replaceAll("'", "'\\''")}'`
          const clearPath =
            shell === PowerShell
              ? '$env:PATH = ""'
              : shell === Fish
                ? "set -gx PATH ''"
                : "export PATH=''"
          const missing =
            shell === PowerShell
              ? 'if (Get-Command fnm -ErrorAction SilentlyContinue) { throw "fnm is on PATH" }'
              : shell === Fish
                ? "if type -q fnm; exit 1; end"
                : "if command -v fnm; then exit 1; fi"
          const env = shell
            .env({
              executableName: `${quoted} --version-file-strategy=${strategy}`,
              useOnCd: true,
            })
            .replace(" env ", ` env --shell ${shell.name().toLowerCase()} `)
          await script(shell)
            .then(shell.env({}))
            .then(shell.call("fnm", ["install", "8.11.3"]))
            .then(shell.call("fnm", ["install", "12.22.12"]))
            .then(clearPath)
            .then(missing)
            .then(env)
            .then(env)
            .then(testNodeVersion(shell, "v8.11.3"))
            .then(
              shell.call("cd", [
                strategy === "recursive" ? "project/child" : "project",
              ]),
            )
            .then(testNodeVersion(shell, "v12.22.12"))
            .execute(shell)
        })
      }
    }
  })
}
