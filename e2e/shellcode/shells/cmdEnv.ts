import { ScriptLine, define } from "./types"

type EnvConfig = { useOnCd: boolean; logLevel: string }
export type HasEnv = { env(cfg: Partial<EnvConfig>): ScriptLine }

function stringify(envConfig: Partial<EnvConfig> = {}) {
  const { useOnCd, logLevel } = envConfig
  return [
    `fnm env`,
    useOnCd && "--use-on-cd",
    logLevel && `--log-level=${logLevel}`,
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
