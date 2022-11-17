import { ScriptLine, define } from "./types.js"
import quote from "shell-escape"

type HasInSubShell = { inSubShell: (script: ScriptLine) => ScriptLine }

export const cmdInSubShell = {
  bash: define<HasInSubShell>({
    inSubShell: (script) => `echo ${quote([script])} | bash`,
  }),
  zsh: define<HasInSubShell>({
    inSubShell: (script) => `echo ${quote([script])} | zsh`,
  }),
  fish: define<HasInSubShell>({
    inSubShell: (script) => `fish -c ${quote([script])}`,
  }),
  powershell: define<HasInSubShell>({
    inSubShell: (script) =>
      `echo '${script.replace(/'/g, "\\'")}' | pwsh -NoProfile`,
  }),
}
