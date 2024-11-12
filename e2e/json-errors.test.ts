import { script } from "./shellcode/script.js"
import { Bash, Fish, PowerShell, Zsh } from "./shellcode/shells.js"
import http from "node:http"
import describe from "./describe.js"
import getStderr from "./shellcode/get-stderr.js"

for (const shell of [Bash, Zsh, Fish, PowerShell]) {
  describe(shell, () => {
    test("`exec` usage", async () => {
      const server = http.createServer((_req, res) => {
        res.write(JSON.stringify([{ version: 666 }]))
        res.end()
      })

      try {
        server.listen(12345)

        await script(shell)
          .then(
            shell.env({
              nodeDistMirror: "http://localhost:12345",
            }),
          )
          .then(
            shell.scriptOutputContains(
              getStderr(shell.call("fnm", ["install", "v23"])),
              "'╰── invalid type: integer `666`'",
            ),
          )
          // .takeSnapshot(shell)
          .execute(shell)
      } finally {
        server.close()
      }
    })
  })
}
