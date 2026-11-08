import { script } from "./shellcode/script.js"
import { Bash, Fish, PowerShell, Zsh } from "./shellcode/shells.js"
import describe from "./describe.js"
import { writeFileSync } from "node:fs"
import { join } from "node:path"
import testTmpDir from "./shellcode/test-tmp-dir.js"

for (const shell of [Bash, Zsh, Fish, PowerShell]) {
  describe(shell, () => {
    test.each([
      {
        name: "removes multiple versions",
        installed: ["12.0.0", "14.0.0"],
        args: ["12.0.0", "14.0.0"],
        remaining: [],
        status: 0,
        errors: []
      },
      {
        name: "continues after missing, ambiguous and system targets",
        installed: ["12.0.0", "12.1.0", "14.0.0"],
        args: ["99.0.0", "12", "system", "14.0.0"],
        remaining: ["12.0.0", "12.1.0"],
        status: 1,
        errors: [
          "Can't find a matching version",
          "Too many versions",
          "Can't uninstall system"
        ]
      },
      {
        name: "skips repeated versions and removed aliases",
        installed: ["12.0.0", "14.0.0"],
        args: ["hello", "12.0.0", "hello", "14.0.0"],
        remaining: [],
        status: 1,
        errors: ["Can't find a matching version"]
      },
      {
        name: "resolves each target against the remaining versions",
        installed: ["12.0.0", "12.1.0", "14.0.0"],
        args: ["12.0.0", "12", "14.0.0"],
        remaining: [],
        status: 0,
        errors: []
      },
      {
        name: "preserves single-target errors",
        installed: ["12.0.0", "14.0.0"],
        args: ["99.0.0"],
        remaining: ["12.0.0", "14.0.0"],
        status: 1,
        errors: ["Can't find a matching version"]
      },
      {
        name: "infers a version when no arguments are supplied",
        installed: ["12.0.0", "14.0.0"],
        args: [],
        remaining: ["14.0.0"],
        status: 0,
        errors: []
      }
    ])("$name", async ({ installed, args, remaining, status, errors }) => {
      const filename = join(testTmpDir(), "uninstall.cjs")
      writeFileSync(
        filename,
        `
          const fs = require('node:fs');
          const path = require('node:path');
          const assert = require('node:assert/strict');
          const { spawnSync } = require('node:child_process');
          const root = process.env.FNM_DIR;
          const installations = path.join(root, 'node-versions');
          fs.rmSync(root, { recursive: true, force: true });
          for (const version of ${JSON.stringify(installed)}) {
            fs.mkdirSync(path.join(installations, 'v' + version, 'installation'), { recursive: true });
          }
          assert.equal(spawnSync('fnm', ['alias', '12.0.0', 'hello']).status, 0);
          fs.writeFileSync('.node-version', '12.0.0');
          const result = spawnSync('fnm', ['uninstall', ...${JSON.stringify(args)}], { encoding: 'utf8' });
          assert.equal(result.status, ${status}, result.stderr);
          for (const error of ${JSON.stringify(errors)}) assert.ok(result.stderr.includes(error), result.stderr);
          assert.deepEqual(fs.readdirSync(installations).sort(), ${JSON.stringify(remaining.map((v) => `v${v}`).sort())});
          if (!${JSON.stringify(remaining)}.includes('12.0.0')) assert.deepEqual(fs.readdirSync(path.join(root, 'aliases')), []);
        `
      )
      await script(shell)
        .then(shell.call("node", [JSON.stringify(filename)]))
        .execute(shell)
    })

    if (process.platform !== "win32") {
      test("continues after a deletion fails", async () => {
        const filename = join(testTmpDir(), "uninstall.cjs")
        writeFileSync(
          filename,
          `
            const fs = require('node:fs');
            const path = require('node:path');
            const assert = require('node:assert/strict');
            const { spawnSync } = require('node:child_process');
            const root = process.env.FNM_DIR;
            const installations = path.join(root, 'node-versions');
            fs.rmSync(root, { recursive: true, force: true });
            for (const version of ['v12.0.0', 'v14.0.0']) {
              fs.mkdirSync(path.join(installations, version, 'installation'), { recursive: true });
            }
            const blocked = path.join(installations, 'v12.0.0');
            fs.chmodSync(blocked, 0o555);
            try {
              const result = spawnSync('fnm', ['uninstall', '12.0.0', '14.0.0'], { encoding: 'utf8' });
              assert.equal(result.status, 1, result.stderr);
              assert.ok(result.stderr.includes("Can't delete Node.js version"), result.stderr);
              assert.deepEqual(fs.readdirSync(installations), ['v12.0.0']);
            } finally {
              fs.chmodSync(blocked, 0o755);
            }
          `
        )
        await script(shell)
          .then(shell.call("node", [JSON.stringify(filename)]))
          .execute(shell)
      })
    }
  })
}
