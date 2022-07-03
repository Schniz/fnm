#!/usr/bin/env node

/// @ts-check

const fs = require("fs");
const cp = require("child_process");
const path = require("path");
const cmd = require("cmd-ts");
const toml = require("toml");
const assert = require("assert");

const CARGO_TOML_PATH = path.join(__dirname, "../Cargo.toml");

const command = cmd.command({
  name: "prepare-version",
  description: "Prepare a new fnm version",
  args: {},
  async handler({}) {
    updateCargoToml(await getPackageVersion());
    exec("cargo build --release");
    exec("yarn generate-command-docs --binary-path=./target/release/fnm");
    exec("./.ci/record_screen.sh");
  },
});

cmd.run(cmd.binary(command), process.argv);

//////////////////////
// Helper functions //
//////////////////////

/**
 * @returns {Promise<string>}
 */
async function getPackageVersion() {
  const pkgJson = await fs.promises.readFile(
    path.join(__dirname, "../package.json"),
    "utf8"
  );
  const version = JSON.parse(pkgJson).version;
  assert(version, "package.json version is not set");
  return version;
}

function updateCargoToml(nextVersion) {
  const cargoToml = fs.readFileSync(CARGO_TOML_PATH, "utf8");
  const cargoTomlContents = toml.parse(cargoToml);
  const currentVersion = cargoTomlContents.package.version;

  const newToml = cargoToml.replace(
    `version = "${currentVersion}"`,
    `version = "${nextVersion}"`
  );

  if (newToml === cargoToml) {
    console.error("Cargo.toml didn't change, error!");
    process.exitCode = 1;
    return;
  }

  fs.writeFileSync(CARGO_TOML_PATH, newToml, "utf8");

  return nextVersion;
}

function exec(command, env) {
  console.log(`$ ${command}`);
  return cp.execSync(command, {
    cwd: path.join(__dirname, ".."), // root of repo
    stdio: "inherit",
    env: { ...process.env, ...env },
  });
}

/**
 * @param {"patch" | "minor" | "major"} type
 * @param {string} version
 */
function changeVersion(type, version) {
  const [major, minor, patch] = version.split(".").map((x) => parseInt(x, 10));
  switch (type) {
    case "patch":
      return [major, minor, patch + 1].join(".");
    case "minor":
      return [major, minor + 1, 0].join(".");
    case "major":
      return [major + 1, 0, 0].join(".");
  }
}
