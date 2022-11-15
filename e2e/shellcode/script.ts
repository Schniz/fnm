import { ScriptLine, Shell } from "./shells/types"
import execa from "execa"
import testTmpDir from "./test-tmp-dir"
import dedent from "ts-dedent"
import testCwd from "./test-cwd"
import { join } from "node:path"
import { writeFile } from "node:fs/promises"

class Script {
  constructor(
    private readonly config: {
      fnmDir: string
    },
    private readonly lines: ScriptLine[]
  ) {}
  then(line: ScriptLine): Script {
    return new Script(this.config, [...this.lines, line])
  }

  takeSnapshot(shell: Pick<Shell, "name">): this {
    const script = this.lines.join("\n")
    expect(script).toMatchSnapshot(shell.name())

    return this
  }

  async execute(
    shell: Pick<Shell, "binaryName" | "launchArgs" | "currentlySupported">
  ): Promise<void> {
    if (!shell.currentlySupported()) {
      return
    }

    const filename = join(testTmpDir(), "script")
    await writeFile(filename, [...this.lines, "exit 0"].join("\n"))

    const child = execa(shell.binaryName(), [...shell.launchArgs(), filename], {
      stdio: ["ignore", "pipe", "pipe"],
      cwd: testCwd(),
      env: {
        ...removeAllFnmEnvVars(process.env),
        FNM_DIR: this.config.fnmDir,
      },
      extendEnv: false,
      reject: false,
    })

    const finished = await child

    if (finished.failed) {
      console.error(
        dedent`
          Script failed.
            code ${finished.exitCode}
            signal ${finished.signal}

          stdout:
          ${padAllLines(finished.stdout, 2)}

          stderr:
          ${padAllLines(finished.stderr, 2)}
        `
      )

      throw new Error(
        `Script failed on ${testCwd()} with code ${finished.exitCode}`
      )
    }
  }
}

function padAllLines(text: string, padding: number): string {
  return text
    .split("\n")
    .map((line) => " ".repeat(padding) + line)
    .join("\n")
}

export function script(shell: Pick<Shell, "dieOnErrors">): Script {
  const fnmDir = `${testTmpDir()}/fnm`
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
