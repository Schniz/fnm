import { define } from "./types.js"

export type HasGetEnvVar = {
  getEnvVar(name: string): string
}

export const getEnvVar = {
  posix: define<HasGetEnvVar>({
    getEnvVar: (name) => `$${name}`,
  }),
  powershell: define<HasGetEnvVar>({
    getEnvVar: (name) => `$env:${name}`,
  }),
  winCmd: define<HasGetEnvVar>({
    getEnvVar: (name) => `%${name}%`,
  }),
}
