---
"fnm": minor
---

Added `--reinstall-packages-from` flag to `fnm install`. When specified, global npm packages from the given Node version are automatically reinstalled on the newly installed version. Analogous to nvm's `--reinstall-packages-from` flag.

Package discovery now uses filesystem scanning on non-Windows platforms and `npm ls --global --depth=0 --json` on Windows; symlinked global package entries are skipped to avoid reinstalling locally linked packages.
