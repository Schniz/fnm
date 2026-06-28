---
"fnm": patch
---

Avoid embedding a stale PATH snapshot in PowerShell env output by referencing `$env:PATH` at evaluation time.
