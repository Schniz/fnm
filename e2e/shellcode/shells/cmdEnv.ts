import { ScriptLine, define } from "./types.js"

type EnvConfig = {
  executableName: string
  useOnCd: boolean
  logLevel: string
  corepackEnabled: boolean
  resolveEngines: boolean
}
export type HasEnv = { env(cfg: Partial<EnvConfig>): ScriptLine }

function stringify(envConfig: Partial<EnvConfig> = {}) {
  const {
    useOnCd,
    logLevel,
    corepackEnabled,
    resolveEngines,
    executableName = "fnm",
  } = envConfig
  return [
    `${executableName} env`,
    useOnCd && "--use-on-cd",
    logLevel && `--log-level=${logLevel}`,
    corepackEnabled && "--corepack-enabled",
    resolveEngines && `--resolve-engines`,
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
