#!/usr/bin/env node

/// @ts-check

const execa = require("execa");
const path = require("path");
const fs = require("fs");

main();

async function main() {
  const targetFile = path.join(__dirname, "../docs/commands.md");
  const stream = fs.createWriteStream(targetFile);

  const { subcommands, text: mainText } = await getCommandHelp();

  await write(stream, line(`fnm`, mainText));

  for (const subcommand of subcommands) {
    const { text: subcommandText } = await getCommandHelp(subcommand);
    await write(stream, "\n" + line(`fnm ${subcommand}`, subcommandText));
  }

  stream.close();

  await execa(`yarn`, ["prettier", "--write", targetFile]);
}

/**
 * @param {import('stream').Writable} stream
 * @param {string} content
 * @returns {Promise<void>}
 */
function write(stream, content) {
  return new Promise((resolve, reject) => {
    stream.write(content, (err) => (err ? reject(err) : resolve()));
  });
}

function line(cmd, text) {
  const cmdCode = "`" + cmd + "`";
  const textCode = "```\n" + text + "\n```";
  return `# ${cmdCode}\n${textCode}`;
}

/**
 * @returns {Promise<{ subcommands: string[], text: string }>}
 */
async function getCommandHelp(command) {
  const cmdArg = command ? [command] : [];
  const result = await run([...cmdArg, "--help"]);
  const text = result.stdout;
  const rows = text.split("\n");
  const headerIndex = rows.findIndex((x) => x.includes("SUBCOMMANDS"));
  /** @type {string[]} */
  const subcommands = [];
  for (const row of rows.slice(headerIndex + 1)) {
    const matched = row.match(/^\s{4}(\w+)/);
    if (!matched) break;
    subcommands.push(matched[1]);
  }
  return {
    subcommands,
    text,
  };
}

/**
 * @param {string[]} args
 * @returns {import('execa').ExecaChildProcess<string>}
 */
function run(args) {
  const target = path.join(__dirname, "../target/debug/fnm");
  return execa(target, args, {
    reject: false,
    stdout: "pipe",
    stderr: "pipe",
  });
}

async function ensureNoChangedFilesInGit() {
  execa("git", ["status"]);
}
