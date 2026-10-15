---
"fnm": patch
---

Verify every Node.js download against the `SHASUMS256.txt` the mirror publishes for that release before unpacking it. A download whose SHA-256 doesn't match, or that the mirror publishes no checksum for, is not installed, and the error says which file and both checksums.
