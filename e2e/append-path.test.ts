import { chmod, mkdir, writeFile } from "node:fs/promises"
import { join } from "node:path"
import describe from "./describe.js"
import { script } from "./shellcode/script.js"
import { Bash, Fish, PowerShell, WinCmd, Zsh } from "./shellcode/shells.js"
import testBinDir from "./shellcode/test-bin-dir.js"
import testCwd from "./shellcode/test-cwd.js"
import testNodeVersion from "./shellcode/test-node-version.js"
import testTmpDir from "./shellcode/test-tmp-dir.js"

const markerName = "fnm-path-order-marker"
type TestShell =
  | typeof Bash
  | typeof Zsh
  | typeof Fish
  | typeof PowerShell
  | typeof WinCmd

for (const shell of [Bash, Zsh, Fish, PowerShell, WinCmd]) {
  describe(shell, () => {
    test("prepends the active Node path by default", async () => {
      await install(shell, ["v8.11.3"])
      await writePathCommand(testBinDir(), markerName, "earlier", shell)
      await writeVersionCommand("v8.11.3", markerName, "managed", shell)

      await script(shell)
        .then(shell.env({}))
        .then(shell.call("fnm", ["use", "v8.11.3"]))
        .then(expectOutput(shell, markerName, "managed"))
        .execute(shell)
    })

    test("appends the active Node path after existing commands", async () => {
      await install(shell, ["v8.11.3"])
      await writePathCommand(testBinDir(), markerName, "earlier", shell)
      await writeVersionCommand("v8.11.3", markerName, "managed", shell)

      await script(shell)
        .then(shell.env({ append: true }))
        .then(shell.call("fnm", ["use", "v8.11.3"]))
        .then(expectOutput(shell, markerName, "earlier"))
        .execute(shell)
    })

    test("finds commands that exist only in the appended Node path", async () => {
      await install(shell, ["v8.11.3"])
      await writeVersionCommand("v8.11.3", markerName, "managed", shell)

      await script(shell)
        .then(shell.env({ append: true }))
        .then(shell.call("fnm", ["use", "v8.11.3"]))
        .then(expectOutput(shell, markerName, "managed"))
        .execute(shell)
    })

    test("uses the newest session path after repeated append setup", async () => {
      await install(shell, ["v8.11.3", "v10.10.0"])
      await writeVersionCommand("v8.11.3", markerName, "v8", shell)
      await writeVersionCommand("v10.10.0", markerName, "v10", shell)

      await script(shell)
        .then(shell.env({ append: true }))
        .then(shell.call("fnm", ["use", "v8.11.3"]))
        .then(expectOutput(shell, markerName, "v8"))
        .then(shell.env({ append: true }))
        .then(shell.call("fnm", ["use", "v10.10.0"]))
        .then(expectOutput(shell, markerName, "v10"))
        .execute(shell)
    })

    test("removes stale session paths when switching from prepend to append", async () => {
      await install(shell, ["v8.11.3", "v10.10.0"])
      await writeVersionCommand("v8.11.3", markerName, "v8", shell)
      await writeVersionCommand("v10.10.0", markerName, "v10", shell)

      await script(shell)
        .then(shell.env({}))
        .then(shell.call("fnm", ["use", "v8.11.3"]))
        .then(shell.env({}))
        .then(shell.env({ append: true }))
        .then(shell.call("fnm", ["use", "v10.10.0"]))
        .then(expectOutput(shell, markerName, "v10"))
        .execute(shell)
    })

    test("keeps append precedence while use-on-cd changes versions", async () => {
      await mkdir(join(testCwd(), "subdir"), { recursive: true })
      await writeFile(join(testCwd(), ".node-version"), "v8.11.3")
      await writeFile(join(testCwd(), "subdir", ".node-version"), "v10.10.0")
      await install(shell, ["v8.11.3", "v10.10.0"])
      await writeVersionCommand("v8.11.3", markerName, "v8", shell)
      await writeVersionCommand("v10.10.0", markerName, "v10", shell)
      await writeCustomNode(shell)

      await script(shell)
        .then(shell.env({ append: true, useOnCd: true }))
        .then(expectOutput(shell, markerName, "v8"))
        .then(testNodeVersion(shell, "custom"))
        .then(shell.call("cd", ["subdir"]))
        .then(expectOutput(shell, markerName, "v10"))
        .then(testNodeVersion(shell, "custom"))
        .execute(shell)
    })

    if (process.platform !== "win32") {
      test("handles shell metacharacters in the multishell storage path", async () => {
        await install(shell, ["v8.11.3"])
        await writeVersionCommand("v8.11.3", markerName, "managed", shell)
        const stateHome = join(
          testTmpDir(),
          "state with 'quotes' $dollars `backticks` [brackets] ; semicolon",
        )

        await script(shell)
          .addExtraEnvVar("XDG_STATE_HOME", stateHome)
          .then(shell.env({ append: true }))
          .then(shell.call("fnm", ["use", "v8.11.3"]))
          .then(expectOutput(shell, markerName, "managed"))
          .execute(shell)
      })
    }

    if (process.platform === "win32" && shell === PowerShell) {
      test("handles PowerShell metacharacters in the multishell storage path", async () => {
        await install(shell, ["v8.11.3"])
        await writeVersionCommand("v8.11.3", markerName, "managed", shell)
        const stateHome = join(
          testTmpDir(),
          "state with 'quotes' $dollars `backticks` [brackets] & semicolon;",
        )

        await script(shell)
          .addExtraEnvVar("XDG_STATE_HOME", stateHome)
          .then(shell.env({ append: true }))
          .then(shell.call("fnm", ["use", "v8.11.3"]))
          .then(expectOutput(shell, markerName, "managed"))
          .execute(shell)
      })
    }
  })
}

async function install(shell: TestShell, versions: string[]): Promise<void> {
  let setup = script(shell)
  for (const version of versions) {
    setup = setup.then(shell.call("fnm", ["install", version]))
  }
  await setup.execute(shell)
}

async function writeVersionCommand(
  version: string,
  name: string,
  output: string,
  shell: TestShell,
): Promise<void> {
  const installation = join(
    testTmpDir(),
    "fnm",
    "node-versions",
    version,
    "installation",
  )
  const bin =
    process.platform === "win32" ? installation : join(installation, "bin")
  await writePathCommand(bin, name, output, shell)
}

async function writePathCommand(
  directory: string,
  name: string,
  output: string,
  shell: TestShell,
): Promise<void> {
  await mkdir(directory, { recursive: true })
  if (
    process.platform === "win32" &&
    (shell === PowerShell || shell === WinCmd)
  ) {
    await writeFile(join(directory, `${name}.cmd`), `@echo ${output}`)
    return
  }

  const filename = join(directory, name)
  await writeFile(filename, `#!/bin/sh\necho ${output}\n`)
  await chmod(filename, 0o766)
}

function expectOutput(shell: TestShell, command: string, output: string) {
  return shell.hasCommandOutput(shell.call(command, []), output, command)
}

async function writeCustomNode(shell: TestShell): Promise<void> {
  const filename = join(
    testBinDir(),
    process.platform === "win32" && (shell === PowerShell || shell === WinCmd)
      ? "node.cmd"
      : "node",
  )
  await writeFile(
    filename,
    process.platform === "win32" && filename.endsWith(".cmd")
      ? "@echo custom"
      : "#!/bin/sh\necho custom\n",
  )
  if (!filename.endsWith(".cmd")) {
    await chmod(filename, 0o766)
  }
}
