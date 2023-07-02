import { ScriptLine, Shell } from "./shells/types.js"
import { execa, type ExecaChildProcess } from "execa"
import testTmpDir from "./test-tmp-dir.js"
import { Writable } from "node:stream"
import { dedent } from "ts-dedent"
import testCwd from "./test-cwd.js"
import path, { join } from "node:path"
import { writeFile } from "node:fs/promises"
import chalk from "chalk"
import testBinDir from "./test-bin-dir.js"
import { rmSync } from "node:fs"

class Script {
  constructor(
    private readonly config: {
      fnmDir: string
    },
    private readonly lines: ScriptLine[],
    private readonly extraEnvVars: Record<string, string> = {}
  ) {}
  then(line: ScriptLine): Script {
    return new Script(this.config, [...this.lines, line])
  }

  takeSnapshot(shell: Pick<Shell, "name">): this {
    const script = this.lines.join("\n")
    expect(script).toMatchSnapshot(shell.name())

    return this
  }

  addExtraEnvVar(name: string, value: string): this {
    return new Script(
      this.config,
      this.lines,
      Object.assign({}, this.extraEnvVars, { [name]: value })
    ) as this
  }

  async execute(
    shell: Pick<
      Shell,
      "binaryName" | "launchArgs" | "currentlySupported" | "forceFile"
    >
  ): Promise<void> {
    if (!shell.currentlySupported()) {
      return
    }

    const args = [...shell.launchArgs()]

    if (shell.forceFile) {
      let filename = join(testTmpDir(), "script")
      if (typeof shell.forceFile === "string") {
        filename = filename + shell.forceFile
      }
      await writeFile(filename, [...this.lines, "exit 0"].join("\n"))
      args.push(filename)
    }

    const child = execa(shell.binaryName(), args, {
      stdio: [shell.forceFile ? "ignore" : "pipe", "pipe", "pipe"],
      cwd: testCwd(),
      env: (() => {
        const newProcessEnv: Record<string, string> = {
          ...this.extraEnvVars,
          ...removeAllFnmEnvVars(process.env),
          PATH: [testBinDir(), fnmTargetDir(), process.env.PATH]
            .filter(Boolean)
            .join(path.delimiter),
          FNM_DIR: this.config.fnmDir,
          FNM_NODE_DIST_MIRROR: "http://localhost:8080",
        }

        delete newProcessEnv.NODE_OPTIONS
        return newProcessEnv
      })(),
      extendEnv: false,
      reject: false,
    })

    if (child.stdin) {
      const childStdin = child.stdin

      for (const line of this.lines) {
        await write(childStdin, `${line}\n`)
      }

      await write(childStdin, "exit 0\n")
    }

    const { stdout, stderr } = streamOutputsAndBuffer(child)

    const finished = await child

    if (finished.failed) {
      console.error(
        dedent`
          Script failed.
            code ${finished.exitCode}
            signal ${finished.signal}

          stdout:
          ${padAllLines(stdout.join(""), 2)}

          stderr:
          ${padAllLines(stderr.join(""), 2)}
        `
      )

      throw new Error(
        `Script failed on ${testCwd()} with code ${finished.exitCode}`
      )
    }
  }

  asLine(): ScriptLine {
    return this.lines.join("\n")
  }
}

function streamOutputsAndBuffer(child: ExecaChildProcess) {
  const stdout: string[] = []
  const stderr: string[] = []
  const testName = expect.getState().currentTestName ?? "unknown"
  const testPath = expect.getState().testPath ?? "unknown"

  const stdoutPrefix = chalk.cyan.dim(`[stdout] ${testPath}/${testName}: `)
  const stderrPrefix = chalk.magenta.dim(`[stderr] ${testPath}/${testName}: `)

  if (child.stdout) {
    child.stdout.on("data", (data) => {
      const lines = data.toString().trim().split(/\r?\n/)
      for (const line of lines) {
        process.stdout.write(`${stdoutPrefix}${line}\n`)
      }
      stdout.push(data.toString())
    })
  }

  if (child.stderr) {
    child.stderr.on("data", (data) => {
      const lines = data.toString().trim().split(/\r?\n/)
      for (const line of lines) {
        process.stdout.write(`${stderrPrefix}${line}\n`)
      }
      stderr.push(data.toString())
    })
  }

  return { stdout, stderr }
}

function padAllLines(text: string, padding: number): string {
  return text
    .split("\n")
    .map((line) => " ".repeat(padding) + line)
    .join("\n")
}

function write(writable: Writable, text: string): Promise<void> {
  return new Promise<void>((resolve, reject) => {
    writable.write(text, (err) => {
      if (err) return reject(err)
      return resolve()
    })
  })
}

export function script(shell: Pick<Shell, "dieOnErrors">): Script {
  const fnmDir = path.join(testTmpDir(), "fnm")
  rmSync(join(fnmDir, "aliases"), { recursive: true, force: true })
  return new Script({ fnmDir }, shell.dieOnErrors ? [shell.dieOnErrors()] : [])
}

function removeAllFnmEnvVars(obj: NodeJS.ProcessEnv): NodeJS.ProcessEnv {
  const result: NodeJS.ProcessEnv = {}
  for (const [key, value] of Object.entries(obj)) {
    if (!key.startsWith("FNM_")) {
      result[key] = value
    }
  }
  return result
}

function fnmTargetDir(): string {
  return path.join(
    process.cwd(),
    "target",
    process.env.FNM_TARGET_NAME ?? "debug"
  )
}
