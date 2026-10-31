# 配置

fnm 内置了很多开箱即用的功能。其中一些默认未启用，因为它们会改变你 shell 的默认行为；另一些则是功能开关，用于避免破坏性变更，或在我们确认值得引入前作为实验特性存在。

这些能力都可以通过在初始化 shell 时给 `fnm env` 添加参数来配置。比如，如果你的 shell 配置是 `eval "$(fnm env)"`，那么可以改成 `eval "$(fnm env --my-flag=value)"` 来增加一个参数。

下面是这些特性与能力的列表：

### `--use-on-cd`

**✅ 强烈推荐**

`--use-on-cd` 会在 `fnm env` 的输出中追加脚本逻辑，用于在你切换目录时挂载到 shell，并根据当前目录中的 `.node-version` 或 `.nvmrc`（若启用了 `--resolve-engines`，还包括 `package.json#engines#node`）自动切换 Node.js 版本。

这样你就不需要总是手动执行 `fnm use`，只要 `cd <DIR>` 即可生效。

### `--version-file-strategy=recursive`

**✅ 强烈推荐**

让 `fnm use` 和 `fnm install` 在未提供版本参数时，查找版本文件（"dotfile"）时会考虑父目录。

例如我们有如下目录结构：

```
repo/
├── package.json
├── .node-version <- 内容：`20.0.0`
└── packages/
  └── my-package/ <- 我在这里
    └── package.json
```

并执行以下命令：

```sh-session
repo/packages/my-package$ fnm use
```

那么 fnm 会切换到 Node.js v20.0.0。

如果不显式启用该参数，默认值是 `local`，不会向上遍历目录树，因此会输出：

```sh-session
repo/packages/my-package$ fnm use
error: Can't find version in dotfiles. Please provide a version manually to the command.
```

### `--corepack-enabled`

**🧪 实验性**

当安装新的 Node.js 版本时执行 [`corepack enable`](https://nodejs.org/api/corepack.html#enabling-the-feature)。之所以是实验性，是因为 Corepack 本身也仍是实验性特性。

### `--resolve-engines`

**🧪 实验性**

将 `package.json#engines#node` 视为有效的 Node.js 版本文件（"dotfile"）。例如你有如下 `package.json`：

```json
{
  "engines": {
    "node": ">=20 <21"
  }
}
```

那么：

- `fnm install` 会安装 Node.js 发行服务器中满足条件的最新 20.x 版本
- `fnm use` 会使用你系统中满足条件的最新 20.x 版本；如果没有匹配版本，则提示安装
