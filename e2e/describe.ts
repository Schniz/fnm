import { Shell } from "./shellcode/shells/types"

export default function describe(
  shell: Pick<Shell, "name">,
  fn: () => void
): void {
  globalThis.describe(shell.name(), fn)
}
