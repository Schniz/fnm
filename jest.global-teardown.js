import { server } from "./tests/proxy-server/index.mjs"

export default () => {
  server.close()
}
