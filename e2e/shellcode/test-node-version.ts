import { HasCall } from "./shells/cmdCall.js"
import { ScriptLine } from "./shells/types.js"
import { HasExpectCommandOutput } from "./shells/expect-command-output.js"

export default function testNodeVersion<
  S extends HasCall & HasExpectCommandOutput
>(shell: S, version: string): ScriptLine {
  const nodeVersion = shell.call("node", ["--version"])
  return shell.hasCommandOutput(nodeVersion, version, "node version")
}
