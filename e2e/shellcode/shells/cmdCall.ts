import { define, ScriptLine } from "./types.js"

export type HasCall = {
  call: (binary: string, args: string[]) => ScriptLine
}

export const cmdCall = {
  all: define<HasCall>({
    call: (binary: string, args: string[]) =>
      `${binary} ${args.join(" ")}` as ScriptLine,
  }),
}
