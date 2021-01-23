<h1 align="center">
  Fast Node Manager (<code>fnm</code>)
  <img alt="Amount of downloads" src="https://img.shields.io/github/downloads/Schniz/fnm/total.svg?style=flat" />
  <a href="https://github.com/Schniz/fnm/actions"><img src="https://img.shields.io/github/workflow/status/Schniz/fnm/Rust/master?label=workflow" alt="GitHub Actions workflow status" /></a>
</h1>

> :rocket: Fast and simple Node.js version manager, built in Rust

<div align="center">
  <img src="./docs/fnm.svg" alt="Blazing fast!">
</div>

## Features

:earth_americas: Cross-platform support (macOS, Windows, Linux)

:sparkles: Single file, easy installation, instant startup

:rocket: Built with speed in mind

:thinking: Works with `.node-version` and `.nvmrc` files

## Installation

### Using a script (macOS/Linux)

For `bash`, `zsh` and `fish` shells, there's an [automatic installation script](./.ci/install.sh):

```bash
curl -fsSL https://fnm.vercel.app/install | bash
```

#### Upgrade

On macOS, it is as simple as `brew upgrade fnm`.

On other operating systems, upgrading `fnm` is almost the same as installing it. To prevent duplication in your shell config file add `--skip-shell` to install command.

#### Parameters

`--install-dir`

Set a custom directory for fnm to be installed. The default is `$HOME/.fnm`.

`--skip-shell`

Skip appending shell specific loader to shell config file, based on the current user shell, defined in `$SHELL`. e.g. for Bash, `$HOME/.bashrc`. `$HOME/.zshrc` for Zsh. For Fish - `$HOME/.config/fish/conf.d/fnm.fish`

`--force-install`

macOS installations using the installation script are deprecated in favor of the Homebrew formula, but this forces the script to install using it anyway.

Example:

```bash
curl -fsSL https://fnm.vercel.app/install | bash -s -- --install-dir "./.fnm" --skip-shell
```

### Manually

#### Using Homebrew (macOS)

```bash
brew install fnm
```

Then, [set up your shell for fnm](#shell-setup)

#### Using Scoop (Windows)

```bash
scoop install fnm
```

Then, [set up your shell for fnm](#shell-setup)

#### Using Cargo (Linux/macOS/Windows)

```bash
cargo install fnm
```

Then, [set up your shell for fnm](#shell-setup)

#### Using a release binary (Linux/macOS/Windows)

- Download the [latest release binary](https://github.com/Schniz/fnm/releases) for your system
- Make it available globally on `PATH` environment variable
- Configure your shell profile:

## Completions

fnm ships its completions with the binary:

```
fnm completions --shell <SHELL>
```

Where `<SHELL>` can be one of the supported shells:

- `bash`
- `zsh`
- `fish`
- `powershell`

Please follow your shell instructions to install them.

### Shell Setup

fnm needs to run some shell commands before you can start using it.
This is done by evaluating the output of `fnm env`. Check out the following guides for the shell you use:

#### Bash

add the following to your `.bashrc` profile:

```bash
eval "$(fnm env)"
```

#### Zsh

add the following to your `.zshrc` profile:

```zsh
eval "$(fnm env)"
```

#### Fish shell

create `~/.config/fish/conf.d/fnm.fish` add this line to it:

```fish
fnm env | source
```

#### PowerShell

add the following to the end of your profile file::

```powershell
fnm env --use-on-cd | Out-String | Invoke-Expression
```

- On Windows, the profile is located at `~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1`
- For macOS/Linux, the profile is located at `~/.config/powershell/Microsoft.PowerShell_profile.ps1`

#### Windows Command Prompt aka Batch aka WinCMD

fnm is also supported but is not entirely covered. [You can set up a startup script](https://superuser.com/a/144348) and append the following line:

```
FOR /f "tokens=*" %i IN ('fnm env --use-on-cd') DO CALL %i
```

## Usage

### Global Options

```
fnm [--shell=fish|bash|zsh] [--node-dist-mirror=URI] [--fnm-dir=DIR] [--log-level=quiet|error|info] <command>
```

- Providing `--shell=fish` will output the Fish-compliant version. Omitting it and `fnm` will try to infer the current shell based on the process tree
- Providing `--node-dist-mirror="https://npm.taobao.org/dist"` will use the Chinese mirror of Node.js
- Providing `--fnm-dir="/tmp/fnm"` will install and use versions in `/tmp/fnm` directory

You can always use `fnm --help` to read the docs:

### `fnm install [VERSION]`

Installs `[VERSION]`. If no version provided, it will install the version specified in the `.node-version` or `.nvmrc` files located in the current working directory.

### `fnm install --lts`

Installs the latest LTS version of Node

### `fnm use [VERSION]`

Activates `[VERSION]` as the current Node version. If no version provided, it will activate the version specified in the `.node-version` or `.nvmrc` file located in the current working directory.

#### Flags

- `--install-if-missing` — installs the version if it isn't installed yet

### `fnm current`

Display currently activated Node version.

### `fnm list`

Lists the installed Node versions.

### `fnm list-remote`

Lists the Node versions available to download remotely.

### `fnm uninstall [VERSION]`

Uninstalls the node version specified in `[VERSION]`.

### `fnm alias [VERSION] [NAME]`

Aliases a Node version to a given name.

### `fnm default [VERSION]`

Aliases a Node version as default. Uses `fnm alias` underneath.

### `fnm env`

Prints the required shell commands in order to configure your shell, Bash compliant if can't infer the shell. This command is highly influenced by [the global options](#global-options)

#### Options:

- `--use-on-cd` will also output a script that will automatically change the node version if a `.node-version`/`.nvmrc` file is found

## Contributing

PRs welcome :tada:

### Developing:

```
# Install Rust
git clone https://github.com/Schniz/fnm.git
cd fnm/
cargo build
```

### Running Binary:

```
cargo run -- --help # Will behave like `fnm --help`
```

### Running Tests:

```
cargo test
```
