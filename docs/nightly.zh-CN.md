# 使用 Node nightly 构建

fnm 没有提供用于安装 nightly 构建的快捷方式，但你可以通过 `--node-dist-mirror` 参数手动指定一个 "dist mirror" 来实现。

## 使用示例

下面是安装 Node 23 的示例。

先运行以下命令获取可用版本列表：

    fnm --node-dist-mirror https://nodejs.org/download/nightly/ ls-remote

使用并安装 nightly 版本：

    fnm --node-dist-mirror https://nodejs.org/download/nightly/ use 23
    # 或
    fnm --node-dist-mirror https://nodejs.org/download/nightly/ use v23.0.0-nightly202407253de7a4c374

安装完成后，你就可以在版本列表中看到它：

    fnm ls

并且之后可以不再传 `--node-dist-mirror` 参数直接使用：

    fnm use 23

