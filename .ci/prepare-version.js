#!/usr/bin/env node

const fs = require("fs");
const cp = require("child_process");
const ARGUMENTS = process.argv.slice(2);

const versions = {
  patch: "patch",
  minor: "minor",
  major: "major"
};

if (!ARGUMENTS[0]) {
  console.log(
    [
      "esy version:prepare, prepare a new fnm version",
      "",
      "Usage:",
      "------",
      "",
      "  esy version:prepare patch - to prepare a patch version (X.X.X+1)",
      "  esy version:prepare minor - to prepare a minor version (X.X+1.0)",
      "  esy version:prepare major - to prepare a major version (X+1.0.0)"
    ].join("\n")
  );
  process.exit(1);
}
const versionType = versions[ARGUMENTS[0].toLowerCase()];
if (!versionType) {
  throw new Error("Version (argument 0) must be one of major/minor/patch.");
}

const pkgJson = JSON.parse(fs.readFileSync("./package.json", "utf8"));
pkgJson.version = changeVersion(versionType, pkgJson.version);
fs.writeFileSync("./package.json", JSON.stringify(pkgJson, null, 2));

exec("git fetch origin");
exec("esy update-fnm-package");
exec("esy verify-fnm-package");
exec("esy build");
exec("./docs/record_screen.sh");
exec(`esy changelog`, { NEXT_VERSION: `v${pkgJson.version}` });

function exec(command, env) {
  console.log(`$ ${command}`);
  return cp.execSync(command, {
    stdio: "inherit",
    env: { ...process.env, ...env }
  });
}

function changeVersion(type, version) {
  const [major, minor, patch] = version.split(".").map(x => parseFloat(x, 10));
  switch (type) {
    case "patch":
      return [major, minor, patch + 1].join(".");
    case "minor":
      return [major, minor + 1, 0].join(".");
    case "major":
      return [major + 1, 0, 0].join(".");
  }
}
