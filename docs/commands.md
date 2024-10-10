# `fnm`

```
A fast and simple Node.js manager

Usage: fnm [OPTIONS] <COMMAND>

Commands:
  list-remote  List all remote Node.js versions [aliases: ls-remote]
  list         List all locally installed Node.js versions [aliases: ls]
  install      Install a new Node.js version [aliases: i]
  use          Change Node.js version
  env          Print and set up required environment variables for fnm
  completions  Print shell completions to stdout
  alias        Alias a version to a common name
  unalias      Remove an alias definition
  default      Set a version as the default version
  current      Print the current Node.js version
  exec         Run a command within fnm context
  uninstall    Uninstall a Node.js version [aliases: uni]
  help         Print this message or the help of the given subcommand(s)

Options:
      --node-dist-mirror <NODE_DIST_MIRROR>
          <https://nodejs.org/dist/> mirror

          [env: FNM_NODE_DIST_MIRROR]
          [default: https://nodejs.org/dist]

      --fnm-dir <BASE_DIR>
          The root directory of fnm installations

          [env: FNM_DIR]

      --log-level <LOG_LEVEL>
          The log level of fnm commands

          [env: FNM_LOGLEVEL]
          [default: info]
          [possible values: quiet, error, info]

      --arch <ARCH>
          Override the architecture of the installed Node binary. Defaults to arch of fnm binary

          [env: FNM_ARCH]

      --version-file-strategy <VERSION_FILE_STRATEGY>
          A strategy for how to resolve the Node version. Used whenever `fnm use` or `fnm install` is called without a version, or when `--use-on-cd` is configured on evaluation

          [env: FNM_VERSION_FILE_STRATEGY]
          [default: local]

          Possible values:
          - local:     Use the local version of Node defined within the current directory
          - recursive: Use the version of Node defined within the current directory and all parent directories

      --corepack-enabled
          Enable corepack support for each new installation. This will make fnm call `corepack enable` on every Node.js installation. For more information about corepack see <https://nodejs.org/api/corepack.html>

          [env: FNM_COREPACK_ENABLED]

      --resolve-engines
          Resolve `engines.node` field in `package.json` whenever a `.node-version` or `.nvmrc` file is not present.
          Experimental: This feature is subject to change.
          Note: `engines.node` can be any semver range, with the latest satisfying version being resolved.

          [env: FNM_RESOLVE_ENGINES]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

# `fnm list-remote`

```
List all remote Node.js versions

Usage: fnm list-remote [OPTIONS]

Options:
      --filter <FILTER>
          Filter versions by a user-defined version or a semver range

      --node-dist-mirror <NODE_DIST_MIRROR>
          <https://nodejs.org/dist/> mirror

          [env: FNM_NODE_DIST_MIRROR]
          [default: https://nodejs.org/dist]

      --fnm-dir <BASE_DIR>
          The root directory of fnm installations

          [env: FNM_DIR]

      --lts [<LTS>]
          Show only LTS versions (optionally filter by LTS codename)

      --sort <SORT>
          Version sorting order

          [default: asc]

          Possible values:
          - desc: Sort versions in descending order (latest to earliest)
          - asc:  Sort versions in ascending order (earliest to latest)

      --latest
          Only show the latest matching version

      --log-level <LOG_LEVEL>
          The log level of fnm commands

          [env: FNM_LOGLEVEL]
          [default: info]
          [possible values: quiet, error, info]

      --arch <ARCH>
          Override the architecture of the installed Node binary. Defaults to arch of fnm binary

          [env: FNM_ARCH]

      --version-file-strategy <VERSION_FILE_STRATEGY>
          A strategy for how to resolve the Node version. Used whenever `fnm use` or `fnm install` is called without a version, or when `--use-on-cd` is configured on evaluation

          [env: FNM_VERSION_FILE_STRATEGY]
          [default: local]

          Possible values:
          - local:     Use the local version of Node defined within the current directory
          - recursive: Use the version of Node defined within the current directory and all parent directories

      --corepack-enabled
          Enable corepack support for each new installation. This will make fnm call `corepack enable` on every Node.js installation. For more information about corepack see <https://nodejs.org/api/corepack.html>

          [env: FNM_COREPACK_ENABLED]

      --resolve-engines
          Resolve `engines.node` field in `package.json` whenever a `.node-version` or `.nvmrc` file is not present.
          Experimental: This feature is subject to change.
          Note: `engines.node` can be any semver range, with the latest satisfying version being resolved.

          [env: FNM_RESOLVE_ENGINES]

  -h, --help
          Print help (see a summary with '-h')
```

# `fnm list`

```
List all locally installed Node.js versions

Usage: fnm list [OPTIONS]

Options:
      --node-dist-mirror <NODE_DIST_MIRROR>
          <https://nodejs.org/dist/> mirror

          [env: FNM_NODE_DIST_MIRROR]
          [default: https://nodejs.org/dist]

      --fnm-dir <BASE_DIR>
          The root directory of fnm installations

          [env: FNM_DIR]

      --log-level <LOG_LEVEL>
          The log level of fnm commands

          [env: FNM_LOGLEVEL]
          [default: info]
          [possible values: quiet, error, info]

      --arch <ARCH>
          Override the architecture of the installed Node binary. Defaults to arch of fnm binary

          [env: FNM_ARCH]

      --version-file-strategy <VERSION_FILE_STRATEGY>
          A strategy for how to resolve the Node version. Used whenever `fnm use` or `fnm install` is called without a version, or when `--use-on-cd` is configured on evaluation

          [env: FNM_VERSION_FILE_STRATEGY]
          [default: local]

          Possible values:
          - local:     Use the local version of Node defined within the current directory
          - recursive: Use the version of Node defined within the current directory and all parent directories

      --corepack-enabled
          Enable corepack support for each new installation. This will make fnm call `corepack enable` on every Node.js installation. For more information about corepack see <https://nodejs.org/api/corepack.html>

          [env: FNM_COREPACK_ENABLED]

      --resolve-engines
          Resolve `engines.node` field in `package.json` whenever a `.node-version` or `.nvmrc` file is not present.
          Experimental: This feature is subject to change.
          Note: `engines.node` can be any semver range, with the latest satisfying version being resolved.

          [env: FNM_RESOLVE_ENGINES]

  -h, --help
          Print help (see a summary with '-h')
```

# `fnm install`

```
Install a new Node.js version

Usage: fnm install [OPTIONS] [VERSION]

Arguments:
  [VERSION]
          A version string. Can be a partial semver or a LTS version name by the format lts/NAME

Options:
      --lts
          Install latest LTS

      --node-dist-mirror <NODE_DIST_MIRROR>
          <https://nodejs.org/dist/> mirror

          [env: FNM_NODE_DIST_MIRROR]
          [default: https://nodejs.org/dist]

      --fnm-dir <BASE_DIR>
          The root directory of fnm installations

          [env: FNM_DIR]

      --latest
          Install latest version

      --progress <PROGRESS>
          Show an interactive progress bar for the download status

          [default: auto]
          [possible values: auto, never, always]

      --log-level <LOG_LEVEL>
          The log level of fnm commands

          [env: FNM_LOGLEVEL]
          [default: info]
          [possible values: quiet, error, info]

      --arch <ARCH>
          Override the architecture of the installed Node binary. Defaults to arch of fnm binary

          [env: FNM_ARCH]

      --version-file-strategy <VERSION_FILE_STRATEGY>
          A strategy for how to resolve the Node version. Used whenever `fnm use` or `fnm install` is called without a version, or when `--use-on-cd` is configured on evaluation

          [env: FNM_VERSION_FILE_STRATEGY]
          [default: local]

          Possible values:
          - local:     Use the local version of Node defined within the current directory
          - recursive: Use the version of Node defined within the current directory and all parent directories

      --corepack-enabled
          Enable corepack support for each new installation. This will make fnm call `corepack enable` on every Node.js installation. For more information about corepack see <https://nodejs.org/api/corepack.html>

          [env: FNM_COREPACK_ENABLED]

      --resolve-engines
          Resolve `engines.node` field in `package.json` whenever a `.node-version` or `.nvmrc` file is not present.
          Experimental: This feature is subject to change.
          Note: `engines.node` can be any semver range, with the latest satisfying version being resolved.

          [env: FNM_RESOLVE_ENGINES]

  -h, --help
          Print help (see a summary with '-h')
```

# `fnm use`

```
Change Node.js version

Usage: fnm use [OPTIONS] [VERSION]

Arguments:
  [VERSION]


Options:
      --install-if-missing
          Install the version if it isn't installed yet

      --node-dist-mirror <NODE_DIST_MIRROR>
          <https://nodejs.org/dist/> mirror

          [env: FNM_NODE_DIST_MIRROR]
          [default: https://nodejs.org/dist]

      --fnm-dir <BASE_DIR>
          The root directory of fnm installations

          [env: FNM_DIR]

      --silent-if-unchanged
          Don't output a message identifying the version being used if it will not change due to execution of this command

      --log-level <LOG_LEVEL>
          The log level of fnm commands

          [env: FNM_LOGLEVEL]
          [default: info]
          [possible values: quiet, error, info]

      --arch <ARCH>
          Override the architecture of the installed Node binary. Defaults to arch of fnm binary

          [env: FNM_ARCH]

      --version-file-strategy <VERSION_FILE_STRATEGY>
          A strategy for how to resolve the Node version. Used whenever `fnm use` or `fnm install` is called without a version, or when `--use-on-cd` is configured on evaluation

          [env: FNM_VERSION_FILE_STRATEGY]
          [default: local]

          Possible values:
          - local:     Use the local version of Node defined within the current directory
          - recursive: Use the version of Node defined within the current directory and all parent directories

      --corepack-enabled
          Enable corepack support for each new installation. This will make fnm call `corepack enable` on every Node.js installation. For more information about corepack see <https://nodejs.org/api/corepack.html>

          [env: FNM_COREPACK_ENABLED]

      --resolve-engines
          Resolve `engines.node` field in `package.json` whenever a `.node-version` or `.nvmrc` file is not present.
          Experimental: This feature is subject to change.
          Note: `engines.node` can be any semver range, with the latest satisfying version being resolved.

          [env: FNM_RESOLVE_ENGINES]

  -h, --help
          Print help (see a summary with '-h')
```

# `fnm env`

```
Print and set up required environment variables for fnm

This command generates a series of shell commands that should be evaluated by your shell to create a fnm-ready environment.

Each shell has its own syntax of evaluating a dynamic expression. For example, evaluating fnm on Bash and Zsh would look like `eval "$(fnm env)"`. In Fish, evaluating would look like `fnm env | source`

Usage: fnm env [OPTIONS]

Options:
      --node-dist-mirror <NODE_DIST_MIRROR>
          <https://nodejs.org/dist/> mirror

          [env: FNM_NODE_DIST_MIRROR]
          [default: https://nodejs.org/dist]

      --shell <SHELL>
          The shell syntax to use. Infers when missing

          [possible values: bash, zsh, fish, powershell]

      --fnm-dir <BASE_DIR>
          The root directory of fnm installations

          [env: FNM_DIR]

      --json
          Print JSON instead of shell commands

      --log-level <LOG_LEVEL>
          The log level of fnm commands

          [env: FNM_LOGLEVEL]
          [default: info]
          [possible values: quiet, error, info]

      --use-on-cd
          Print the script to change Node versions every directory change

      --arch <ARCH>
          Override the architecture of the installed Node binary. Defaults to arch of fnm binary

          [env: FNM_ARCH]

      --version-file-strategy <VERSION_FILE_STRATEGY>
          A strategy for how to resolve the Node version. Used whenever `fnm use` or `fnm install` is called without a version, or when `--use-on-cd` is configured on evaluation

          [env: FNM_VERSION_FILE_STRATEGY]
          [default: local]

          Possible values:
          - local:     Use the local version of Node defined within the current directory
          - recursive: Use the version of Node defined within the current directory and all parent directories

      --corepack-enabled
          Enable corepack support for each new installation. This will make fnm call `corepack enable` on every Node.js installation. For more information about corepack see <https://nodejs.org/api/corepack.html>

          [env: FNM_COREPACK_ENABLED]

      --resolve-engines
          Resolve `engines.node` field in `package.json` whenever a `.node-version` or `.nvmrc` file is not present.
          Experimental: This feature is subject to change.
          Note: `engines.node` can be any semver range, with the latest satisfying version being resolved.

          [env: FNM_RESOLVE_ENGINES]

  -h, --help
          Print help (see a summary with '-h')
```

# `fnm completions`

```
Print shell completions to stdout

Usage: fnm completions [OPTIONS]

Options:
      --node-dist-mirror <NODE_DIST_MIRROR>
          <https://nodejs.org/dist/> mirror

          [env: FNM_NODE_DIST_MIRROR]
          [default: https://nodejs.org/dist]

      --shell <SHELL>
          The shell syntax to use. Infers when missing

          [possible values: bash, zsh, fish, powershell]

      --fnm-dir <BASE_DIR>
          The root directory of fnm installations

          [env: FNM_DIR]

      --log-level <LOG_LEVEL>
          The log level of fnm commands

          [env: FNM_LOGLEVEL]
          [default: info]
          [possible values: quiet, error, info]

      --arch <ARCH>
          Override the architecture of the installed Node binary. Defaults to arch of fnm binary

          [env: FNM_ARCH]

      --version-file-strategy <VERSION_FILE_STRATEGY>
          A strategy for how to resolve the Node version. Used whenever `fnm use` or `fnm install` is called without a version, or when `--use-on-cd` is configured on evaluation

          [env: FNM_VERSION_FILE_STRATEGY]
          [default: local]

          Possible values:
          - local:     Use the local version of Node defined within the current directory
          - recursive: Use the version of Node defined within the current directory and all parent directories

      --corepack-enabled
          Enable corepack support for each new installation. This will make fnm call `corepack enable` on every Node.js installation. For more information about corepack see <https://nodejs.org/api/corepack.html>

          [env: FNM_COREPACK_ENABLED]

      --resolve-engines
          Resolve `engines.node` field in `package.json` whenever a `.node-version` or `.nvmrc` file is not present.
          Experimental: This feature is subject to change.
          Note: `engines.node` can be any semver range, with the latest satisfying version being resolved.

          [env: FNM_RESOLVE_ENGINES]

  -h, --help
          Print help (see a summary with '-h')
```

# `fnm alias`

```
Alias a version to a common name

Usage: fnm alias [OPTIONS] <TO_VERSION> <NAME>

Arguments:
  <TO_VERSION>


  <NAME>


Options:
      --node-dist-mirror <NODE_DIST_MIRROR>
          <https://nodejs.org/dist/> mirror

          [env: FNM_NODE_DIST_MIRROR]
          [default: https://nodejs.org/dist]

      --fnm-dir <BASE_DIR>
          The root directory of fnm installations

          [env: FNM_DIR]

      --log-level <LOG_LEVEL>
          The log level of fnm commands

          [env: FNM_LOGLEVEL]
          [default: info]
          [possible values: quiet, error, info]

      --arch <ARCH>
          Override the architecture of the installed Node binary. Defaults to arch of fnm binary

          [env: FNM_ARCH]

      --version-file-strategy <VERSION_FILE_STRATEGY>
          A strategy for how to resolve the Node version. Used whenever `fnm use` or `fnm install` is called without a version, or when `--use-on-cd` is configured on evaluation

          [env: FNM_VERSION_FILE_STRATEGY]
          [default: local]

          Possible values:
          - local:     Use the local version of Node defined within the current directory
          - recursive: Use the version of Node defined within the current directory and all parent directories

      --corepack-enabled
          Enable corepack support for each new installation. This will make fnm call `corepack enable` on every Node.js installation. For more information about corepack see <https://nodejs.org/api/corepack.html>

          [env: FNM_COREPACK_ENABLED]

      --resolve-engines
          Resolve `engines.node` field in `package.json` whenever a `.node-version` or `.nvmrc` file is not present.
          Experimental: This feature is subject to change.
          Note: `engines.node` can be any semver range, with the latest satisfying version being resolved.

          [env: FNM_RESOLVE_ENGINES]

  -h, --help
          Print help (see a summary with '-h')
```

# `fnm unalias`

```
Remove an alias definition

Usage: fnm unalias [OPTIONS] <REQUESTED_ALIAS>

Arguments:
  <REQUESTED_ALIAS>


Options:
      --node-dist-mirror <NODE_DIST_MIRROR>
          <https://nodejs.org/dist/> mirror

          [env: FNM_NODE_DIST_MIRROR]
          [default: https://nodejs.org/dist]

      --fnm-dir <BASE_DIR>
          The root directory of fnm installations

          [env: FNM_DIR]

      --log-level <LOG_LEVEL>
          The log level of fnm commands

          [env: FNM_LOGLEVEL]
          [default: info]
          [possible values: quiet, error, info]

      --arch <ARCH>
          Override the architecture of the installed Node binary. Defaults to arch of fnm binary

          [env: FNM_ARCH]

      --version-file-strategy <VERSION_FILE_STRATEGY>
          A strategy for how to resolve the Node version. Used whenever `fnm use` or `fnm install` is called without a version, or when `--use-on-cd` is configured on evaluation

          [env: FNM_VERSION_FILE_STRATEGY]
          [default: local]

          Possible values:
          - local:     Use the local version of Node defined within the current directory
          - recursive: Use the version of Node defined within the current directory and all parent directories

      --corepack-enabled
          Enable corepack support for each new installation. This will make fnm call `corepack enable` on every Node.js installation. For more information about corepack see <https://nodejs.org/api/corepack.html>

          [env: FNM_COREPACK_ENABLED]

      --resolve-engines
          Resolve `engines.node` field in `package.json` whenever a `.node-version` or `.nvmrc` file is not present.
          Experimental: This feature is subject to change.
          Note: `engines.node` can be any semver range, with the latest satisfying version being resolved.

          [env: FNM_RESOLVE_ENGINES]

  -h, --help
          Print help (see a summary with '-h')
```

# `fnm default`

```
Set a version as the default version

This is a shorthand for `fnm alias VERSION default`

Usage: fnm default [OPTIONS] <VERSION>

Arguments:
  <VERSION>


Options:
      --node-dist-mirror <NODE_DIST_MIRROR>
          <https://nodejs.org/dist/> mirror

          [env: FNM_NODE_DIST_MIRROR]
          [default: https://nodejs.org/dist]

      --fnm-dir <BASE_DIR>
          The root directory of fnm installations

          [env: FNM_DIR]

      --log-level <LOG_LEVEL>
          The log level of fnm commands

          [env: FNM_LOGLEVEL]
          [default: info]
          [possible values: quiet, error, info]

      --arch <ARCH>
          Override the architecture of the installed Node binary. Defaults to arch of fnm binary

          [env: FNM_ARCH]

      --version-file-strategy <VERSION_FILE_STRATEGY>
          A strategy for how to resolve the Node version. Used whenever `fnm use` or `fnm install` is called without a version, or when `--use-on-cd` is configured on evaluation

          [env: FNM_VERSION_FILE_STRATEGY]
          [default: local]

          Possible values:
          - local:     Use the local version of Node defined within the current directory
          - recursive: Use the version of Node defined within the current directory and all parent directories

      --corepack-enabled
          Enable corepack support for each new installation. This will make fnm call `corepack enable` on every Node.js installation. For more information about corepack see <https://nodejs.org/api/corepack.html>

          [env: FNM_COREPACK_ENABLED]

      --resolve-engines
          Resolve `engines.node` field in `package.json` whenever a `.node-version` or `.nvmrc` file is not present.
          Experimental: This feature is subject to change.
          Note: `engines.node` can be any semver range, with the latest satisfying version being resolved.

          [env: FNM_RESOLVE_ENGINES]

  -h, --help
          Print help (see a summary with '-h')
```

# `fnm current`

```
Print the current Node.js version

Usage: fnm current [OPTIONS]

Options:
      --node-dist-mirror <NODE_DIST_MIRROR>
          <https://nodejs.org/dist/> mirror

          [env: FNM_NODE_DIST_MIRROR]
          [default: https://nodejs.org/dist]

      --fnm-dir <BASE_DIR>
          The root directory of fnm installations

          [env: FNM_DIR]

      --log-level <LOG_LEVEL>
          The log level of fnm commands

          [env: FNM_LOGLEVEL]
          [default: info]
          [possible values: quiet, error, info]

      --arch <ARCH>
          Override the architecture of the installed Node binary. Defaults to arch of fnm binary

          [env: FNM_ARCH]

      --version-file-strategy <VERSION_FILE_STRATEGY>
          A strategy for how to resolve the Node version. Used whenever `fnm use` or `fnm install` is called without a version, or when `--use-on-cd` is configured on evaluation

          [env: FNM_VERSION_FILE_STRATEGY]
          [default: local]

          Possible values:
          - local:     Use the local version of Node defined within the current directory
          - recursive: Use the version of Node defined within the current directory and all parent directories

      --corepack-enabled
          Enable corepack support for each new installation. This will make fnm call `corepack enable` on every Node.js installation. For more information about corepack see <https://nodejs.org/api/corepack.html>

          [env: FNM_COREPACK_ENABLED]

      --resolve-engines
          Resolve `engines.node` field in `package.json` whenever a `.node-version` or `.nvmrc` file is not present.
          Experimental: This feature is subject to change.
          Note: `engines.node` can be any semver range, with the latest satisfying version being resolved.

          [env: FNM_RESOLVE_ENGINES]

  -h, --help
          Print help (see a summary with '-h')
```

# `fnm exec`

```
Run a command within fnm context

Example:
--------
fnm exec --using=v12.0.0 node --version
=> v12.0.0

Usage: fnm exec [OPTIONS] [ARGUMENTS]...

Arguments:
  [ARGUMENTS]...
          The command to run

Options:
      --node-dist-mirror <NODE_DIST_MIRROR>
          <https://nodejs.org/dist/> mirror

          [env: FNM_NODE_DIST_MIRROR]
          [default: https://nodejs.org/dist]

      --using <VERSION>
          Either an explicit version, or a filename with the version written in it

      --fnm-dir <BASE_DIR>
          The root directory of fnm installations

          [env: FNM_DIR]

      --log-level <LOG_LEVEL>
          The log level of fnm commands

          [env: FNM_LOGLEVEL]
          [default: info]
          [possible values: quiet, error, info]

      --arch <ARCH>
          Override the architecture of the installed Node binary. Defaults to arch of fnm binary

          [env: FNM_ARCH]

      --version-file-strategy <VERSION_FILE_STRATEGY>
          A strategy for how to resolve the Node version. Used whenever `fnm use` or `fnm install` is called without a version, or when `--use-on-cd` is configured on evaluation

          [env: FNM_VERSION_FILE_STRATEGY]
          [default: local]

          Possible values:
          - local:     Use the local version of Node defined within the current directory
          - recursive: Use the version of Node defined within the current directory and all parent directories

      --corepack-enabled
          Enable corepack support for each new installation. This will make fnm call `corepack enable` on every Node.js installation. For more information about corepack see <https://nodejs.org/api/corepack.html>

          [env: FNM_COREPACK_ENABLED]

      --resolve-engines
          Resolve `engines.node` field in `package.json` whenever a `.node-version` or `.nvmrc` file is not present.
          Experimental: This feature is subject to change.
          Note: `engines.node` can be any semver range, with the latest satisfying version being resolved.

          [env: FNM_RESOLVE_ENGINES]

  -h, --help
          Print help (see a summary with '-h')
```

# `fnm uninstall`

```
Uninstall a Node.js version

> Warning: when providing an alias, it will remove the Node version the alias is pointing to, along with the other aliases that point to the same version.

Usage: fnm uninstall [OPTIONS] [VERSION]

Arguments:
  [VERSION]


Options:
      --node-dist-mirror <NODE_DIST_MIRROR>
          <https://nodejs.org/dist/> mirror

          [env: FNM_NODE_DIST_MIRROR]
          [default: https://nodejs.org/dist]

      --fnm-dir <BASE_DIR>
          The root directory of fnm installations

          [env: FNM_DIR]

      --log-level <LOG_LEVEL>
          The log level of fnm commands

          [env: FNM_LOGLEVEL]
          [default: info]
          [possible values: quiet, error, info]

      --arch <ARCH>
          Override the architecture of the installed Node binary. Defaults to arch of fnm binary

          [env: FNM_ARCH]

      --version-file-strategy <VERSION_FILE_STRATEGY>
          A strategy for how to resolve the Node version. Used whenever `fnm use` or `fnm install` is called without a version, or when `--use-on-cd` is configured on evaluation

          [env: FNM_VERSION_FILE_STRATEGY]
          [default: local]

          Possible values:
          - local:     Use the local version of Node defined within the current directory
          - recursive: Use the version of Node defined within the current directory and all parent directories

      --corepack-enabled
          Enable corepack support for each new installation. This will make fnm call `corepack enable` on every Node.js installation. For more information about corepack see <https://nodejs.org/api/corepack.html>

          [env: FNM_COREPACK_ENABLED]

      --resolve-engines
          Resolve `engines.node` field in `package.json` whenever a `.node-version` or `.nvmrc` file is not present.
          Experimental: This feature is subject to change.
          Note: `engines.node` can be any semver range, with the latest satisfying version being resolved.

          [env: FNM_RESOLVE_ENGINES]

  -h, --help
          Print help (see a summary with '-h')
```

# `fnm help`

```

```
