import { createServer } from "node:http"
import { rm, writeFile } from "node:fs/promises"
import { join } from "node:path"
import describe from "./describe.js"
import getStderr from "./shellcode/get-stderr.js"
import { script } from "./shellcode/script.js"
import { Bash, Fish, PowerShell, Zsh } from "./shellcode/shells.js"
import testCwd from "./shellcode/test-cwd.js"
import testNodeVersion from "./shellcode/test-node-version.js"
import testTmpDir from "./shellcode/test-tmp-dir.js"

// Pin the release list while using the usual local proxy for real Node archives.
const mirror = createServer((request, response) => {
  if (request.url === "/index.json") {
    response.setHeader("Content-Type", "application/json")
    response.end(
      JSON.stringify([
        { version: "v20.14.0", lts: "Iron" },
        { version: "v18.20.3", lts: "Hydrogen" },
      ]),
    )
  } else {
    response.writeHead(302, {
      Location: `http://localhost:8080${request.url}`,
    })
    response.end()
  }
})
let mirrorUrl: string

beforeAll(async () => {
  await new Promise<void>((resolve) => mirror.listen(0, "127.0.0.1", resolve))
  const address = mirror.address()
  if (!address || typeof address === "string") {
    throw new Error("Expected a local TCP mirror")
  }
  mirrorUrl = `http://127.0.0.1:${address.port}`
})

afterAll(async () => {
  await new Promise<void>((resolve, reject) => {
    mirror.close((error) => (error ? reject(error) : resolve()))
  })
})

for (const shell of [Bash, Zsh, Fish, PowerShell]) {
  describe(shell, () => {
    test("issue #1506: installs node from .nvmrc as the latest release", async () => {
      await writeFile(join(testCwd(), ".nvmrc"), "node\n")
      await script(shell)
        .then(shell.env({ nodeDistMirror: mirrorUrl }))
        .then(shell.call("fnm", ["install"]))
        .then(shell.call("fnm", ["use"]))
        .then(testNodeVersion(shell, "v20.14.0"))
        .execute(shell)
    })

    test("issue #1506: installs a missing node request when using it", async () => {
      await rm(join(testTmpDir(), "fnm"), { recursive: true, force: true })
      await writeFile(join(testCwd(), ".nvmrc"), "node")
      await script(shell)
        .then(shell.env({ nodeDistMirror: mirrorUrl }))
        .then(
          shell.scriptOutputContains(
            getStderr(shell.call("fnm", ["use"])),
            "'Requested version node is not currently installed'",
          ),
        )
        .then(shell.call("fnm", ["use", "--install-if-missing"]))
        .then(testNodeVersion(shell, "v20.14.0"))
        .execute(shell)
    })

    test("issue #1506: installs an explicit node request", async () => {
      await script(shell)
        .then(shell.env({ nodeDistMirror: mirrorUrl }))
        .then(shell.call("fnm", ["install", "node", "--use"]))
        .then(testNodeVersion(shell, "v20.14.0"))
        .execute(shell)
    })

    test("issue #1506: node selects the latest installed version", async () => {
      await writeFile(join(testCwd(), ".nvmrc"), "node")
      await script(shell)
        .then(shell.env({ nodeDistMirror: mirrorUrl }))
        .then(shell.call("fnm", ["install", "18.20.3"]))
        .then(shell.call("fnm", ["install", "20.14.0"]))
        .then(
          shell.hasCommandOutput(
            shell.call("fnm", ["ls-remote", "--filter=node", "--latest"]),
            "v20.14.0 (Iron)",
            "latest remote node version",
          ),
        )
        .then(
          shell.scriptOutputContains(
            getStderr(shell.call("fnm", ["uninstall", "node"])),
            "'Too many versions had matched'",
          ),
        )
        .then(shell.call("fnm", ["alias", "18.20.3", "latest"]))
        .then(shell.call("fnm", ["use"]))
        .then(testNodeVersion(shell, "v20.14.0"))
        .then(
          shell.hasCommandOutput(
            shell.call("fnm", ["exec", "node", "--version"]),
            "v20.14.0",
            "node version from .nvmrc in exec",
          ),
        )
        .then(shell.call("fnm", ["use", "node"]))
        .then(testNodeVersion(shell, "v20.14.0"))
        .then(
          shell.hasCommandOutput(
            shell.call("fnm", ["exec", "--using=node", "node", "--version"]),
            "v20.14.0",
            "node version in exec",
          ),
        )
        .then(shell.call("fnm", ["default", "node"]))
        .then(
          shell.hasCommandOutput(
            shell.call("fnm", ["default"]),
            "v20.14.0",
            "default version",
          ),
        )
        .then(
          shell.scriptOutputContains(
            getStderr(shell.call("fnm", ["unalias", "node"])),
            "'Requested alias node not found'",
          ),
        )
        .then(shell.call("fnm", ["use", "20.14.0"]))
        .then(testNodeVersion(shell, "v20.14.0"))
        .execute(shell)
    })

    test("issue #1506: preserves custom aliases named node", async () => {
      await writeFile(join(testCwd(), ".nvmrc"), "node")
      await script(shell)
        .then(shell.env({ nodeDistMirror: mirrorUrl }))
        .then(shell.call("fnm", ["install", "18.20.3"]))
        .then(shell.call("fnm", ["install", "20.14.0"]))
        .then(shell.call("fnm", ["alias", "18.20.3", "node"]))
        .then(shell.call("fnm", ["use"]))
        .then(testNodeVersion(shell, "v18.20.3"))
        .then(
          shell.hasCommandOutput(
            shell.call("fnm", ["ls-remote", "--filter=node", "--latest"]),
            "v18.20.3 (Hydrogen)",
            "custom node alias in remote filter",
          ),
        )
        .then(
          shell.hasCommandOutput(
            shell.call("fnm", ["exec", "--using=node", "node", "--version"]),
            "v18.20.3",
            "custom node alias in exec",
          ),
        )
        .then(
          shell.scriptOutputContains(
            getStderr(shell.call("fnm", ["install", "node"])),
            "'The requested version is not installable: node'",
          ),
        )
        .then(shell.call("fnm", ["unalias", "node"]))
        .then(shell.call("fnm", ["use"]))
        .then(testNodeVersion(shell, "v20.14.0"))
        .execute(shell)
    })

    test("issue #1506: preserves a node alias for the system version", async () => {
      await script(shell)
        .then(shell.env({}))
        .then(shell.call("fnm", ["alias", "system", "node"]))
        .then(
          shell.scriptOutputContains(
            shell.call("fnm", ["use", "node"]),
            "'Bypassing fnm'",
          ),
        )
        .then(shell.call("fnm", ["unalias", "node"]))
        .execute(shell)
    })
  })
}
