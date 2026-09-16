---
"fnm": patch
---

Return `128 + signal` from `fnm exec` when its child is terminated by a signal on Unix, instead of reporting an error with exit code 1. Fixes [#649](https://github.com/Schniz/fnm/issues/649).
