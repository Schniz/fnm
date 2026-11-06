import { script } from "./shellcode/script.js"
import { Bash, Fish, PowerShell, WinCmd, Zsh } from "./shellcode/shells.js"
import testCwd from "./shellcode/test-cwd.js"
import fs from "node:fs/promises"
import path from "node:path"
import describe from "./describe.js"

for (const shell of [Bash, Zsh, Fish, PowerShell, WinCmd]) {
  describe(shell, () => {
    const shellTest = shell.currentlySupported() ? test : test.skip
    shellTest("exec preserves ordinary exit codes", async () => {
      await fs.writeFile(
        path.join(testCwd(), "exit-codes.cjs"),
        `const assert = require("assert");
const { spawnSync } = require("child_process");
for (const code of [0, 1, 42, 143]) {
  const result = spawnSync("fnm", ["exec", "--using=system", "--", process.execPath, "-e", "process.exit(" + code + ")"], { encoding: "utf8", timeout: 10000 });
  assert.ifError(result.error);
  assert.strictEqual(result.status, code);
  assert.strictEqual(result.signal, null);
  assert.strictEqual(result.stderr, "");
}
const missing = spawnSync("fnm", ["exec", "--using=system", "--", "fnm-649-nonexistent-command"], { encoding: "utf8", timeout: 10000 });
assert.ifError(missing.error);
assert.strictEqual(missing.status, 1);
assert.ok(missing.stderr.includes("Can't spawn program:"));
console.log("passed");`,
      )

      await script(shell)
        .then(
          shell.hasCommandOutput(
            shell.call("node", ["exit-codes.cjs"]),
            "passed",
            "exec exit status assertions",
          ),
        )
        .execute(shell)
    })

    const unixTest = process.platform === "win32" ? test.skip : shellTest
    unixTest("exec returns 128 + signal for issue #649", async () => {
      await fs.writeFile(
        path.join(testCwd(), "signals.cjs"),
        `const assert = require("assert");
const { spawnSync } = require("child_process");
const { signals } = require("os").constants;
for (const signal of ["SIGTERM", "SIGKILL", "SIGINT"]) {
  const args = ["-e", "process.kill(process.pid, '" + signal + "')"];
  const direct = spawnSync(process.execPath, args, { timeout: 10000 });
  assert.ifError(direct.error);
  assert.strictEqual(direct.status, null);
  assert.strictEqual(direct.signal, signal);
  const result = spawnSync("fnm", ["exec", "--using=system", "--", process.execPath, ...args], { encoding: "utf8", timeout: 10000 });
  assert.ifError(result.error);
  assert.strictEqual(result.status, 128 + signals[signal], signal);
  assert.strictEqual(result.signal, null);
  assert.strictEqual(result.stderr, "");
}
console.log("passed");`,
      )

      await script(shell)
        .then(
          shell.hasCommandOutput(
            shell.call("node", ["signals.cjs"]),
            "passed",
            "exec exit status assertions",
          ),
        )
        .execute(shell)
    })
  })
}
