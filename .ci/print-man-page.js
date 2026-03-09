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
        "Can't find debug target, please run `cargo build` or provide a specific binary path",
      )
    }
    return target.pathname
  },
}

const command = cmd.command({
  name: "print-man-page",
  description: "prints the man/*.1 files with updated contents",
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
    const targetFiles = await main(fnmPath)
    if (checkForDirty) {
      const gitStatus = await checkGitStatus(targetFiles)
      if (gitStatus.state === "dirty") {
        process.exitCode = 1
        console.error(
          "The files have changed. Please re-run `pnpm generate-man-page`.",
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
 * @param {string} fnmPath
 * @returns {Promise<string[]>}
 */
async function main(fnmPath) {
  const manDir = new URL("../man/", import.meta.url).pathname
  await fs.promises.mkdir(manDir, { recursive: true })
  const subcommands = await getSubcommands(fnmPath)
  const targets = [
    {
      path: `${manDir}fnm.1`,
      args: ["man"],
    },
    ...subcommands.map((name) => ({
      path: `${manDir}fnm-${name}.1`,
      args: ["man", name],
    })),
  ]

  for (const target of targets) {
    await writeManPage(fnmPath, target.path, target.args)
  }

  return targets.map((target) => target.path)
}

/**
 * @param {string} fnmPath
 * @param {string} targetFile
 * @param {string[]} args
 * @returns {Promise<void>}
 */
async function writeManPage(fnmPath, targetFile, args) {
  const result = await execa(fnmPath, args, {
    reject: false,
    stdout: "pipe",
    stderr: "pipe",
  })

  if (result.exitCode !== 0) {
    throw new Error(result.stderr || "Failed generating man page")
  }

  await fs.promises.writeFile(targetFile, result.stdout, "utf8")
}

/**
 * @param {string} fnmPath
 * @returns {Promise<string[]>}
 */
async function getSubcommands(fnmPath) {
  const result = await execa(fnmPath, ["--help"], {
    reject: false,
    stdout: "pipe",
    stderr: "pipe",
  })

  if (result.exitCode !== 0) {
    throw new Error(result.stderr || "Failed reading fnm --help")
  }

  const rows = result.stdout.split("\n")
  const commandsHeader = rows.findIndex((line) => line.trim() === "Commands:")

  if (commandsHeader === -1) {
    return []
  }

  const end = rows.indexOf("", commandsHeader + 1)
  const commandRows = rows.slice(
    commandsHeader + 1,
    end === -1 ? undefined : end,
  )

  /** @type {string[]} */
  const subcommands = []

  for (const row of commandRows) {
    const [name] = row.trim().split(/\s+/)
    if (!name) {
      continue
    }
    subcommands.push(name)
  }

  return subcommands
}

/**
 * @param {string[]} targetFiles
 * @returns {Promise<{ state: "dirty", diff: string } | { state: "clean" }>}
 */
async function checkGitStatus(targetFiles) {
  if (targetFiles.length === 0) {
    return { state: "clean" }
  }

  const { stdout, exitCode } = await execa(
    `git`,
    ["diff", "--color", "--exit-code", ...targetFiles],
    {
      reject: false,
    },
  )
  if (exitCode === 0) {
    return { state: "clean" }
  }
  return { state: "dirty", diff: stdout }
}
