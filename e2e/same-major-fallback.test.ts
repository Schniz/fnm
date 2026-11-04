import { mkdir, rm, writeFile } from "node:fs/promises"
import { join } from "node:path"
import describe from "./describe.js"
import { script } from "./shellcode/script.js"
import { Bash, Fish, PowerShell, WinCmd, Zsh } from "./shellcode/shells.js"
import testCwd from "./shellcode/test-cwd.js"
import testTmpDir from "./shellcode/test-tmp-dir.js"
import testNodeVersion from "./shellcode/test-node-version.js"

async function installedVersions(...versions: string[]) {
  for (const version of versions) {
    await mkdir(
      join(testTmpDir(), "fnm", "node-versions", `v${version}`, "installation"),
      { recursive: true },
    )
  }
}

for (const shell of [Bash, Zsh, Fish, PowerShell, WinCmd]) {
  const shellName =
    shell === PowerShell
      ? "powershell"
      : shell === WinCmd
        ? "cmd"
        : shell.binaryName()
  const env = (config: Parameters<typeof shell.env>[0]) =>
    shell.env({ shell: shellName, ...config })
  describe(shell, () => {
    beforeEach(async () => {
      await rm(join(testTmpDir(), "fnm"), { recursive: true, force: true })
      await rm(testCwd(), { recursive: true, force: true })
    })

    test("issue #1500: fallback is disabled by default", async () => {
      await installedVersions("20.20.0")
      await writeFile(join(testCwd(), ".nvmrc"), "20.6.1")
      await script(shell)
        .addExtraEnvVar("NO_COLOR", "1")
        .then(env({}))
        .then(
          shell.hasCommandOutput(
            shell.call("fnm", ["use"]) + " 2>&1",
            "error: Requested version v20.6.1 is not currently installed",
            "missing exact version diagnostic",
          ),
        )
        .execute(shell)
    })

    test("issue #1500: selects latest newer same-major version and explains fallback", async () => {
      await installedVersions("20.10.0", "20.20.0", "21.0.0", "20.21.0-rc.1")
      await writeFile(join(testCwd(), ".nvmrc"), "20.6.1")
      await writeFile(join(testCwd(), ".node-version"), "21.0.0")
      await writeFile(
        join(testCwd(), "package.json"),
        JSON.stringify({ engines: { node: ">=21" } }),
      )
      await script(shell)
        .addExtraEnvVar("NO_COLOR", "1")
        .then(env({ fallbackToLatestInMajor: true }))
        .then(shell.call("fnm", ["alias", "21.0.0", "20.6.1"]))
        .then(
          shell.hasCommandOutput(
            shell.call("fnm", ["use", "--install-if-missing", ".nvmrc"]),
            "Using Node v20.20.0 instead of requested v20.6.1",
            "fallback diagnostic",
          ),
        )
        .then(shell.call("fnm", ["use"]))
        .then(
          shell.hasCommandOutput("fnm current", "v20.20.0", "selected version"),
        )
        .execute(shell)
    })

    test("issue #1500: exact installed versions take precedence", async () => {
      await installedVersions("20.6.1", "20.20.0")
      await writeFile(join(testCwd(), ".node-version"), "20.6.1")
      await script(shell)
        .addExtraEnvVar("NO_COLOR", "1")
        .then(env({ fallbackToLatestInMajor: true }))
        .then(shell.call("fnm", ["use"]))
        .then(shell.hasCommandOutput("fnm current", "v20.6.1", "exact version"))
        .execute(shell)
    })

    test.each(["20.5.0", "21.0.0", "20.20.0-rc.1"])(
      "issue #1500: %s alone does not satisfy a missing stable pin",
      async (version) => {
        await installedVersions(version)
        await writeFile(join(testCwd(), ".nvmrc"), "20.6.1")
        await script(shell)
          .addExtraEnvVar("NO_COLOR", "1")
          .then(env({ fallbackToLatestInMajor: true }))
          .then(
            shell.hasCommandOutput(
              shell.call("fnm", ["use"]) + " 2>&1",
              "error: Requested version v20.6.1 is not currently installed",
              "missing fallback diagnostic",
            ),
          )
          .execute(shell)
      },
    )

    test("issue #1500: explicit versions stay exact in opted-in shells", async () => {
      await installedVersions("20.20.0")
      await script(shell)
        .addExtraEnvVar("NO_COLOR", "1")
        .then(env({ fallbackToLatestInMajor: true }))
        .then(
          shell.hasCommandOutput(
            shell.call("fnm", ["use", "20.6.1"]) + " 2>&1",
            "error: Requested version v20.6.1 is not currently installed",
            "explicit version diagnostic",
          ),
        )
        .execute(shell)
    })

    test("issue #1500: named aliases keep their target", async () => {
      await installedVersions("20.20.0", "21.0.0")
      await writeFile(join(testCwd(), ".nvmrc"), "work")
      await script(shell)
        .addExtraEnvVar("NO_COLOR", "1")
        .then(env({ fallbackToLatestInMajor: true }))
        .then(shell.call("fnm", ["alias", "21.0.0", "work"]))
        .then(shell.call("fnm", ["use"]))
        .then(shell.hasCommandOutput("fnm current", "v21.0.0", "alias target"))
        .execute(shell)
    })

    test("issue #1500: exact engines ranges are not relaxed", async () => {
      await installedVersions("20.20.0")
      await writeFile(
        join(testCwd(), "package.json"),
        JSON.stringify({ engines: { node: "20.6.1" } }),
      )
      await script(shell)
        .addExtraEnvVar("NO_COLOR", "1")
        .then(env({ fallbackToLatestInMajor: true }))
        .then(
          shell.hasCommandOutput(
            shell.call("fnm", ["use"]) + " 2>&1",
            "error: Requested version 20.6.1 is not currently installed",
            "engines range diagnostic",
          ),
        )
        .execute(shell)
    })

    test("issue #1500: use-on-cd switches the real Node executable", async () => {
      await mkdir(join(testCwd(), "project"), { recursive: true })
      await writeFile(join(testCwd(), "project", ".nvmrc"), "20.6.1")
      await script(shell)
        .addExtraEnvVar("NO_COLOR", "1")
        .then(env({ useOnCd: true, fallbackToLatestInMajor: true }))
        .then(shell.call("fnm", ["install", "20.11.0"]))
        .then(shell.call("fnm", ["use", "system"]))
        .then(shell.call("cd", ["project"]))
        .then(testNodeVersion(shell, "v20.11.0"))
        .execute(shell)
    })

    test("issue #1500: initial env setup uses fallback", async () => {
      await installedVersions("20.20.0")
      await writeFile(join(testCwd(), ".nvmrc"), "20.6.1")
      await script(shell)
        .addExtraEnvVar("NO_COLOR", "1")
        .then(env({ useOnCd: true, fallbackToLatestInMajor: true }))
        .then(
          shell.hasCommandOutput("fnm current", "v20.20.0", "initial version"),
        )
        .execute(shell)
    })
  })
}
