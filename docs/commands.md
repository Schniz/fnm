# `fnm`

```
fnm 1.28.0
A fast and simple Node.js manager

USAGE:
    fnm [OPTIONS] <SUBCOMMAND>

FLAGS:
    -h, --help
            Prints help information

    -V, --version
            Prints version information


OPTIONS:
        --arch <arch>
            Override the architecture of the installed Node binary. Defaults to arch of fnm binary [env: FNM_ARCH]
            [default: x64]
        --fnm-dir <base-dir>
            The root directory of fnm installations [env: FNM_DIR]

        --log-level <log-level>
            The log level of fnm commands [env: FNM_LOGLEVEL]  [default: info]  [possible values: quiet, info, all,
            error]
        --node-dist-mirror <node-dist-mirror>
            https://nodejs.org/dist/ mirror [env: FNM_NODE_DIST_MIRROR]  [default: https://nodejs.org/dist]


SUBCOMMANDS:
    alias          Alias a version to a common name
    completions    Print shell completions to stdout
    current        Print the current Node.js version
    default        Set a version as the default version
    env            Print and set up required environment variables for fnm
    exec           Run a command within fnm context
    help           Prints this message or the help of the given subcommand(s)
    install        Install a new Node.js version
    list           List all locally installed Node.js versions [aliases: ls]
    list-remote    List all remote Node.js versions [aliases: ls-remote]
    unalias        Remove an alias definition
    uninstall      Uninstall a Node.js version
    use            Change Node.js version
```

# `fnm alias`

```
fnm-alias 1.28.0
Alias a version to a common name

USAGE:
    fnm alias [OPTIONS] <to-version> <name>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --arch <arch>
            Override the architecture of the installed Node binary. Defaults to arch of fnm binary [env: FNM_ARCH]
            [default: x64]
        --fnm-dir <base-dir>                     The root directory of fnm installations [env: FNM_DIR]
        --log-level <log-level>
            The log level of fnm commands [env: FNM_LOGLEVEL]  [default: info]  [possible values: quiet, info, all,
            error]
        --node-dist-mirror <node-dist-mirror>
            https://nodejs.org/dist/ mirror [env: FNM_NODE_DIST_MIRROR]  [default: https://nodejs.org/dist]


ARGS:
    <to-version>
    <name>
```

# `fnm completions`

```
fnm-completions 1.28.0
Print shell completions to stdout

USAGE:
    fnm completions [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --arch <arch>
            Override the architecture of the installed Node binary. Defaults to arch of fnm binary [env: FNM_ARCH]
            [default: x64]
        --fnm-dir <base-dir>                     The root directory of fnm installations [env: FNM_DIR]
        --log-level <log-level>
            The log level of fnm commands [env: FNM_LOGLEVEL]  [default: info]  [possible values: quiet, info, all,
            error]
        --node-dist-mirror <node-dist-mirror>
            https://nodejs.org/dist/ mirror [env: FNM_NODE_DIST_MIRROR]  [default: https://nodejs.org/dist]

        --shell <shell>
            The shell syntax to use. Infers when missing [possible values: zsh, bash, fish, powershell, elvish]

```

# `fnm current`

```
fnm-current 1.28.0
Print the current Node.js version

USAGE:
    fnm current [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --arch <arch>
            Override the architecture of the installed Node binary. Defaults to arch of fnm binary [env: FNM_ARCH]
            [default: x64]
        --fnm-dir <base-dir>                     The root directory of fnm installations [env: FNM_DIR]
        --log-level <log-level>
            The log level of fnm commands [env: FNM_LOGLEVEL]  [default: info]  [possible values: quiet, info, all,
            error]
        --node-dist-mirror <node-dist-mirror>
            https://nodejs.org/dist/ mirror [env: FNM_NODE_DIST_MIRROR]  [default: https://nodejs.org/dist]

```

# `fnm default`

```
fnm-default 1.28.0
Set a version as the default version

This is a shorthand for `fnm alias VERSION default`

USAGE:
    fnm default [OPTIONS] <version>

FLAGS:
    -h, --help
            Prints help information

    -V, --version
            Prints version information


OPTIONS:
        --arch <arch>
            Override the architecture of the installed Node binary. Defaults to arch of fnm binary [env: FNM_ARCH]
            [default: x64]
        --fnm-dir <base-dir>
            The root directory of fnm installations [env: FNM_DIR]

        --log-level <log-level>
            The log level of fnm commands [env: FNM_LOGLEVEL]  [default: info]  [possible values: quiet, info, all,
            error]
        --node-dist-mirror <node-dist-mirror>
            https://nodejs.org/dist/ mirror [env: FNM_NODE_DIST_MIRROR]  [default: https://nodejs.org/dist]


ARGS:
    <version>


```

# `fnm env`

```
fnm-env 1.28.0
Print and set up required environment variables for fnm

This command generates a series of shell commands that should be evaluated by your shell to create a fnm-ready
environment.

Each shell has its own syntax of evaluating a dynamic expression. For example, evaluating fnm on Bash and Zsh would look
like `eval "$(fnm env)"`. In Fish, evaluating would look like `fnm env | source`

USAGE:
    fnm env [FLAGS] [OPTIONS]

FLAGS:
    -h, --help
            Prints help information

        --use-on-cd
            Print the script to change Node versions every directory change

    -V, --version
            Prints version information


OPTIONS:
        --arch <arch>
            Override the architecture of the installed Node binary. Defaults to arch of fnm binary [env: FNM_ARCH]
            [default: x64]
        --fnm-dir <base-dir>
            The root directory of fnm installations [env: FNM_DIR]

        --log-level <log-level>
            The log level of fnm commands [env: FNM_LOGLEVEL]  [default: info]  [possible values: quiet, info, all,
            error]
        --node-dist-mirror <node-dist-mirror>
            https://nodejs.org/dist/ mirror [env: FNM_NODE_DIST_MIRROR]  [default: https://nodejs.org/dist]

        --shell <shell>
            The shell syntax to use. Infers when missing [possible values: bash, zsh, fish, powershell]

```

# `fnm exec`

```
fnm-exec 1.28.0
Run a command within fnm context

Example:
--------
fnm exec --using=v12.0.0 node --version
=> v12.0.0

USAGE:
    fnm exec [OPTIONS] [arguments]...

FLAGS:
    -h, --help
            Prints help information

    -V, --version
            Prints version information


OPTIONS:
        --arch <arch>
            Override the architecture of the installed Node binary. Defaults to arch of fnm binary [env: FNM_ARCH]
            [default: x64]
        --fnm-dir <base-dir>
            The root directory of fnm installations [env: FNM_DIR]

        --log-level <log-level>
            The log level of fnm commands [env: FNM_LOGLEVEL]  [default: info]  [possible values: quiet, info, all,
            error]
        --node-dist-mirror <node-dist-mirror>
            https://nodejs.org/dist/ mirror [env: FNM_NODE_DIST_MIRROR]  [default: https://nodejs.org/dist]

        --using <version>
            Either an explicit version, or a filename with the version written in it


ARGS:
    <arguments>...
            The command to run

```

# `fnm help`

```

```

# `fnm install`

```
fnm-install 1.28.0
Install a new Node.js version

USAGE:
    fnm install [FLAGS] [OPTIONS] [version]

FLAGS:
    -h, --help       Prints help information
        --lts        Install latest LTS
    -V, --version    Prints version information

OPTIONS:
        --arch <arch>
            Override the architecture of the installed Node binary. Defaults to arch of fnm binary [env: FNM_ARCH]
            [default: x64]
        --fnm-dir <base-dir>                     The root directory of fnm installations [env: FNM_DIR]
        --log-level <log-level>
            The log level of fnm commands [env: FNM_LOGLEVEL]  [default: info]  [possible values: quiet, info, all,
            error]
        --node-dist-mirror <node-dist-mirror>
            https://nodejs.org/dist/ mirror [env: FNM_NODE_DIST_MIRROR]  [default: https://nodejs.org/dist]


ARGS:
    <version>    A version string. Can be a partial semver or a LTS version name by the format lts/NAME
```

# `fnm list`

```
fnm-list 1.28.0
List all locally installed Node.js versions

USAGE:
    fnm list [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --arch <arch>
            Override the architecture of the installed Node binary. Defaults to arch of fnm binary [env: FNM_ARCH]
            [default: x64]
        --fnm-dir <base-dir>                     The root directory of fnm installations [env: FNM_DIR]
        --log-level <log-level>
            The log level of fnm commands [env: FNM_LOGLEVEL]  [default: info]  [possible values: quiet, info, all,
            error]
        --node-dist-mirror <node-dist-mirror>
            https://nodejs.org/dist/ mirror [env: FNM_NODE_DIST_MIRROR]  [default: https://nodejs.org/dist]

```

# `fnm list-remote`

```
fnm-list-remote 1.28.0
List all remote Node.js versions

USAGE:
    fnm list-remote [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --arch <arch>
            Override the architecture of the installed Node binary. Defaults to arch of fnm binary [env: FNM_ARCH]
            [default: x64]
        --fnm-dir <base-dir>                     The root directory of fnm installations [env: FNM_DIR]
        --log-level <log-level>
            The log level of fnm commands [env: FNM_LOGLEVEL]  [default: info]  [possible values: quiet, info, all,
            error]
        --node-dist-mirror <node-dist-mirror>
            https://nodejs.org/dist/ mirror [env: FNM_NODE_DIST_MIRROR]  [default: https://nodejs.org/dist]

```

# `fnm unalias`

```
fnm-unalias 1.28.0
Remove an alias definition

USAGE:
    fnm unalias [OPTIONS] <requested-alias>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --arch <arch>
            Override the architecture of the installed Node binary. Defaults to arch of fnm binary [env: FNM_ARCH]
            [default: x64]
        --fnm-dir <base-dir>                     The root directory of fnm installations [env: FNM_DIR]
        --log-level <log-level>
            The log level of fnm commands [env: FNM_LOGLEVEL]  [default: info]  [possible values: quiet, info, all,
            error]
        --node-dist-mirror <node-dist-mirror>
            https://nodejs.org/dist/ mirror [env: FNM_NODE_DIST_MIRROR]  [default: https://nodejs.org/dist]


ARGS:
    <requested-alias>
```

# `fnm uninstall`

```
fnm-uninstall 1.28.0
Uninstall a Node.js version

> Warning: when providing an alias, it will remove the Node version the alias is pointing to, along with the other
aliases that point to the same version.

USAGE:
    fnm uninstall [OPTIONS] [version]

FLAGS:
    -h, --help
            Prints help information

    -V, --version
            Prints version information


OPTIONS:
        --arch <arch>
            Override the architecture of the installed Node binary. Defaults to arch of fnm binary [env: FNM_ARCH]
            [default: x64]
        --fnm-dir <base-dir>
            The root directory of fnm installations [env: FNM_DIR]

        --log-level <log-level>
            The log level of fnm commands [env: FNM_LOGLEVEL]  [default: info]  [possible values: quiet, info, all,
            error]
        --node-dist-mirror <node-dist-mirror>
            https://nodejs.org/dist/ mirror [env: FNM_NODE_DIST_MIRROR]  [default: https://nodejs.org/dist]


ARGS:
    <version>


```

# `fnm use`

```
fnm-use 1.28.0
Change Node.js version

USAGE:
    fnm use [FLAGS] [OPTIONS] [version]

FLAGS:
    -h, --help                  Prints help information
        --install-if-missing    Install the version if it isn't installed yet
    -V, --version               Prints version information

OPTIONS:
        --arch <arch>
            Override the architecture of the installed Node binary. Defaults to arch of fnm binary [env: FNM_ARCH]
            [default: x64]
        --fnm-dir <base-dir>                     The root directory of fnm installations [env: FNM_DIR]
        --log-level <log-level>
            The log level of fnm commands [env: FNM_LOGLEVEL]  [default: info]  [possible values: quiet, info, all,
            error]
        --node-dist-mirror <node-dist-mirror>
            https://nodejs.org/dist/ mirror [env: FNM_NODE_DIST_MIRROR]  [default: https://nodejs.org/dist]


ARGS:
    <version>
```
