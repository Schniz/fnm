import { ScriptLine, define } from "./types.js"

type EnvConfig = { useOnCd: boolean; logLevel: string; args: string[] }
export type HasEnv = { env(cfg: Partial<EnvConfig>): ScriptLine }

function stringify(config: Partial<EnvConfig> = {}) {
  return [
    `fnm env`,
    config.useOnCd && "--use-on-cd",
    config.logLevel && `--log-level=${config.logLevel}`,
    ...(config.args ?? []),
  ]
    .filter(Boolean)
    .join(" ")
}

export const cmdEnv = {
  bash: define<HasEnv>({
    env: (envConfig) => `eval "$(${stringify(envConfig)})"`,
  }),
  fish: define<HasEnv>({
    env: (envConfig) => `${stringify(envConfig)} | source`,
  }),
  powershell: define<HasEnv>({
    env: (envConfig) =>
      `${stringify(envConfig)} | Out-String | Invoke-Expression`,
  }),
  wincmd: define<HasEnv>({
    env: (envConfig) =>
      `FOR /f "tokens=*" %i IN ('${stringify(envConfig)}') DO CALL %i`,
  }),
}
