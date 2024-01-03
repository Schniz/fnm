<h1 align="center">
  Fast Node Manager (<code>fnm</code>)
  <img alt="Amount of downloads" src="https://img.shields.io/github/downloads/Schniz/fnm/total.svg?style=flat" />
  <a href="https://github.com/Schniz/fnm/actions"><img src="https://img.shields.io/github/actions/workflow/status/Schniz/fnm/rust.yml?branch=master&label=workflow" alt="GitHub Actions workflow status" /></a>
</h1>

> :rocket: Fast and simple Node.js version manager, built in Rust

<div align="center">
  <img src="./docs/fnm.svg" alt="Blazing fast!">
</div>

## Features

:earth_americas: Cross-platform support (macOS, Windows, Linux)

:sparkles: Single file, easy installation, instant startup

:rocket: Built with speed in mind

:open_file_folder: Works with `.node-version` and `.nvmrc` files

## Installation

### Using a script (macOS/Linux)

For `bash`, `zsh` and `fish` shells, there's an [automatic installation script](./.ci/install.sh).

First ensure that `curl` and `unzip` are already installed on you operating system. Then execute:

```sh
curl -fsSL https://fnm.vercel.app/install | bash
```

#### Upgrade

On macOS, it is as simple as `brew upgrade fnm`.

On other operating systems, upgrading `fnm` is almost the same as installing it. To prevent duplication in your shell config file add `--skip-shell` to install command.

#### Parameters

`--install-dir`

Set a custom directory for fnm to be installed. The default is `$XDG_DATA_HOME/fnm` (if `$XDG_DATA_HOME` is not defined it falls back to `$HOME/.local/share/fnm` on linux and `$HOME/Library/Application Support/fnm` on MacOS).

`--skip-shell`

Skip appending shell specific loader to shell config file, based on the current user shell, defined in `$SHELL`. e.g. for Bash, `$HOME/.bashrc`. `$HOME/.zshrc` for Zsh. For Fish - `$HOME/.config/fish/conf.d/fnm.fish`

`--force-install`

macOS installations using the installation script are deprecated in favor of the Homebrew formula, but this forces the script to install using it anyway.

Example:

```sh
curl -fsSL https://fnm.vercel.app/install | bash -s -- --install-dir "./.fnm" --skip-shell
```

### Manually

#### Using Homebrew (macOS/Linux)

```sh
brew install fnm
```

Then, [set up your shell for fnm](#shell-setup)

#### Using Winget (Windows)

```sh
winget install Schniz.fnm
```

#### Using Scoop (Windows)

```sh
scoop install fnm
```

Then, [set up your shell for fnm](#shell-setup)

#### Using Chocolatey (Windows)

```sh
choco install fnm
```

Then, [set up your shell for fnm](#shell-setup)

#### Using Cargo (Linux/macOS/Windows)

```sh
cargo install fnm
```

Then, [set up your shell for fnm](#shell-setup)

#### Using a release binary (Linux/macOS/Windows)

- Download the [latest release binary](https://github.com/Schniz/fnm/releases) for your system
- Make it available globally on `PATH` environment variable
- [Set up your shell for fnm](#shell-setup)

### Removing

To remove fnm (😢), just delete the `.fnm` folder in your home directory. You should also edit your shell configuration to remove any references to fnm (ie. read [Shell Setup](#shell-setup), and do the opposite).

## Completions

fnm ships its completions with the binary:

```sh
fnm completions --shell <SHELL>
```

Where `<SHELL>` can be one of the supported shells:

- `bash`
- `zsh`
- `fish`
- `powershell`

Please follow your shell instructions to install them.

### Shell Setup

Environment variables need to be setup before you can start using fnm.
This is done by evaluating the output of `fnm env`.
To automatically run `fnm use` when a directory contains a `.node-version` or `.nvmrc` file, add the `--use-on-cd` option to your shell setup.

Adding a `.node-version` to your project is as simple as:

```bash
$ node --version
v14.18.3
$ node --version > .node-version
```

Check out the following guides for the shell you use:

#### Bash

Add the following to your `.bashrc` profile:

```bash
eval "$(fnm env --use-on-cd)"
```

#### Zsh

Add the following to your `.zshrc` profile:

```zsh
eval "$(fnm env --use-on-cd)"
```

#### Fish shell

Create `~/.config/fish/conf.d/fnm.fish` add this line to it:

```fish
fnm env --use-on-cd | source
```

#### PowerShell

Add the following to the end of your profile file:

```powershell
fnm env --use-on-cd | Out-String | Invoke-Expression
```

- For macOS/Linux, the profile is located at `~/.config/powershell/Microsoft.PowerShell_profile.ps1`
- On Windows, PowerShell comes pre-installed, but there are two versions of it. [Read more about it here](https://learn.microsoft.com/en-us/powershell/scripting/windows-powershell/install/installing-windows-powershell). The profile is located at different places depending on which version you're using:
  - Built in PowerShell (aka "Windows PowerShell"): `~\Documents\WindowsPowerShell\Microsoft.PowerShell_profile.ps1`
  - The newer, PowerShell >= 7, that's not built in: `~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1`

#### Windows Command Prompt aka Batch aka WinCMD

fnm is also supported but is not entirely covered. [You can set up a startup script](https://superuser.com/a/144348) and append the following line:

```batch
FOR /f "tokens=*" %i IN ('fnm env --use-on-cd') DO CALL %i
```

⚠️ If you get the error `i was unexpected at this time`, please make a .cmd file as suggested by the first step in the Usage with Cmder secton add it's path to the `AutoRun` registry key.

#### Usage with Cmder

Usage is very similar to the normal WinCMD install, apart for a few tweaks to allow being called from the cmder startup script. The example **assumes** that the `CMDER_ROOT` environment variable is **set** to the **root directory** of your Cmder installation.
Then you can do something like this:

- Make a .cmd file to invoke it

```batch
:: %CMDER_ROOT%\bin\fnm_init.cmd
@echo off
FOR /f "tokens=*" %%z IN ('fnm env --use-on-cd') DO CALL %%z
```

- Add it to the startup script

```batch
:: %CMDER_ROOT%\config\user_profile.cmd
call "%CMDER_ROOT%\bin\fnm_init.cmd"
```

You can replace `%CMDER_ROOT%` with any other convenient path too.

## [Usage](./docs/commands.md)

[See the available commands for an extended usage documentation](./docs/commands.md)

## Contributing

PRs welcome :tada:

### Developing:

```sh
# Install Rust
git clone https://github.com/Schniz/fnm.git
cd fnm/
cargo build
```

### Running Binary:

```sh
cargo run -- --help # Will behave like `fnm --help`
```

### Running Tests:

```sh
cargo test
```
