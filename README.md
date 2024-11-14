<h1 align="center">
  Fast Node Manager (<code>fnm</code>)
  <img alt="Amount of downloads" src="https://img.shields.io/github/downloads/Schniz/fnm/total.svg?style=flat" />
  <a href="https://github.com/Schniz/fnm/actions"><img src="https://img.shields.io/github/actions/workflow/status/Schniz/fnm/rust.yml?branch=master&label=workflow" alt="GitHub Actions workflow status" /></a>
</h1>

> 🚀 Fast and simple Node.js version manager, built in Rust

<div align="center">
  <img src="./docs/fnm.svg" alt="Blazing fast!">
</div>

## Features

🌎 Cross-platform support (macOS, Windows, Linux)

✨ Single file, easy installation, instant startup

🚀 Built with speed in mind

📂 Works with `.node-version` and `.nvmrc` files

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

> [!NOTE]
> Check out the [Configuration](./docs/configuration.md) section to enable highly
> recommended features, like automatic version switching.

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
eval "$(fnm env --use-on-cd --shell bash)"
```

#### Zsh

Add the following to your `.zshrc` profile:

```zsh
eval "$(fnm env --use-on-cd --shell zsh)"
```

#### Fish shell

Create `~/.config/fish/conf.d/fnm.fish` and add this line to it:

```fish
fnm env --use-on-cd --shell fish | source
```

#### PowerShell

Add the following to the end of your profile file:

```powershell
fnm env --use-on-cd --shell powershell | Out-String | Invoke-Expression
```

- For macOS/Linux, the profile is located at `~/.config/powershell/Microsoft.PowerShell_profile.ps1`
- On Windows to edit your profile you can run this in a PowerShell
  ```powershell
  notepad $profile
  ```

#### Windows Command Prompt aka Batch aka WinCMD

fnm is also supported but is not entirely covered. You can set up a startup script for [cmd.exe]( https://superuser.com/a/144348) or [Windows Terminal](https://superuser.com/a/1855283) and append the following lines:

```batch
@echo off
:: for /F will launch a new instance of cmd so we create a guard to prevent an infnite loop
if not defined FNM_AUTORUN_GUARD (
    set "FNM_AUTORUN_GUARD=AutorunGuard"
    FOR /f "tokens=*" %%z IN ('fnm env --use-on-cd') DO CALL %%z
)
```

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

## [Configuration](./docs/configuration.md)

[See the available configuration options for an extended configuration documentation](./docs/configuration.md)

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
