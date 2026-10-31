# `fnm`

```
一个快速且简洁的 Node.js 管理器

用法：fnm [OPTIONS] <COMMAND>

命令：
  list-remote  列出所有远程 Node.js 版本 [aliases: ls-remote]
  list         列出所有本地已安装的 Node.js 版本 [aliases: ls]
  install      安装新的 Node.js 版本 [aliases: i]
  use          切换 Node.js 版本
  env          打印并配置 fnm 所需的环境变量
  completions  将 shell 自动补全输出到 stdout
  alias        为版本设置常用别名
  unalias      移除别名定义
  default      将某个版本设为默认版本，或获取当前默认版本
  current      打印当前 Node.js 版本
  exec         在 fnm 上下文中运行命令
  uninstall    卸载 Node.js 版本 [aliases: uni]
  help         打印本消息或指定子命令的帮助信息

选项：
      --node-dist-mirror <NODE_DIST_MIRROR>
          <https://nodejs.org/dist/> 镜像地址

          [env: FNM_NODE_DIST_MIRROR]
          [default: https://nodejs.org/dist]

      --fnm-dir <BASE_DIR>
          fnm 安装的根目录

          [env: FNM_DIR]

      --log-level <LOG_LEVEL>
          fnm 命令的日志级别

          [env: FNM_LOGLEVEL]
          [default: info]
          [possible values: quiet, error, info]

      --arch <ARCH>
          覆盖已安装 Node 二进制文件的架构。默认使用 fnm 二进制文件的架构

          [env: FNM_ARCH]

      --version-file-strategy <VERSION_FILE_STRATEGY>
          Node 版本解析策略。当 `fnm use` 或 `fnm install` 未提供版本参数时，或在启用了 `--use-on-cd` 的求值流程中使用

          [env: FNM_VERSION_FILE_STRATEGY]
          [default: local]

          可选值：
          - local:     使用当前目录中定义的本地 Node 版本
          - recursive: 使用当前目录及其所有父目录中定义的 Node 版本

      --corepack-enabled
          为每次新安装启用 corepack 支持。这会让 fnm 在每次安装 Node.js 后执行 `corepack enable`。更多信息见 <https://nodejs.org/api/corepack.html>

          [env: FNM_COREPACK_ENABLED]

      --resolve-engines [<RESOLVE_ENGINES>]
          当不存在 `.node-version` 或 `.nvmrc` 文件时，解析 `package.json` 中的 `engines.node` 字段。
          该功能默认启用。若要禁用，请传入 `--resolve-engines=false`。

          注意：`engines.node` 可以是任意 semver 范围，最终会解析为满足条件的最新版本。
          注意 2：如果你禁用了它，请在 GitHub 提交 issue 说明禁用原因。
                  未来禁用它可能不再产生效果，因此了解你为何禁用它很有价值。

          [env: FNM_RESOLVE_ENGINES]
          [possible values: true, false]

  -h, --help
          打印帮助（可用 '-h' 查看摘要）

  -V, --version
          打印版本号
```

# `fnm list-remote`

```
列出所有远程 Node.js 版本

用法：fnm list-remote [OPTIONS]

选项：
      --filter <FILTER>
          按用户指定版本或 semver 范围过滤版本

      --node-dist-mirror <NODE_DIST_MIRROR>
          <https://nodejs.org/dist/> 镜像地址

          [env: FNM_NODE_DIST_MIRROR]
          [default: https://nodejs.org/dist]

      --fnm-dir <BASE_DIR>
          fnm 安装的根目录

          [env: FNM_DIR]

      --lts [<LTS>]
          仅显示 LTS 版本（可按 LTS 代号进一步过滤）

      --sort <SORT>
          版本排序顺序

          [default: asc]

          可选值：
          - desc: 按降序排序（从最新到最早）
          - asc:  按升序排序（从最早到最新）

      --latest
          仅显示最新的匹配版本

      --log-level <LOG_LEVEL>
          fnm 命令的日志级别

          [env: FNM_LOGLEVEL]
          [default: info]
          [possible values: quiet, error, info]

      --arch <ARCH>
          覆盖已安装 Node 二进制文件的架构。默认使用 fnm 二进制文件的架构

          [env: FNM_ARCH]

      --version-file-strategy <VERSION_FILE_STRATEGY>
          Node 版本解析策略。当 `fnm use` 或 `fnm install` 未提供版本参数时，或在启用了 `--use-on-cd` 的求值流程中使用

          [env: FNM_VERSION_FILE_STRATEGY]
          [default: local]

          可选值：
          - local:     使用当前目录中定义的本地 Node 版本
          - recursive: 使用当前目录及其所有父目录中定义的 Node 版本

      --corepack-enabled
          为每次新安装启用 corepack 支持。这会让 fnm 在每次安装 Node.js 后执行 `corepack enable`。更多信息见 <https://nodejs.org/api/corepack.html>

          [env: FNM_COREPACK_ENABLED]

      --resolve-engines [<RESOLVE_ENGINES>]
          当不存在 `.node-version` 或 `.nvmrc` 文件时，解析 `package.json` 中的 `engines.node` 字段。
          该功能默认启用。若要禁用，请传入 `--resolve-engines=false`。

          注意：`engines.node` 可以是任意 semver 范围，最终会解析为满足条件的最新版本。
          注意 2：如果你禁用了它，请在 GitHub 提交 issue 说明禁用原因。
                  未来禁用它可能不再产生效果，因此了解你为何禁用它很有价值。

          [env: FNM_RESOLVE_ENGINES]
          [possible values: true, false]

  -h, --help
          打印帮助（可用 '-h' 查看摘要）
```

# `fnm list`

```
列出所有本地已安装的 Node.js 版本

用法：fnm list [OPTIONS]

选项：
      --node-dist-mirror <NODE_DIST_MIRROR>
          <https://nodejs.org/dist/> 镜像地址

          [env: FNM_NODE_DIST_MIRROR]
          [default: https://nodejs.org/dist]

      --fnm-dir <BASE_DIR>
          fnm 安装的根目录

          [env: FNM_DIR]

      --log-level <LOG_LEVEL>
          fnm 命令的日志级别

          [env: FNM_LOGLEVEL]
          [default: info]
          [possible values: quiet, error, info]

      --arch <ARCH>
          覆盖已安装 Node 二进制文件的架构。默认使用 fnm 二进制文件的架构

          [env: FNM_ARCH]

      --version-file-strategy <VERSION_FILE_STRATEGY>
          Node 版本解析策略。当 `fnm use` 或 `fnm install` 未提供版本参数时，或在启用了 `--use-on-cd` 的求值流程中使用

          [env: FNM_VERSION_FILE_STRATEGY]
          [default: local]

          可选值：
          - local:     使用当前目录中定义的本地 Node 版本
          - recursive: 使用当前目录及其所有父目录中定义的 Node 版本

      --corepack-enabled
          为每次新安装启用 corepack 支持。这会让 fnm 在每次安装 Node.js 后执行 `corepack enable`。更多信息见 <https://nodejs.org/api/corepack.html>

          [env: FNM_COREPACK_ENABLED]

      --resolve-engines [<RESOLVE_ENGINES>]
          当不存在 `.node-version` 或 `.nvmrc` 文件时，解析 `package.json` 中的 `engines.node` 字段。
          该功能默认启用。若要禁用，请传入 `--resolve-engines=false`。

          注意：`engines.node` 可以是任意 semver 范围，最终会解析为满足条件的最新版本。
          注意 2：如果你禁用了它，请在 GitHub 提交 issue 说明禁用原因。
                  未来禁用它可能不再产生效果，因此了解你为何禁用它很有价值。

          [env: FNM_RESOLVE_ENGINES]
          [possible values: true, false]

  -h, --help
          打印帮助（可用 '-h' 查看摘要）
```

# `fnm install`

```
安装新的 Node.js 版本

用法：fnm install [OPTIONS] [VERSION]

参数：
  [VERSION]
          版本字符串。可以是部分 semver，或 `lts/NAME` 格式的 LTS 版本名

选项：
      --lts
          安装最新 LTS 版本

      --node-dist-mirror <NODE_DIST_MIRROR>
          <https://nodejs.org/dist/> 镜像地址

          [env: FNM_NODE_DIST_MIRROR]
          [default: https://nodejs.org/dist]

      --fnm-dir <BASE_DIR>
          fnm 安装的根目录

          [env: FNM_DIR]

      --latest
          安装最新版本

      --progress <PROGRESS>
          显示用于下载状态的交互式进度条

          [default: auto]
          [possible values: auto, never, always]

      --log-level <LOG_LEVEL>
          fnm 命令的日志级别

          [env: FNM_LOGLEVEL]
          [default: info]
          [possible values: quiet, error, info]

      --use
          安装后立即使用该版本

      --arch <ARCH>
          覆盖已安装 Node 二进制文件的架构。默认使用 fnm 二进制文件的架构

          [env: FNM_ARCH]

      --version-file-strategy <VERSION_FILE_STRATEGY>
          Node 版本解析策略。当 `fnm use` 或 `fnm install` 未提供版本参数时，或在启用了 `--use-on-cd` 的求值流程中使用

          [env: FNM_VERSION_FILE_STRATEGY]
          [default: local]

          可选值：
          - local:     使用当前目录中定义的本地 Node 版本
          - recursive: 使用当前目录及其所有父目录中定义的 Node 版本

      --corepack-enabled
          为每次新安装启用 corepack 支持。这会让 fnm 在每次安装 Node.js 后执行 `corepack enable`。更多信息见 <https://nodejs.org/api/corepack.html>

          [env: FNM_COREPACK_ENABLED]

      --resolve-engines [<RESOLVE_ENGINES>]
          当不存在 `.node-version` 或 `.nvmrc` 文件时，解析 `package.json` 中的 `engines.node` 字段。
          该功能默认启用。若要禁用，请传入 `--resolve-engines=false`。

          注意：`engines.node` 可以是任意 semver 范围，最终会解析为满足条件的最新版本。
          注意 2：如果你禁用了它，请在 GitHub 提交 issue 说明禁用原因。
                  未来禁用它可能不再产生效果，因此了解你为何禁用它很有价值。

          [env: FNM_RESOLVE_ENGINES]
          [possible values: true, false]

  -h, --help
          打印帮助（可用 '-h' 查看摘要）
```

# `fnm use`

```
切换 Node.js 版本

用法：fnm use [OPTIONS] [VERSION]

参数：
  [VERSION]


选项：
      --install-if-missing
          若该版本尚未安装，则先安装

      --node-dist-mirror <NODE_DIST_MIRROR>
          <https://nodejs.org/dist/> 镜像地址

          [env: FNM_NODE_DIST_MIRROR]
          [default: https://nodejs.org/dist]

      --fnm-dir <BASE_DIR>
          fnm 安装的根目录

          [env: FNM_DIR]

      --silent-if-unchanged
          如果执行该命令后使用版本不会变化，则不输出当前版本提示信息

      --log-level <LOG_LEVEL>
          fnm 命令的日志级别

          [env: FNM_LOGLEVEL]
          [default: info]
          [possible values: quiet, error, info]

      --arch <ARCH>
          覆盖已安装 Node 二进制文件的架构。默认使用 fnm 二进制文件的架构

          [env: FNM_ARCH]

      --version-file-strategy <VERSION_FILE_STRATEGY>
          Node 版本解析策略。当 `fnm use` 或 `fnm install` 未提供版本参数时，或在启用了 `--use-on-cd` 的求值流程中使用

          [env: FNM_VERSION_FILE_STRATEGY]
          [default: local]

          可选值：
          - local:     使用当前目录中定义的本地 Node 版本
          - recursive: 使用当前目录及其所有父目录中定义的 Node 版本

      --corepack-enabled
          为每次新安装启用 corepack 支持。这会让 fnm 在每次安装 Node.js 后执行 `corepack enable`。更多信息见 <https://nodejs.org/api/corepack.html>

          [env: FNM_COREPACK_ENABLED]

      --resolve-engines [<RESOLVE_ENGINES>]
          当不存在 `.node-version` 或 `.nvmrc` 文件时，解析 `package.json` 中的 `engines.node` 字段。
          该功能默认启用。若要禁用，请传入 `--resolve-engines=false`。

          注意：`engines.node` 可以是任意 semver 范围，最终会解析为满足条件的最新版本。
          注意 2：如果你禁用了它，请在 GitHub 提交 issue 说明禁用原因。
                  未来禁用它可能不再产生效果，因此了解你为何禁用它很有价值。

          [env: FNM_RESOLVE_ENGINES]
          [possible values: true, false]

  -h, --help
          打印帮助（可用 '-h' 查看摘要）
```

# `fnm env`

```
打印并配置 fnm 所需的环境变量

该命令会生成一组 shell 命令，你应在 shell 中求值它们，以创建可用的 fnm 环境。

每种 shell 求值动态表达式的语法都不同。例如，在 Bash 和 Zsh 中可以写成 `eval "$(fnm env --shell bash)"`；在 Fish 中则是 `fnm env --shell fish | source`

用法：fnm env [OPTIONS]

选项：
      --node-dist-mirror <NODE_DIST_MIRROR>
          <https://nodejs.org/dist/> 镜像地址

          [env: FNM_NODE_DIST_MIRROR]
          [default: https://nodejs.org/dist]

      --shell <SHELL>
          要使用的 shell 语法。未提供时会自动推断

          [possible values: bash, zsh, fish, powershell]

      --fnm-dir <BASE_DIR>
          fnm 安装的根目录

          [env: FNM_DIR]

      --json
          输出 JSON 而不是 shell 命令

      --log-level <LOG_LEVEL>
          fnm 命令的日志级别

          [env: FNM_LOGLEVEL]
          [default: info]
          [possible values: quiet, error, info]

      --use-on-cd
          输出在每次切换目录时切换 Node 版本的脚本

      --arch <ARCH>
          覆盖已安装 Node 二进制文件的架构。默认使用 fnm 二进制文件的架构

          [env: FNM_ARCH]

      --version-file-strategy <VERSION_FILE_STRATEGY>
          Node 版本解析策略。当 `fnm use` 或 `fnm install` 未提供版本参数时，或在启用了 `--use-on-cd` 的求值流程中使用

          [env: FNM_VERSION_FILE_STRATEGY]
          [default: local]

          可选值：
          - local:     使用当前目录中定义的本地 Node 版本
          - recursive: 使用当前目录及其所有父目录中定义的 Node 版本

      --corepack-enabled
          为每次新安装启用 corepack 支持。这会让 fnm 在每次安装 Node.js 后执行 `corepack enable`。更多信息见 <https://nodejs.org/api/corepack.html>

          [env: FNM_COREPACK_ENABLED]

      --resolve-engines [<RESOLVE_ENGINES>]
          当不存在 `.node-version` 或 `.nvmrc` 文件时，解析 `package.json` 中的 `engines.node` 字段。
          该功能默认启用。若要禁用，请传入 `--resolve-engines=false`。

          注意：`engines.node` 可以是任意 semver 范围，最终会解析为满足条件的最新版本。
          注意 2：如果你禁用了它，请在 GitHub 提交 issue 说明禁用原因。
                  未来禁用它可能不再产生效果，因此了解你为何禁用它很有价值。

          [env: FNM_RESOLVE_ENGINES]
          [possible values: true, false]

  -h, --help
          打印帮助（可用 '-h' 查看摘要）
```

# `fnm completions`

```
将 shell 自动补全输出到 stdout

用法：fnm completions [OPTIONS]

选项：
      --node-dist-mirror <NODE_DIST_MIRROR>
          <https://nodejs.org/dist/> 镜像地址

          [env: FNM_NODE_DIST_MIRROR]
          [default: https://nodejs.org/dist]

      --shell <SHELL>
          要使用的 shell 语法。未提供时会自动推断

          [possible values: bash, zsh, fish, powershell]

      --fnm-dir <BASE_DIR>
          fnm 安装的根目录

          [env: FNM_DIR]

      --log-level <LOG_LEVEL>
          fnm 命令的日志级别

          [env: FNM_LOGLEVEL]
          [default: info]
          [possible values: quiet, error, info]

      --arch <ARCH>
          覆盖已安装 Node 二进制文件的架构。默认使用 fnm 二进制文件的架构

          [env: FNM_ARCH]

      --version-file-strategy <VERSION_FILE_STRATEGY>
          Node 版本解析策略。当 `fnm use` 或 `fnm install` 未提供版本参数时，或在启用了 `--use-on-cd` 的求值流程中使用

          [env: FNM_VERSION_FILE_STRATEGY]
          [default: local]

          可选值：
          - local:     使用当前目录中定义的本地 Node 版本
          - recursive: 使用当前目录及其所有父目录中定义的 Node 版本

      --corepack-enabled
          为每次新安装启用 corepack 支持。这会让 fnm 在每次安装 Node.js 后执行 `corepack enable`。更多信息见 <https://nodejs.org/api/corepack.html>

          [env: FNM_COREPACK_ENABLED]

      --resolve-engines [<RESOLVE_ENGINES>]
          当不存在 `.node-version` 或 `.nvmrc` 文件时，解析 `package.json` 中的 `engines.node` 字段。
          该功能默认启用。若要禁用，请传入 `--resolve-engines=false`。

          注意：`engines.node` 可以是任意 semver 范围，最终会解析为满足条件的最新版本。
          注意 2：如果你禁用了它，请在 GitHub 提交 issue 说明禁用原因。
                  未来禁用它可能不再产生效果，因此了解你为何禁用它很有价值。

          [env: FNM_RESOLVE_ENGINES]
          [possible values: true, false]

  -h, --help
          打印帮助（可用 '-h' 查看摘要）
```

# `fnm alias`

```
为版本设置常用别名

用法：fnm alias [OPTIONS] <TO_VERSION> <NAME>

参数：
  <TO_VERSION>


  <NAME>


选项：
      --node-dist-mirror <NODE_DIST_MIRROR>
          <https://nodejs.org/dist/> 镜像地址

          [env: FNM_NODE_DIST_MIRROR]
          [default: https://nodejs.org/dist]

      --fnm-dir <BASE_DIR>
          fnm 安装的根目录

          [env: FNM_DIR]

      --log-level <LOG_LEVEL>
          fnm 命令的日志级别

          [env: FNM_LOGLEVEL]
          [default: info]
          [possible values: quiet, error, info]

      --arch <ARCH>
          覆盖已安装 Node 二进制文件的架构。默认使用 fnm 二进制文件的架构

          [env: FNM_ARCH]

      --version-file-strategy <VERSION_FILE_STRATEGY>
          Node 版本解析策略。当 `fnm use` 或 `fnm install` 未提供版本参数时，或在启用了 `--use-on-cd` 的求值流程中使用

          [env: FNM_VERSION_FILE_STRATEGY]
          [default: local]

          可选值：
          - local:     使用当前目录中定义的本地 Node 版本
          - recursive: 使用当前目录及其所有父目录中定义的 Node 版本

      --corepack-enabled
          为每次新安装启用 corepack 支持。这会让 fnm 在每次安装 Node.js 后执行 `corepack enable`。更多信息见 <https://nodejs.org/api/corepack.html>

          [env: FNM_COREPACK_ENABLED]

      --resolve-engines [<RESOLVE_ENGINES>]
          当不存在 `.node-version` 或 `.nvmrc` 文件时，解析 `package.json` 中的 `engines.node` 字段。
          该功能默认启用。若要禁用，请传入 `--resolve-engines=false`。

          注意：`engines.node` 可以是任意 semver 范围，最终会解析为满足条件的最新版本。
          注意 2：如果你禁用了它，请在 GitHub 提交 issue 说明禁用原因。
                  未来禁用它可能不再产生效果，因此了解你为何禁用它很有价值。

          [env: FNM_RESOLVE_ENGINES]
          [possible values: true, false]

  -h, --help
          打印帮助（可用 '-h' 查看摘要）
```

# `fnm unalias`

```
移除别名定义

用法：fnm unalias [OPTIONS] <REQUESTED_ALIAS>

参数：
  <REQUESTED_ALIAS>


选项：
      --node-dist-mirror <NODE_DIST_MIRROR>
          <https://nodejs.org/dist/> 镜像地址

          [env: FNM_NODE_DIST_MIRROR]
          [default: https://nodejs.org/dist]

      --fnm-dir <BASE_DIR>
          fnm 安装的根目录

          [env: FNM_DIR]

      --log-level <LOG_LEVEL>
          fnm 命令的日志级别

          [env: FNM_LOGLEVEL]
          [default: info]
          [possible values: quiet, error, info]

      --arch <ARCH>
          覆盖已安装 Node 二进制文件的架构。默认使用 fnm 二进制文件的架构

          [env: FNM_ARCH]

      --version-file-strategy <VERSION_FILE_STRATEGY>
          Node 版本解析策略。当 `fnm use` 或 `fnm install` 未提供版本参数时，或在启用了 `--use-on-cd` 的求值流程中使用

          [env: FNM_VERSION_FILE_STRATEGY]
          [default: local]

          可选值：
          - local:     使用当前目录中定义的本地 Node 版本
          - recursive: 使用当前目录及其所有父目录中定义的 Node 版本

      --corepack-enabled
          为每次新安装启用 corepack 支持。这会让 fnm 在每次安装 Node.js 后执行 `corepack enable`。更多信息见 <https://nodejs.org/api/corepack.html>

          [env: FNM_COREPACK_ENABLED]

      --resolve-engines [<RESOLVE_ENGINES>]
          当不存在 `.node-version` 或 `.nvmrc` 文件时，解析 `package.json` 中的 `engines.node` 字段。
          该功能默认启用。若要禁用，请传入 `--resolve-engines=false`。

          注意：`engines.node` 可以是任意 semver 范围，最终会解析为满足条件的最新版本。
          注意 2：如果你禁用了它，请在 GitHub 提交 issue 说明禁用原因。
                  未来禁用它可能不再产生效果，因此了解你为何禁用它很有价值。

          [env: FNM_RESOLVE_ENGINES]
          [possible values: true, false]

  -h, --help
          打印帮助（可用 '-h' 查看摘要）
```

# `fnm default`

```
将某个版本设为默认版本，或获取当前默认版本。

这是 `fnm alias VERSION default` 的简写

用法：fnm default [OPTIONS] [VERSION]

参数：
  [VERSION]


选项：
      --node-dist-mirror <NODE_DIST_MIRROR>
          <https://nodejs.org/dist/> 镜像地址

          [env: FNM_NODE_DIST_MIRROR]
          [default: https://nodejs.org/dist]

      --fnm-dir <BASE_DIR>
          fnm 安装的根目录

          [env: FNM_DIR]

      --log-level <LOG_LEVEL>
          fnm 命令的日志级别

          [env: FNM_LOGLEVEL]
          [default: info]
          [possible values: quiet, error, info]

      --arch <ARCH>
          覆盖已安装 Node 二进制文件的架构。默认使用 fnm 二进制文件的架构

          [env: FNM_ARCH]

      --version-file-strategy <VERSION_FILE_STRATEGY>
          Node 版本解析策略。当 `fnm use` 或 `fnm install` 未提供版本参数时，或在启用了 `--use-on-cd` 的求值流程中使用

          [env: FNM_VERSION_FILE_STRATEGY]
          [default: local]

          可选值：
          - local:     使用当前目录中定义的本地 Node 版本
          - recursive: 使用当前目录及其所有父目录中定义的 Node 版本

      --corepack-enabled
          为每次新安装启用 corepack 支持。这会让 fnm 在每次安装 Node.js 后执行 `corepack enable`。更多信息见 <https://nodejs.org/api/corepack.html>

          [env: FNM_COREPACK_ENABLED]

      --resolve-engines [<RESOLVE_ENGINES>]
          当不存在 `.node-version` 或 `.nvmrc` 文件时，解析 `package.json` 中的 `engines.node` 字段。
          该功能默认启用。若要禁用，请传入 `--resolve-engines=false`。

          注意：`engines.node` 可以是任意 semver 范围，最终会解析为满足条件的最新版本。
          注意 2：如果你禁用了它，请在 GitHub 提交 issue 说明禁用原因。
                  未来禁用它可能不再产生效果，因此了解你为何禁用它很有价值。

          [env: FNM_RESOLVE_ENGINES]
          [possible values: true, false]

  -h, --help
          打印帮助（可用 '-h' 查看摘要）
```

# `fnm current`

```
打印当前 Node.js 版本

用法：fnm current [OPTIONS]

选项：
      --node-dist-mirror <NODE_DIST_MIRROR>
          <https://nodejs.org/dist/> 镜像地址

          [env: FNM_NODE_DIST_MIRROR]
          [default: https://nodejs.org/dist]

      --fnm-dir <BASE_DIR>
          fnm 安装的根目录

          [env: FNM_DIR]

      --log-level <LOG_LEVEL>
          fnm 命令的日志级别

          [env: FNM_LOGLEVEL]
          [default: info]
          [possible values: quiet, error, info]

      --arch <ARCH>
          覆盖已安装 Node 二进制文件的架构。默认使用 fnm 二进制文件的架构

          [env: FNM_ARCH]

      --version-file-strategy <VERSION_FILE_STRATEGY>
          Node 版本解析策略。当 `fnm use` 或 `fnm install` 未提供版本参数时，或在启用了 `--use-on-cd` 的求值流程中使用

          [env: FNM_VERSION_FILE_STRATEGY]
          [default: local]

          可选值：
          - local:     使用当前目录中定义的本地 Node 版本
          - recursive: 使用当前目录及其所有父目录中定义的 Node 版本

      --corepack-enabled
          为每次新安装启用 corepack 支持。这会让 fnm 在每次安装 Node.js 后执行 `corepack enable`。更多信息见 <https://nodejs.org/api/corepack.html>

          [env: FNM_COREPACK_ENABLED]

      --resolve-engines [<RESOLVE_ENGINES>]
          当不存在 `.node-version` 或 `.nvmrc` 文件时，解析 `package.json` 中的 `engines.node` 字段。
          该功能默认启用。若要禁用，请传入 `--resolve-engines=false`。

          注意：`engines.node` 可以是任意 semver 范围，最终会解析为满足条件的最新版本。
          注意 2：如果你禁用了它，请在 GitHub 提交 issue 说明禁用原因。
                  未来禁用它可能不再产生效果，因此了解你为何禁用它很有价值。

          [env: FNM_RESOLVE_ENGINES]
          [possible values: true, false]

  -h, --help
          打印帮助（可用 '-h' 查看摘要）
```

# `fnm exec`

```
在 fnm 上下文中运行命令

示例：
--------
fnm exec --using=v12.0.0 node --version
=> v12.0.0

用法：fnm exec [OPTIONS] [ARGUMENTS]...

参数：
  [ARGUMENTS]...
          要运行的命令

选项：
      --node-dist-mirror <NODE_DIST_MIRROR>
          <https://nodejs.org/dist/> 镜像地址

          [env: FNM_NODE_DIST_MIRROR]
          [default: https://nodejs.org/dist]

      --using <VERSION>
          可以是显式版本号，或写有版本号的文件名

      --fnm-dir <BASE_DIR>
          fnm 安装的根目录

          [env: FNM_DIR]

      --log-level <LOG_LEVEL>
          fnm 命令的日志级别

          [env: FNM_LOGLEVEL]
          [default: info]
          [possible values: quiet, error, info]

      --arch <ARCH>
          覆盖已安装 Node 二进制文件的架构。默认使用 fnm 二进制文件的架构

          [env: FNM_ARCH]

      --version-file-strategy <VERSION_FILE_STRATEGY>
          Node 版本解析策略。当 `fnm use` 或 `fnm install` 未提供版本参数时，或在启用了 `--use-on-cd` 的求值流程中使用

          [env: FNM_VERSION_FILE_STRATEGY]
          [default: local]

          可选值：
          - local:     使用当前目录中定义的本地 Node 版本
          - recursive: 使用当前目录及其所有父目录中定义的 Node 版本

      --corepack-enabled
          为每次新安装启用 corepack 支持。这会让 fnm 在每次安装 Node.js 后执行 `corepack enable`。更多信息见 <https://nodejs.org/api/corepack.html>

          [env: FNM_COREPACK_ENABLED]

      --resolve-engines [<RESOLVE_ENGINES>]
          当不存在 `.node-version` 或 `.nvmrc` 文件时，解析 `package.json` 中的 `engines.node` 字段。
          该功能默认启用。若要禁用，请传入 `--resolve-engines=false`。

          注意：`engines.node` 可以是任意 semver 范围，最终会解析为满足条件的最新版本。
          注意 2：如果你禁用了它，请在 GitHub 提交 issue 说明禁用原因。
                  未来禁用它可能不再产生效果，因此了解你为何禁用它很有价值。

          [env: FNM_RESOLVE_ENGINES]
          [possible values: true, false]

  -h, --help
          打印帮助（可用 '-h' 查看摘要）
```

# `fnm uninstall`

```
卸载 Node.js 版本

> 警告：当传入别名时，会删除该别名所指向的 Node 版本，以及指向同一版本的其他别名。

用法：fnm uninstall [OPTIONS] [VERSION]

参数：
  [VERSION]


选项：
      --node-dist-mirror <NODE_DIST_MIRROR>
          <https://nodejs.org/dist/> 镜像地址

          [env: FNM_NODE_DIST_MIRROR]
          [default: https://nodejs.org/dist]

      --fnm-dir <BASE_DIR>
          fnm 安装的根目录

          [env: FNM_DIR]

      --log-level <LOG_LEVEL>
          fnm 命令的日志级别

          [env: FNM_LOGLEVEL]
          [default: info]
          [possible values: quiet, error, info]

      --arch <ARCH>
          覆盖已安装 Node 二进制文件的架构。默认使用 fnm 二进制文件的架构

          [env: FNM_ARCH]

      --version-file-strategy <VERSION_FILE_STRATEGY>
          Node 版本解析策略。当 `fnm use` 或 `fnm install` 未提供版本参数时，或在启用了 `--use-on-cd` 的求值流程中使用

          [env: FNM_VERSION_FILE_STRATEGY]
          [default: local]

          可选值：
          - local:     使用当前目录中定义的本地 Node 版本
          - recursive: 使用当前目录及其所有父目录中定义的 Node 版本

      --corepack-enabled
          为每次新安装启用 corepack 支持。这会让 fnm 在每次安装 Node.js 后执行 `corepack enable`。更多信息见 <https://nodejs.org/api/corepack.html>

          [env: FNM_COREPACK_ENABLED]

      --resolve-engines [<RESOLVE_ENGINES>]
          当不存在 `.node-version` 或 `.nvmrc` 文件时，解析 `package.json` 中的 `engines.node` 字段。
          该功能默认启用。若要禁用，请传入 `--resolve-engines=false`。

          注意：`engines.node` 可以是任意 semver 范围，最终会解析为满足条件的最新版本。
          注意 2：如果你禁用了它，请在 GitHub 提交 issue 说明禁用原因。
                  未来禁用它可能不再产生效果，因此了解你为何禁用它很有价值。

          [env: FNM_RESOLVE_ENGINES]
          [possible values: true, false]

  -h, --help
          打印帮助（可用 '-h' 查看摘要）
```

# `fnm help`

```

```

