import { HasCall } from "./shells/cmdCall"
import { ScriptLine } from "./shells/types"
import { HasExpectCommandOutput } from "./shells/expect-command-output"

export default function testNodeVersion<
  S extends HasCall & HasExpectCommandOutput
>(shell: S, version: string): ScriptLine {
  const nodeVersion = shell.call("node", ["--version"])
  return shell.hasCommandOutput(nodeVersion, version, "node version")
}
