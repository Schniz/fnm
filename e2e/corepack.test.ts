import fs from "fs"
import { script } from "./shellcode/script.js"
import { Bash, Fish, PowerShell, Zsh } from "./shellcode/shells.js"
import describe from "./describe.js"
import path from "path"
import testCwd from "./shellcode/test-cwd.js"
import { createRequire } from "module"

const require = createRequire(import.meta.url)
const whichPath = require.resolve("which")

const nodescript = `
  const which = require(${JSON.stringify(whichPath)});
  const pnpmBinary = which.sync('pnpm')
  const nodeBinary = which.sync('node')

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

for (const shell of [Bash, Fish, PowerShell, Zsh]) {
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
        // .addExtraEnvVar("RUST_LOG", "fnm=debug")
        .execute(shell)
    })
  })
}
