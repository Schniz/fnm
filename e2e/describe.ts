import { WinCmd } from "./shellcode/shells.js"
import { Shell } from "./shellcode/shells/types.js"
import { describe as _describe } from "vitest"

export default function describe(
  shell: Pick<Shell, "name">,
  fn: () => void
) {
  if (shell === WinCmd) {
    // wincmd tests do not work right now and I don't have a Windows machine to fix it
    // maybe in the future when I have some time to spin a VM to check what's happening.
    return _describe.skip("WinCmd", fn)
  }

  return _describe(shell.name(), fn)
}
