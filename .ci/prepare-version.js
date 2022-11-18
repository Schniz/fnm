#!/usr/bin/env node

/// @ts-check

import fs from "fs"
import cp from "child_process"
import cmd from "cmd-ts"
import toml from "toml"
import assert from "assert"

const CARGO_TOML_PATH = new URL("../Cargo.toml", import.meta.url).pathname

const command = cmd.command({
  name: "prepare-version",
  description: "Prepare a new fnm version",
  args: {},
  async handler({}) {
    updateCargoToml(await getPackageVersion())
    exec("cargo build --release")
    exec("pnpm generate-command-docs --binary-path=./target/release/fnm")
    exec("./.ci/record_screen.sh")
  },
})

cmd.run(cmd.binary(command), process.argv)

//////////////////////
// Helper functions //
//////////////////////

/**
 * @returns {Promise<string>}
 */
async function getPackageVersion() {
  const pkgJson = await fs.promises.readFile(
    new URL("../package.json", import.meta.url),
    "utf8"
  )
  const version = JSON.parse(pkgJson).version
  assert(version, "package.json version is not set")
  return version
}

function updateCargoToml(nextVersion) {
  const cargoToml = fs.readFileSync(CARGO_TOML_PATH, "utf8")
  const cargoTomlContents = toml.parse(cargoToml)
  const currentVersion = cargoTomlContents.package.version

  const newToml = cargoToml.replace(
    `version = "${currentVersion}"`,
    `version = "${nextVersion}"`
  )

  if (newToml === cargoToml) {
    console.error("Cargo.toml didn't change, error!")
    process.exitCode = 1
    return
  }

  fs.writeFileSync(CARGO_TOML_PATH, newToml, "utf8")

  return nextVersion
}

function exec(command, env) {
  console.log(`$ ${command}`)
  return cp.execSync(command, {
    cwd: new URL("..", import.meta.url),
    stdio: "inherit",
    env: { ...process.env, ...env },
  })
}
