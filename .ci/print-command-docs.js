#!/usr/bin/env node

/// @ts-check

const execa = require("execa");
const path = require("path");
const fs = require("fs");
const cmd = require("cmd-ts");
const cmdFs = require("cmd-ts/dist/cjs/batteries/fs");

const FnmBinaryPath = {
  ...cmdFs.ExistingPath,
  defaultValue() {
    const target = path.join(__dirname, "../target/debug/fnm");
    if (!fs.existsSync(target)) {
      throw new Error(
        "Can't find debug target, please run `cargo build` or provide a specific binary path"
      );
    }
    return target;
  },
};

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
    const targetFile = path.join(__dirname, "../docs/commands.md");
    await main(targetFile, fnmPath);
    if (checkForDirty) {
      const gitStatus = await checkGitStatus(targetFile);
      if (gitStatus.state === "dirty") {
        process.exitCode = 1;
        console.error(
          "The file has changed. Please re-run `yarn generate-command-docs`."
        );
        console.error(`hint: The following diff was found:`);
        console.error();
        console.error(gitStatus.diff);
      }
    }
  },
});

cmd.run(cmd.binary(command), process.argv).catch((err) => {
  console.error(err);
  process.exitCode = process.exitCode || 1;
});

/**
 * @param {string} targetFile
 * @param {string} fnmPath
 * @returns {Promise<void>}
 */
async function main(targetFile, fnmPath) {
  const stream = fs.createWriteStream(targetFile);

  const { subcommands, text: mainText } = await getCommandHelp(fnmPath);

  await write(stream, line(`fnm`, mainText));

  for (const subcommand of subcommands) {
    const { text: subcommandText } = await getCommandHelp(fnmPath, subcommand);
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
 * @param {string} fnmPath
 * @param {string} [command]
 * @returns {Promise<{ subcommands: string[], text: string }>}
 */
async function getCommandHelp(fnmPath, command) {
  const cmdArg = command ? [command] : [];
  const result = await run(fnmPath, [...cmdArg, "--help"]);
  const text = removeEnvValuesFromHelpDoc(result.stdout);
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
 * @param {string} text
 * @returns {string}
 */
function removeEnvValuesFromHelpDoc(text) {
  return text.replace(/(\[env: [^=]+)[^\]]+(\])/g, "$1$2");
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
  });
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
  );
  if (exitCode === 0) {
    return { state: "clean" };
  }
  return { state: "dirty", diff: stdout };
}
