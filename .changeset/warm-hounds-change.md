---
"fnm": minor
---

enable `--resolve-engines` by default. out of experimental phase.

to disable it, add a `--resolve-engines=false` flag, and make sure to open an issue describing _why_.
It might feel like a breaking change but .nvmrc and .node-version have precedence so it should not.

I am all in favor of better experience and I believe supporting engines.node is a good direction.
