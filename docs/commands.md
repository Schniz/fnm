# `fnm`

```
fnm 1.31.1
A fast and simple Node.js manager

USAGE:
    fnm [OPTIONS] <SUBCOMMAND>

OPTIONS:
        --arch <ARCH>
            Override the architecture of the installed Node binary. Defaults to arch of fnm binary

            [env: FNM_ARCH]

        --fnm-dir <BASE_DIR>
            The root directory of fnm installations

            [env: FNM_DIR]

    -h, --help
            Print help information

        --log-level <LOG_LEVEL>
            The log level of fnm commands

            [env: FNM_LOGLEVEL]
            [default: info]
            [possible values: quiet, info, all, error]

        --node-dist-mirror <NODE_DIST_MIRROR>
            https://nodejs.org/dist/ mirror

            [env: FNM_NODE_DIST_MIRROR]
            [default: https://nodejs.org/dist]

    -V, --version
            Print version information

        --version-file-strategy <VERSION_FILE_STRATEGY>
            A strategy for how to resolve the Node version. Used whenever `fnm use` or `fnm install`
            is called without a version, or when `--use-on-cd` is configured on evaluation.

            * `local`: Use the local version of Node defined within the current directory

            * `recursive`: Use the version of Node defined within the current directory and all
            parent directories

            [env: FNM_VERSION_FILE_STRATEGY]
            [default: local]
            [possible values: local, recursive]

SUBCOMMANDS:
    alias
            Alias a version to a common name
    completions
            Print shell completions to stdout
    current
            Print the current Node.js version
    default
            Set a version as the default version
    env
            Print and set up required environment variables for fnm
    exec
            Run a command within fnm context
    help
            Print this message or the help of the given subcommand(s)
    install
            Install a new Node.js version
    list
            List all locally installed Node.js versions [aliases: ls]
    list-remote
            List all remote Node.js versions [aliases: ls-remote]
    unalias
            Remove an alias definition
    uninstall
            Uninstall a Node.js version
    use
            Change Node.js version
```

# `fnm alias`

```
fnm-alias
Alias a version to a common name

USAGE:
    fnm alias [OPTIONS] <TO_VERSION> <NAME>

ARGS:
    <TO_VERSION>


    <NAME>


OPTIONS:
        --arch <ARCH>
            Override the architecture of the installed Node binary. Defaults to arch of fnm binary

            [env: FNM_ARCH]

        --fnm-dir <BASE_DIR>
            The root directory of fnm installations

            [env: FNM_DIR]

    -h, --help
            Print help information

        --log-level <LOG_LEVEL>
            The log level of fnm commands

            [env: FNM_LOGLEVEL]
            [default: info]
            [possible values: quiet, info, all, error]

        --node-dist-mirror <NODE_DIST_MIRROR>
            https://nodejs.org/dist/ mirror

            [env: FNM_NODE_DIST_MIRROR]
            [default: https://nodejs.org/dist]

        --version-file-strategy <VERSION_FILE_STRATEGY>
            A strategy for how to resolve the Node version. Used whenever `fnm use` or `fnm install`
            is called without a version, or when `--use-on-cd` is configured on evaluation.

            * `local`: Use the local version of Node defined within the current directory

            * `recursive`: Use the version of Node defined within the current directory and all
            parent directories

            [env: FNM_VERSION_FILE_STRATEGY]
            [default: local]
            [possible values: local, recursive]
```

# `fnm completions`

```
fnm-completions
Print shell completions to stdout

USAGE:
    fnm completions [OPTIONS]

OPTIONS:
        --arch <ARCH>
            Override the architecture of the installed Node binary. Defaults to arch of fnm binary

            [env: FNM_ARCH]

        --fnm-dir <BASE_DIR>
            The root directory of fnm installations

            [env: FNM_DIR]

    -h, --help
            Print help information

        --log-level <LOG_LEVEL>
            The log level of fnm commands

            [env: FNM_LOGLEVEL]
            [default: info]
            [possible values: quiet, info, all, error]

        --node-dist-mirror <NODE_DIST_MIRROR>
            https://nodejs.org/dist/ mirror

            [env: FNM_NODE_DIST_MIRROR]
            [default: https://nodejs.org/dist]

        --shell <SHELL>
            The shell syntax to use. Infers when missing

        --version-file-strategy <VERSION_FILE_STRATEGY>
            A strategy for how to resolve the Node version. Used whenever `fnm use` or `fnm install`
            is called without a version, or when `--use-on-cd` is configured on evaluation.

            * `local`: Use the local version of Node defined within the current directory

            * `recursive`: Use the version of Node defined within the current directory and all
            parent directories

            [env: FNM_VERSION_FILE_STRATEGY]
            [default: local]
            [possible values: local, recursive]
```

# `fnm current`

```
fnm-current
Print the current Node.js version

USAGE:
    fnm current [OPTIONS]

OPTIONS:
        --arch <ARCH>
            Override the architecture of the installed Node binary. Defaults to arch of fnm binary

            [env: FNM_ARCH]

        --fnm-dir <BASE_DIR>
            The root directory of fnm installations

            [env: FNM_DIR]

    -h, --help
            Print help information

        --log-level <LOG_LEVEL>
            The log level of fnm commands

            [env: FNM_LOGLEVEL]
            [default: info]
            [possible values: quiet, info, all, error]

        --node-dist-mirror <NODE_DIST_MIRROR>
            https://nodejs.org/dist/ mirror

            [env: FNM_NODE_DIST_MIRROR]
            [default: https://nodejs.org/dist]

        --version-file-strategy <VERSION_FILE_STRATEGY>
            A strategy for how to resolve the Node version. Used whenever `fnm use` or `fnm install`
            is called without a version, or when `--use-on-cd` is configured on evaluation.

            * `local`: Use the local version of Node defined within the current directory

            * `recursive`: Use the version of Node defined within the current directory and all
            parent directories

            [env: FNM_VERSION_FILE_STRATEGY]
            [default: local]
            [possible values: local, recursive]
```

# `fnm default`

```
fnm-default
Set a version as the default version

This is a shorthand for `fnm alias VERSION default`

USAGE:
    fnm default [OPTIONS] <VERSION>

ARGS:
    <VERSION>


OPTIONS:
        --arch <ARCH>
            Override the architecture of the installed Node binary. Defaults to arch of fnm binary

            [env: FNM_ARCH]

        --fnm-dir <BASE_DIR>
            The root directory of fnm installations

            [env: FNM_DIR]

    -h, --help
            Print help information

        --log-level <LOG_LEVEL>
            The log level of fnm commands

            [env: FNM_LOGLEVEL]
            [default: info]
            [possible values: quiet, info, all, error]

        --node-dist-mirror <NODE_DIST_MIRROR>
            https://nodejs.org/dist/ mirror

            [env: FNM_NODE_DIST_MIRROR]
            [default: https://nodejs.org/dist]

        --version-file-strategy <VERSION_FILE_STRATEGY>
            A strategy for how to resolve the Node version. Used whenever `fnm use` or `fnm install`
            is called without a version, or when `--use-on-cd` is configured on evaluation.

            * `local`: Use the local version of Node defined within the current directory

            * `recursive`: Use the version of Node defined within the current directory and all
            parent directories

            [env: FNM_VERSION_FILE_STRATEGY]
            [default: local]
            [possible values: local, recursive]
```

# `fnm env`

```
fnm-env
Print and set up required environment variables for fnm

This command generates a series of shell commands that should be evaluated by your shell to create a
fnm-ready environment.

Each shell has its own syntax of evaluating a dynamic expression. For example, evaluating fnm on
Bash and Zsh would look like `eval "$(fnm env)"`. In Fish, evaluating would look like `fnm env |
source`

USAGE:
    fnm env [OPTIONS]

OPTIONS:
        --arch <ARCH>
            Override the architecture of the installed Node binary. Defaults to arch of fnm binary

            [env: FNM_ARCH]

        --fnm-dir <BASE_DIR>
            The root directory of fnm installations

            [env: FNM_DIR]

    -h, --help
            Print help information

        --log-level <LOG_LEVEL>
            The log level of fnm commands

            [env: FNM_LOGLEVEL]
            [default: info]
            [possible values: quiet, info, all, error]

        --node-dist-mirror <NODE_DIST_MIRROR>
            https://nodejs.org/dist/ mirror

            [env: FNM_NODE_DIST_MIRROR]
            [default: https://nodejs.org/dist]

        --shell <SHELL>
            The shell syntax to use. Infers when missing

            [possible values: bash, zsh, fish, powershell]

        --use-on-cd
            Print the script to change Node versions every directory change

        --version-file-strategy <VERSION_FILE_STRATEGY>
            A strategy for how to resolve the Node version. Used whenever `fnm use` or `fnm install`
            is called without a version, or when `--use-on-cd` is configured on evaluation.

            * `local`: Use the local version of Node defined within the current directory

            * `recursive`: Use the version of Node defined within the current directory and all
            parent directories

            [env: FNM_VERSION_FILE_STRATEGY]
            [default: local]
            [possible values: local, recursive]
```

# `fnm exec`

```
fnm-exec
Run a command within fnm context

Example:
--------
fnm exec --using=v12.0.0 node --version
=> v12.0.0

USAGE:
    fnm exec [OPTIONS] [ARGUMENTS]...

ARGS:
    <ARGUMENTS>...
            The command to run

OPTIONS:
        --arch <ARCH>
            Override the architecture of the installed Node binary. Defaults to arch of fnm binary

            [env: FNM_ARCH]

        --fnm-dir <BASE_DIR>
            The root directory of fnm installations

            [env: FNM_DIR]

    -h, --help
            Print help information

        --log-level <LOG_LEVEL>
            The log level of fnm commands

            [env: FNM_LOGLEVEL]
            [default: info]
            [possible values: quiet, info, all, error]

        --node-dist-mirror <NODE_DIST_MIRROR>
            https://nodejs.org/dist/ mirror

            [env: FNM_NODE_DIST_MIRROR]
            [default: https://nodejs.org/dist]

        --using <VERSION>
            Either an explicit version, or a filename with the version written in it

        --version-file-strategy <VERSION_FILE_STRATEGY>
            A strategy for how to resolve the Node version. Used whenever `fnm use` or `fnm install`
            is called without a version, or when `--use-on-cd` is configured on evaluation.

            * `local`: Use the local version of Node defined within the current directory

            * `recursive`: Use the version of Node defined within the current directory and all
            parent directories

            [env: FNM_VERSION_FILE_STRATEGY]
            [default: local]
            [possible values: local, recursive]
```

# `fnm help`

```

```

# `fnm install`

```
fnm-install
Install a new Node.js version

USAGE:
    fnm install [OPTIONS] [VERSION]

ARGS:
    <VERSION>
            A version string. Can be a partial semver or a LTS version name by the format lts/NAME

OPTIONS:
        --arch <ARCH>
            Override the architecture of the installed Node binary. Defaults to arch of fnm binary

            [env: FNM_ARCH]

        --fnm-dir <BASE_DIR>
            The root directory of fnm installations

            [env: FNM_DIR]

    -h, --help
            Print help information

        --log-level <LOG_LEVEL>
            The log level of fnm commands

            [env: FNM_LOGLEVEL]
            [default: info]
            [possible values: quiet, info, all, error]

        --lts
            Install latest LTS

        --node-dist-mirror <NODE_DIST_MIRROR>
            https://nodejs.org/dist/ mirror

            [env: FNM_NODE_DIST_MIRROR]
            [default: https://nodejs.org/dist]

        --version-file-strategy <VERSION_FILE_STRATEGY>
            A strategy for how to resolve the Node version. Used whenever `fnm use` or `fnm install`
            is called without a version, or when `--use-on-cd` is configured on evaluation.

            * `local`: Use the local version of Node defined within the current directory

            * `recursive`: Use the version of Node defined within the current directory and all
            parent directories

            [env: FNM_VERSION_FILE_STRATEGY]
            [default: local]
            [possible values: local, recursive]
```

# `fnm list`

```
fnm-list
List all locally installed Node.js versions

USAGE:
    fnm list [OPTIONS]

OPTIONS:
        --arch <ARCH>
            Override the architecture of the installed Node binary. Defaults to arch of fnm binary

            [env: FNM_ARCH]

        --fnm-dir <BASE_DIR>
            The root directory of fnm installations

            [env: FNM_DIR]

    -h, --help
            Print help information

        --log-level <LOG_LEVEL>
            The log level of fnm commands

            [env: FNM_LOGLEVEL]
            [default: info]
            [possible values: quiet, info, all, error]

        --node-dist-mirror <NODE_DIST_MIRROR>
            https://nodejs.org/dist/ mirror

            [env: FNM_NODE_DIST_MIRROR]
            [default: https://nodejs.org/dist]

        --version-file-strategy <VERSION_FILE_STRATEGY>
            A strategy for how to resolve the Node version. Used whenever `fnm use` or `fnm install`
            is called without a version, or when `--use-on-cd` is configured on evaluation.

            * `local`: Use the local version of Node defined within the current directory

            * `recursive`: Use the version of Node defined within the current directory and all
            parent directories

            [env: FNM_VERSION_FILE_STRATEGY]
            [default: local]
            [possible values: local, recursive]
```

# `fnm list-remote`

```
fnm-list-remote
List all remote Node.js versions

USAGE:
    fnm list-remote [OPTIONS]

OPTIONS:
        --arch <ARCH>
            Override the architecture of the installed Node binary. Defaults to arch of fnm binary

            [env: FNM_ARCH]

        --fnm-dir <BASE_DIR>
            The root directory of fnm installations

            [env: FNM_DIR]

    -h, --help
            Print help information

        --log-level <LOG_LEVEL>
            The log level of fnm commands

            [env: FNM_LOGLEVEL]
            [default: info]
            [possible values: quiet, info, all, error]

        --node-dist-mirror <NODE_DIST_MIRROR>
            https://nodejs.org/dist/ mirror

            [env: FNM_NODE_DIST_MIRROR]
            [default: https://nodejs.org/dist]

        --version-file-strategy <VERSION_FILE_STRATEGY>
            A strategy for how to resolve the Node version. Used whenever `fnm use` or `fnm install`
            is called without a version, or when `--use-on-cd` is configured on evaluation.

            * `local`: Use the local version of Node defined within the current directory

            * `recursive`: Use the version of Node defined within the current directory and all
            parent directories

            [env: FNM_VERSION_FILE_STRATEGY]
            [default: local]
            [possible values: local, recursive]
```

# `fnm unalias`

```
fnm-unalias
Remove an alias definition

USAGE:
    fnm unalias [OPTIONS] <REQUESTED_ALIAS>

ARGS:
    <REQUESTED_ALIAS>


OPTIONS:
        --arch <ARCH>
            Override the architecture of the installed Node binary. Defaults to arch of fnm binary

            [env: FNM_ARCH]

        --fnm-dir <BASE_DIR>
            The root directory of fnm installations

            [env: FNM_DIR]

    -h, --help
            Print help information

        --log-level <LOG_LEVEL>
            The log level of fnm commands

            [env: FNM_LOGLEVEL]
            [default: info]
            [possible values: quiet, info, all, error]

        --node-dist-mirror <NODE_DIST_MIRROR>
            https://nodejs.org/dist/ mirror

            [env: FNM_NODE_DIST_MIRROR]
            [default: https://nodejs.org/dist]

        --version-file-strategy <VERSION_FILE_STRATEGY>
            A strategy for how to resolve the Node version. Used whenever `fnm use` or `fnm install`
            is called without a version, or when `--use-on-cd` is configured on evaluation.

            * `local`: Use the local version of Node defined within the current directory

            * `recursive`: Use the version of Node defined within the current directory and all
            parent directories

            [env: FNM_VERSION_FILE_STRATEGY]
            [default: local]
            [possible values: local, recursive]
```

# `fnm uninstall`

```
fnm-uninstall
Uninstall a Node.js version

> Warning: when providing an alias, it will remove the Node version the alias is pointing to, along
with the other aliases that point to the same version.

USAGE:
    fnm uninstall [OPTIONS] [VERSION]

ARGS:
    <VERSION>


OPTIONS:
        --arch <ARCH>
            Override the architecture of the installed Node binary. Defaults to arch of fnm binary

            [env: FNM_ARCH]

        --fnm-dir <BASE_DIR>
            The root directory of fnm installations

            [env: FNM_DIR]

    -h, --help
            Print help information

        --log-level <LOG_LEVEL>
            The log level of fnm commands

            [env: FNM_LOGLEVEL]
            [default: info]
            [possible values: quiet, info, all, error]

        --node-dist-mirror <NODE_DIST_MIRROR>
            https://nodejs.org/dist/ mirror

            [env: FNM_NODE_DIST_MIRROR]
            [default: https://nodejs.org/dist]

        --version-file-strategy <VERSION_FILE_STRATEGY>
            A strategy for how to resolve the Node version. Used whenever `fnm use` or `fnm install`
            is called without a version, or when `--use-on-cd` is configured on evaluation.

            * `local`: Use the local version of Node defined within the current directory

            * `recursive`: Use the version of Node defined within the current directory and all
            parent directories

            [env: FNM_VERSION_FILE_STRATEGY]
            [default: local]
            [possible values: local, recursive]
```

# `fnm use`

```
fnm-use
Change Node.js version

USAGE:
    fnm use [OPTIONS] [VERSION]

ARGS:
    <VERSION>


OPTIONS:
        --arch <ARCH>
            Override the architecture of the installed Node binary. Defaults to arch of fnm binary

            [env: FNM_ARCH]

        --fnm-dir <BASE_DIR>
            The root directory of fnm installations

            [env: FNM_DIR]

    -h, --help
            Print help information

        --install-if-missing
            Install the version if it isn't installed yet

        --log-level <LOG_LEVEL>
            The log level of fnm commands

            [env: FNM_LOGLEVEL]
            [default: info]
            [possible values: quiet, info, all, error]

        --node-dist-mirror <NODE_DIST_MIRROR>
            https://nodejs.org/dist/ mirror

            [env: FNM_NODE_DIST_MIRROR]
            [default: https://nodejs.org/dist]

        --silent-if-unchanged
            Don't output a message identifying the version being used if it will not change due to
            execution of this command

        --version-file-strategy <VERSION_FILE_STRATEGY>
            A strategy for how to resolve the Node version. Used whenever `fnm use` or `fnm install`
            is called without a version, or when `--use-on-cd` is configured on evaluation.

            * `local`: Use the local version of Node defined within the current directory

            * `recursive`: Use the version of Node defined within the current directory and all
            parent directories

            [env: FNM_VERSION_FILE_STRATEGY]
            [default: local]
            [possible values: local, recursive]
```
