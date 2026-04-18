---
"fnm": patch
---

Refine `--reinstall-packages-from` by discovering global packages from the installed Node directory instead of `npm ls`, improving determinism and avoiding npm CLI output and exit-code edge cases.
