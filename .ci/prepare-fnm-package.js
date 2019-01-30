#!/usr/bin/env node

const fs = require("fs");
const path = require("path");
const cp = require("child_process");
const jestDiff = require("jest-diff");

const version = require("../package.json").version;
const filepath = path.resolve(__dirname, "..", "library", "Fnm__Package.re");

main({ failOnDifference: process.argv[2] === "--fail-on-difference" });

function generateModule() {
  const moduleText = `
    let version = "${version}";
  `;
  return cp.execSync(`esy refmt`, { input: moduleText }).toString();
}

function readFile() {
  try {
    return fs.readFileSync(filepath, "utf8");
  } catch (e) {
    return "";
  }
}

function main({ failOnDifference }) {
  const result = generateModule();

  if (failOnDifference) {
    const currentContents = readFile();
    if (currentContents !== result) {
      console.log(jestDiff(result, currentContents));
      console.log(
        "Fnm__Package.re is outdated! Please update it with `esy update-fnm-package`."
      );
      process.exit(1);
    }
  }

  fs.writeFileSync(filepath, result);
}
