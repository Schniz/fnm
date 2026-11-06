---
"fnm": minor
---

Default the Node mirror to `https://unofficial-builds.nodejs.org/download/release/` on musl architectures (`x64-musl`, `arm64-musl`), since the official `https://nodejs.org/dist/` mirror does not ship musl builds. Explicit `--node-dist-mirror` or `FNM_NODE_DIST_MIRROR` values are still respected.
