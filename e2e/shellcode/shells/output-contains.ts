import { define, ScriptLine } from "./types"

export type HasOutputContains = {
  scriptOutputContains(script: ScriptLine, substring: string): ScriptLine
}

export const cmdHasOutputContains = {
  bash: define<HasOutputContains>({
    scriptOutputContains: (script, substring) => {
      return `${script} | grep ${substring}`
    },
  }),
  powershell: define<HasOutputContains>({
    scriptOutputContains: (script, substring) => {
      const inner: string = `${script} | Select-String ${substring}`
      return `$($__out__ = (${inner}); if ($LASTEXITCODE -ne 0) { "WELP" } else { $__out__ })`
    },
  }),
}
