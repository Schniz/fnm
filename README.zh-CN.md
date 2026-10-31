<h1 align="center">
  Fast Node Manager (<code>fnm</code>)
  <img alt="Amount of downloads" src="https://img.shields.io/github/downloads/Schniz/fnm/total.svg?style=flat" />
  <a href="https://github.com/Schniz/fnm/actions"><img src="https://img.shields.io/github/actions/workflow/status/Schniz/fnm/rust.yml?branch=master&label=workflow" alt="GitHub Actions workflow status" /></a>
</h1>

[English](./README.md)

> 🚀 使用 Rust 构建的快速且简洁的 Node.js 版本管理器

<div align="center">
  <img src="./docs/fnm.svg" alt="Blazing fast!">
</div>

## 特性

🌎 跨平台支持（macOS、Windows、Linux）

✨ 单文件、易于安装、即时启动

🚀 以速度为核心设计

📂 支持 `.node-version` 和 `.nvmrc` 文件

## 安装

### 使用脚本（macOS/Linux）

对于 `bash`、`zsh` 和 `fish`，我们提供了[自动安装脚本](./.ci/install.sh)。

请先确保操作系统已安装 `curl` 和 `unzip`，然后执行：

```sh
curl -fsSL https://fnm.vercel.app/install | bash
```

#### 升级

在 macOS 上，直接执行 `brew upgrade fnm` 即可。

在其他操作系统上，升级 `fnm` 与安装方式几乎一致。为了避免在 shell 配置文件中重复写入内容，请在安装命令中传入 `--skip-shell`：

```sh
curl -fsSL https://fnm.vercel.app/install | bash -s -- --skip-shell
```

#### 参数

`--install-dir`

设置 fnm 的自定义安装目录。默认值为 `$XDG_DATA_HOME/fnm`（如果未定义 `$XDG_DATA_HOME`，Linux 会回退到 `$HOME/.local/share/fnm`，macOS 会回退到 `$HOME/Library/Application Support/fnm`）。

> **注意：** 在 macOS 上，此选项只有配合 `--force-install` 才有意义，因为默认安装方式是 Homebrew。

`--skip-shell`

跳过向 shell 配置文件追加加载器配置。具体会根据当前用户的 shell（由 `$SHELL` 定义）判断目标文件。例如 Bash 为 `$HOME/.bashrc`，Zsh 为 `$HOME/.zshrc`，Fish 为 `$HOME/.config/fish/conf.d/fnm.fish`。

`--force-install`

在 macOS 上，使用安装脚本安装已不推荐（推荐使用 Homebrew 公式），但该参数可强制继续使用脚本安装。

示例：

```sh
curl -fsSL https://fnm.vercel.app/install | bash -s -- --install-dir "./.fnm" --skip-shell
```

### 手动安装

#### 使用 Homebrew（macOS/Linux）

```sh
brew install fnm
```

然后，[为 fnm 配置 shell](#shell-setup)

#### 使用 Winget（Windows）

```sh
winget install Schniz.fnm
```

#### 使用 Scoop（Windows）

```sh
scoop install fnm
```

然后，[为 fnm 配置 shell](#shell-setup)

#### 使用 Chocolatey（Windows）

```sh
choco install fnm
```

然后，[为 fnm 配置 shell](#shell-setup)

#### 使用 Cargo（Linux/macOS/Windows）

```sh
cargo install fnm
```

然后，[为 fnm 配置 shell](#shell-setup)

#### 使用发行版二进制文件（Linux/macOS/Windows）

- 下载适用于你系统的[最新发布二进制文件](https://github.com/Schniz/fnm/releases)
- 将其加入全局 `PATH` 环境变量
- [为 fnm 配置 shell](#shell-setup)

### 卸载

要移除 fnm（😢），只需删除主目录中的 `.fnm` 文件夹。你也应当编辑 shell 配置，去掉所有与 fnm 相关的引用（即阅读 [Shell Setup](#shell-setup)，然后反向操作）。

## 自动补全

fnm 的自动补全随二进制文件一起提供：

```sh
fnm completions --shell <SHELL>
```

其中 `<SHELL>` 可以是以下已支持的 shell 之一：

- `bash`
- `zsh`
- `fish`
- `powershell`

请按照你所使用 shell 的说明进行安装。

<a id="shell-setup"></a>
### Shell Setup

在开始使用 fnm 之前，需要先配置环境变量。
方式是执行并求值 `fnm env` 的输出。

> [!TIP]
> 建议显式传入 shell（例如 `fnm env --shell bash`），而不是依赖运行时自动推断。这样会更快一些，也能避免进程树检测。

> [!NOTE]
> 请查看[配置](./docs/configuration.zh-CN.md)章节以启用强烈推荐的功能，例如自动切换版本。

在项目中添加 `.node-version` 非常简单：

```bash
$ node --version
v14.18.3
$ node --version > .node-version
```

根据你使用的 shell，参考以下指南：

#### Bash

将以下内容加入 `.bashrc`：

```bash
eval "$(fnm env --use-on-cd --shell bash)"
```

#### Zsh

将以下内容加入 `.zshrc`：

```zsh
eval "$(fnm env --use-on-cd --shell zsh)"
```

#### Fish shell

创建 `~/.config/fish/conf.d/fnm.fish`，并写入：

```fish
fnm env --use-on-cd --shell fish | source
```

#### PowerShell

在你的 profile 文件末尾加入以下内容：

```powershell
fnm env --use-on-cd --shell powershell | Out-String | Invoke-Expression
```

- 在 macOS/Linux 上，profile 文件位于 `~/.config/powershell/Microsoft.PowerShell_profile.ps1`
- 在 Windows 上，路径可能是：
  - `%userprofile%\Documents\WindowsPowerShell\Microsoft.PowerShell_profile.ps1` PowerShell 5
  - `%userprofile%\Documents\PowerShell\Microsoft.PowerShell_profile.ps1` PowerShell 6+
- 在 PowerShell 中可运行以下命令创建 profile 文件：
  ```powershell
  if (-not (Test-Path $profile)) { New-Item $profile -Force }
  ```
- 在 PowerShell 中可运行以下命令编辑 profile：
  ```powershell
  Invoke-Item $profile
  ```

#### Windows Command Prompt（又名 Batch/WinCMD）

fnm 也支持该环境，但覆盖并不完全。你可以为 [cmd.exe](https://superuser.com/a/144348) 或 [Windows Terminal](https://superuser.com/a/1855283) 设置启动脚本，并追加以下内容：

```batch
@echo off
:: for /F will launch a new instance of cmd so we create a guard to prevent an infnite loop
if not defined FNM_AUTORUN_GUARD (
    set "FNM_AUTORUN_GUARD=AutorunGuard"
    FOR /f "tokens=*" %%z IN ('fnm env --use-on-cd') DO CALL %%z
)
```

#### 在 Cmder 中使用

与普通 WinCMD 安装方式非常相似，只需做少量调整以支持从 Cmder 启动脚本调用。以下示例**假设**环境变量 `CMDER_ROOT` 已**设置**为 Cmder 安装的**根目录**。
你可以这样做：

- 新建一个 `.cmd` 文件来调用 fnm

```batch
:: %CMDER_ROOT%\bin\fnm_init.cmd
@echo off
FOR /f "tokens=*" %%z IN ('fnm env --use-on-cd') DO CALL %%z
```

- 把它加入启动脚本

```batch
:: %CMDER_ROOT%\config\user_profile.cmd
call "%CMDER_ROOT%\bin\fnm_init.cmd"
```

你也可以将 `%CMDER_ROOT%` 替换为任何你觉得更方便的路径。

## [配置](./docs/configuration.zh-CN.md)

[查看可用配置项和更完整的配置文档](./docs/configuration.zh-CN.md)

## [用法](./docs/commands.zh-CN.md)

[查看可用命令和更完整的用法文档](./docs/commands.zh-CN.md)

## 贡献

欢迎 PR :tada:

### 开发：

```sh
# Install Rust
git clone https://github.com/Schniz/fnm.git
cd fnm/
cargo build
```

### 运行二进制：

```sh
cargo run -- --help # Will behave like `fnm --help`
```

### 运行测试：

```sh
cargo test
```
