---
"fnm": patch
---

guard the documented shell setup snippets (and the installer-appended ones) behind an interactive-shell check so non-interactive logins, such as display managers scraping env vars at session start, no longer leak a stale `fnm_multishells` path into `$PATH`. Closes #1551.
