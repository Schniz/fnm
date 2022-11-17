import { ScriptLine, define } from "./types"

type RedirectOutputOpts = { output: string }
export type HasRedirectOutput = {
  redirectOutput(childCommand: ScriptLine, opts: RedirectOutputOpts): string
}

export const redirectOutput = {
  bash: define<HasRedirectOutput>({
    redirectOutput: (childCommand, opts) => `${childCommand} > ${opts.output}`,
  }),
  powershell: define<HasRedirectOutput>({
    redirectOutput: (childCommand, opts) =>
      `${childCommand} | Out-File ${opts.output} -Encoding UTF8`,
  }),
}
