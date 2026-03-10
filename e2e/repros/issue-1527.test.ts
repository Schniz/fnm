import { writeFile } from "node:fs/promises"
import path from "node:path"
import { script } from "../shellcode/script.js"
import { Bash, Fish, PowerShell, Zsh } from "../shellcode/shells.js"
import describe from "../describe.js"
import testCwd from "../shellcode/test-cwd.js"

const findCowsayInPath = `
const fs = require("node:fs")
const path = require("node:path")

const pathEntries = (process.env.PATH || "").split(path.delimiter)
const candidates = process.platform === "win32"
  ? ["cowsay.cmd", "cowsay.exe", "cowsay"]
  : ["cowsay"]

let found = false
for (const entry of pathEntries) {
  for (const candidate of candidates) {
    if (fs.existsSync(path.join(entry, candidate))) {
      found = true
      break
    }
  }

  if (found) {
    break
  }
}

console.log(found ? "found" : "missing")
`

for (const shell of [Bash, Zsh, Fish, PowerShell]) {
  describe(shell, () => {
    test(`issue #1527: cowsay is not in PATH after uninstall on node 16`, async () => {
      const checkPathScript = path.join(testCwd(), "check-cowsay-in-path.js")
      await writeFile(checkPathScript, findCowsayInPath)

      await script(shell)
        .then(shell.env({}))
        .then(shell.call("fnm", ["install", "16"]))
        .then(shell.call("fnm", ["install", "18"]))
        .then(shell.call("fnm", ["use", "16"]))
        .then(shell.call("npm", ["install", "-g", "cowsay"]))
        .then(shell.call("fnm", ["use", "18"]))
        .then(shell.call("npm", ["install", "-g", "cowsay"]))
        .then(shell.call("fnm", ["use", "16"]))
        .then(shell.call("npm", ["uninstall", "-g", "cowsay"]))
        .then(
          shell.scriptOutputContains(
            shell.call("node", ["check-cowsay-in-path.js"]),
            "missing",
          ),
        )
        .execute(shell)
    })
  })
}
