---
"fnm": patch
---

Log the already-installed warning at info level so successful repeated installations do not write to stderr or appear with `--log-level error`. Fixes #1461.
