---
"fnm": minor
---

Verify Node.js downloads before installing them ([#1609](https://github.com/Schniz/fnm/issues/1609))

**Behavior change:** `fnm install` now checks the release's `SHASUMS256.txt.asc` signature against the bundled [Node.js release keys](https://github.com/nodejs/release-keys), and the archive's SHA-256 against the signed list, before it unpacks anything. A download that fails a check is not installed. A mirror that publishes no `SHASUMS256.txt.asc` now fails the install, with a hint to set the new `--download-verification` / `FNM_DOWNLOAD_VERIFICATION` option:

- `signature` (default): signature and checksum.
- `checksum`: checksum only, for private mirrors without signatures. unofficial-builds.nodejs.org gets this automatically.
- `none`: no checks.

Each release is pinned to the releaser key that signed it, from a bundled table; a release newer than the table is accepted only from an active releaser's key. New releaser keys and revocations take effect with fnm releases. Until then, a new releaser's key can be trusted with the new `--node-release-keys` / `FNM_NODE_RELEASE_KEYS` option.

```sh-session
$ fnm install 24
Installing Node v24.22.2 (arm64)
Verified signature by Antoine du Hamel <duhamelantoine1995@gmail.com> (5BE8A3F6C8A5C01D106C0AD820B1A390B168D356)
```
