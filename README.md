<h1 align="center">
  Fast Node Manager (<code>fnm</code>)
  <img alt="Amount of downloads" src="https://img.shields.io/github/downloads/Schniz/fnm/total.svg?style=flat" />
  <a href="https://dev.azure.com/galstar0385/fnm/_build/latest?definitionId=1?branchName=master"><img alt="Build Status" src="https://dev.azure.com/galstar0385/fnm/_apis/build/status/Schniz.fnm?branchName=master" /></a>
</h1>

> :rocket: Fast and simple Node.js version manager, built in ReasonML

<div align="center">
  <img src="./docs/fnm.svg" alt="Blazing fast!">
</div>

## Features

:sparkles: Single file, easy installation

:rocket: Built with speed in mind

:thinking: Works with `.nvmrc` and `.node-version` files

## Installation

### Using a script

For `bash`, `zsh` and `fish` shells, there's an [automatic installation script](./.ci/install.sh):

```bash
curl https://raw.githubusercontent.com/Schniz/fnm/master/.ci/install.sh | bash
```

### Manually

- Download the [latest release binary](https://github.com/Schniz/fnm/releases) for your system
- Make it available globally on `$PATH`
- Add the following line to your `.bashrc`/`.zshrc` file:

  ```bash
  eval `fnm env --multi`
  ```

  If you are using [fish shell](https://fishshell.com/), add this line to your `config.fish` file:

  ```fish
  eval (fnm env --multi --fish)
  ```

## Usage

You can always use `fnm --help` to read the docs:

### `fnm install [VERSION]`

Installs `[VERSION]`. If no version provided, it will install the version specified in the `.nvmrc` file located in the current working directory.

### `fnm use [VERSION]`

Activates `[VERSION]` as the current Node version. If no version provided, it will activate the version specified in the `.nvmrc` or `.node-version` file located in the current working directory.

### `fnm ls`

Lists the installed Node versions.

### `fnm ls-remote`

Lists the Node versions available to download remotely.

### `fnm env [--multi] [--fish] [--node-dist-mirror=URI] [--base-dir=DIR]`

Prints the required shell commands in order to configure your shell, Bash compliant by default.

- Providing `--multi` will output the multishell support, allowing a different current Node version per shell
- Providing `--fish` will output the Fish-compliant version.
- Providing `--node-dist-mirror="https://npm.taobao.org/dist"` will use the Chinese mirror of Node.js
- Providing `--base-dir="/tmp/fnm"` will install and use versions in `/tmp/fnm` directory

## Future Plans

- [ ] Feature: make versions complete the latest: `10` would infer the latest minor and patch versions of node 10. `10.1` would infer the latest patch version of node 10.1
- [ ] Feature: `fnm use --install`
- [ ] Feature: `fnm install lts`?
- [ ] OSX: Add to homebrew?
- [ ] Windows Support?
- [ ] Linux: Replace `curl` usage with `cohttp`/`ocurl` or something else which is statically-linkable
- [ ] Linux: Replace `tar` with a statically linked library too (for ungzip + untar)

## Contributing

PRs welcome :tada:

### Developing:

```
npm install -g esy
git clone https://github.com/Schniz/fnm.git
cd fnm/
esy install
esy bootstrap
esy build
```

### Running Binary:

After building the project, you can run the main binary that is produced.

```
esy x fnm.exe
```

### Running Tests:

```
# Runs some smoke-unity test
esy test

# Runs the feature tests
feature_tests/run.sh
```
