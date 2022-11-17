import { define, ScriptLine } from "./types.js"

export type HasOutputContains = {
  scriptOutputContains(script: ScriptLine, substring: string): ScriptLine
}

export const cmdHasOutputContains = {
  bash: define<HasOutputContains>({
    scriptOutputContains: (script, substring) => {
      return `(${script}) | grep ${substring} || (echo "Expected output to contain ${substring}" && exit 1)`
    },
  }),
  fish: define<HasOutputContains>({
    scriptOutputContains: (script, substring) => {
      return `begin; ${script}; end | grep ${substring}; or echo "Expected output to contain ${substring}" && exit 1`
    },
  }),
  powershell: define<HasOutputContains>({
    scriptOutputContains: (script, substring) => {
      const inner: string = `${script} | Select-String ${substring}`
      return `$($__out__ = $(${inner}); if ($__out__ -eq $null) { exit 1 } else { $__out__ })`
    },
  }),
}
