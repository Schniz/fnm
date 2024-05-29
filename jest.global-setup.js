import { server } from "./tests/proxy-server/index.mjs"

export default function () {
  server.listen(8080)
}
