---
"fnm": patch
---

Add Debian packages for amd64 and arm64

`.deb` packages are now built for amd64 and arm64, wrapping the same statically linked musl binary already shipped in the `fnm-linux` / `fnm-arm64` zip assets. They're a convenience download — install with `sudo dpkg -i fnm_<version>-1_<arch>.deb` — not a package submitted to Debian's official archive. Like every other release asset, they're produced as CI artifacts and attached to releases by hand.
