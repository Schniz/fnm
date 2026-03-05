---
"fnm": minor
---

Reduce shell startup overhead for `env --use-on-cd` by applying the initial version during `fnm env`, instead of requiring an immediate extra `fnm use` subprocess from shell hook output.
