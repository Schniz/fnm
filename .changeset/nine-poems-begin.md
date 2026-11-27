---
"fnm": patch
---

fix: add `package.json` detection for `--use-on-cd` on Windows CMD

this will make sure we run `fnm use` if using `package.json#engines`.
