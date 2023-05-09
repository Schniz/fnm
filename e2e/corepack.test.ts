import fs from "fs"
import { script } from "./shellcode/script.js"
import { Bash, Fish, PowerShell, Zsh } from "./shellcode/shells.js"
import describe from "./describe.js"
import path from "path"
import testCwd from "./shellcode/test-cwd.js"

const nodescript = `
  const pnpmBinary = require('child_process').execSync('which pnpm', { encoding: 'utf8' }).trim()
  const nodeBinary = require('child_process').execSync('which node', { encoding: 'utf8' }).trim()

  const binPath = require('path').dirname(nodeBinary);

  if (!pnpmBinary.includes(binPath)) {
    console.log('pnpm not found in current Node.js bin', { binPath, pnpmBinary });
    process.exit(1);
  }
  const scriptContents = require('fs').readFileSync(pnpmBinary, 'utf8');
  console.log('scriptContents', scriptContents)
  if (!scriptContents.includes('corepack')) {
    console.log('corepack not found in pnpm script');
    process.exit(1);
  }
`

for (const shell of [Bash]) {
  describe(shell, () => {
    test(`installs corepack`, async () => {
      const cwd = testCwd()
      const filepath = path.join(cwd, "test-pnpm-corepack.js")
      fs.writeFileSync(filepath, nodescript)

      await script(shell)
        .then(shell.env({ corepackEnabled: true }))
        .then(shell.call("fnm", ["install", "18"]))
        .then(
          shell.call("fnm", [
            "exec",
            "--using=18",
            "node",
            "test-pnpm-corepack.js",
          ])
        )
        .takeSnapshot(shell)
        .execute(shell)
    })
  })
}
