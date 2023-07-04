#!/usr/bin/env node

/// @ts-check

import { execa } from "execa"
import fs from "node:fs"
import cmd from "cmd-ts"
import cmdFs from "cmd-ts/dist/cjs/batteries/fs.js"

const FnmBinaryPath = {
  ...cmdFs.ExistingPath,
  defaultValue() {
    const target = new URL("../target/debug/fnm", import.meta.url)
    if (!fs.existsSync(target)) {
      throw new Error(
        "Can't find debug target, please run `cargo build` or provide a specific binary path"
      )
    }
    return target.pathname
  },
}

const command = cmd.command({
  name: "print-command-docs",
  description: "prints the docs/command.md file with updated contents",
  args: {
    checkForDirty: cmd.flag({
      long: "check",
      description: `Check that file was not changed`,
    }),
    fnmPath: cmd.option({
      long: "binary-path",
      description: "the fnm binary path",
      type: FnmBinaryPath,
    }),
  },
  async handler({ checkForDirty, fnmPath }) {
    const targetFile = new URL("../docs/commands.md", import.meta.url).pathname
    await main(targetFile, fnmPath)
    if (checkForDirty) {
      const gitStatus = await checkGitStatus(targetFile)
      if (gitStatus.state === "dirty") {
        process.exitCode = 1
        console.error(
          "The file has changed. Please re-run `pnpm generate-command-docs`."
        )
        console.error(`hint: The following diff was found:`)
        console.error()
        console.error(gitStatus.diff)
      }
    }
  },
})

cmd.run(cmd.binary(command), process.argv).catch((err) => {
  console.error(err)
  process.exitCode = process.exitCode || 1
})

/**
 * @param {string} targetFile
 * @param {string} fnmPath
 * @returns {Promise<void>}
 */
async function main(targetFile, fnmPath) {
  const stream = fs.createWriteStream(targetFile)

  const { subcommands, text: mainText } = await getCommandHelp(fnmPath)

  await write(stream, line(`fnm`, mainText))

  for (const subcommand of subcommands) {
    const { text: subcommandText } = await getCommandHelp(fnmPath, subcommand)
    await write(stream, "\n" + line(`fnm ${subcommand}`, subcommandText))
  }

  stream.close()

  await execa(`pnpm`, ["prettier", "--write", targetFile])
}

/**
 * @param {import('stream').Writable} stream
 * @param {string} content
 * @returns {Promise<void>}
 */
function write(stream, content) {
  return new Promise((resolve, reject) => {
    stream.write(content, (err) => (err ? reject(err) : resolve()))
  })
}

function line(cmd, text) {
  const cmdCode = "`" + cmd + "`"
  const textCode = "```\n" + text + "\n```"
  return `# ${cmdCode}\n${textCode}`
}

/**
 * @param {string} fnmPath
 * @param {string} [command]
 * @returns {Promise<{ subcommands: string[], text: string }>}
 */
async function getCommandHelp(fnmPath, command) {
  const cmdArg = command ? [command] : []
  const result = await run(fnmPath, [...cmdArg, "--help"])
  const text = result.stdout
  const rows = text.split("\n")
  const headerIndex = rows.findIndex((x) => x.includes("Commands:"))
  /** @type {string[]} */
  const subcommands = []
  if (!command) {
    for (const row of rows.slice(
      headerIndex + 1,
      rows.indexOf("", headerIndex + 1)
    )) {
      const [, word] = row.split(/\s+/)
      if (word && word[0].toLowerCase() === word[0]) {
        subcommands.push(word)
      }
    }
  }
  return {
    subcommands,
    text,
  }
}

/**
 * @param {string[]} args
 * @returns {import('execa').ExecaChildProcess<string>}
 */
function run(fnmPath, args) {
  return execa(fnmPath, args, {
    reject: false,
    stdout: "pipe",
    stderr: "pipe",
  })
}

/**
 * @param {string} targetFile
 * @returns {Promise<{ state: "dirty", diff: string } | { state: "clean" }>}
 */
async function checkGitStatus(targetFile) {
  const { stdout, exitCode } = await execa(
    `git`,
    ["diff", "--color", "--exit-code", targetFile],
    {
      reject: false,
    }
  )
  if (exitCode === 0) {
    return { state: "clean" }
  }
  return { state: "dirty", diff: stdout }
}
