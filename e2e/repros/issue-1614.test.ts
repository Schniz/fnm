import { execa } from "execa"
import { mkdir, mkdtemp, rm, symlink } from "node:fs/promises"
import { tmpdir } from "node:os"
import path from "node:path"

const describeOnUnix = process.platform === "win32" ? describe.skip : describe

describeOnUnix("issue #1614", () => {
  test("fnm exec remains alive until its child handles SIGINT", async () => {
    const fnmDir = await mkdtemp(path.join(tmpdir(), "fnm-issue-1614-"))
    const nodePath = path.join(
      fnmDir,
      "node-versions",
      "v24.0.0",
      "installation",
      "bin",
      "node",
    )
    await mkdir(path.dirname(nodePath), { recursive: true })
    await symlink(process.execPath, nodePath)

    const child = execa(
      "fnm",
      [
        "exec",
        "--using=24.0.0",
        "--",
        "node",
        "-e",
        `process.on("SIGINT", () => setTimeout(() => { console.log("finished"); process.exit(0) }, 500)); console.log("ready"); setTimeout(() => {}, 10000)`,
      ],
      {
        detached: true,
        env: { ...process.env, FNM_DIR: fnmDir },
        reject: false,
      },
    )

    try {
      let output = ""
      await new Promise<void>((resolve, reject) => {
        const timeout = setTimeout(
          () => reject(new Error("fnm exec did not start its child")),
          10000,
        )
        child.stdout?.on("data", (chunk: Buffer) => {
          output += chunk.toString()
          if (output.includes("ready")) {
            clearTimeout(timeout)
            resolve()
          }
        })
        child.once("error", (error) => {
          clearTimeout(timeout)
          reject(error)
        })
      })

      process.kill(-child.pid!, "SIGINT")
      const result = await child
      output += result.stdout

      expect(result.signal).toBeUndefined()
      expect(result.exitCode).toBe(0)
      expect(output).toContain("finished")
    } finally {
      if (child.pid && child.exitCode === null && child.signalCode === null) {
        try {
          process.kill(-child.pid, "SIGKILL")
        } catch {
          // The process group may have exited while the test was cleaning up.
        }
        await child
      }
      await rm(fnmDir, { recursive: true, force: true })
    }
  })
})
