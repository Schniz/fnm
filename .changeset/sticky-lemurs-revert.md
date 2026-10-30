---
"fnm": patch
---

Fixed `--use-on-cd` with `local` strategy to revert to the default Node version when entering a directory without a version file. Previously, the Node version would remain "stuck" on the last project version after leaving a versioned directory. This now matches nvm's auto-switching behavior.

