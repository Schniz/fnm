---
"fnm": patch
---

Fix networking in corporate root-CA environments on Mac OS

This change enables os-native TLS handling in the `reqwest` crate, which allows fnm to make network requests in corporate environments that, for security reasons, proxy all traffic and have their own root certificate authority.
