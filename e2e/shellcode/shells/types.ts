export type Shell = {
  escapeText(str: string): string
  binaryName(): string
  currentlySupported(): boolean
  name(): string
  launchArgs(): string[]
  dieOnErrors?(): string
  forceFile?: true | string
}

export type ScriptLine = string

export function define<S>(s: S): S {
  return s
}
