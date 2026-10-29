# Configuration

fnm comes with many features out of the box. Some of them are not activated by default as they’re changing your shell default behavior, and some are just a feature flag to avoid breaking changes or just experimental until we decide it is worthwhile to introduce them.

All these features can be configured by adding flags to the `fnm env` call when initializing the shell. For instance, if your shell set up looks like `eval "$(fnm env)"` then you can add a flag to it by changing it to `eval "$(fnm env --my-flag=value)"`

Here’s a list of these features and capabilities:

### `--use-on-cd`

**✅ Highly recommended**

`--use-on-cd` appends output to `fnm env`'s output that will hook into your shell upon changing directories, and will switch the Node.js version based on the requirements of the current directory, based on `.node-version` or `.nvmrc` (or `packages.json#engines#node` if `--resolve-engines` was enabled).

This allows you do avoid thinking about `fnm use`, and only `cd <DIR>` to make it work.

### `--version-file-strategy=recursive`

**✅ Highly recommended**

Makes `fnm use` and `fnm install` take parent directories into account when looking for a version file ("dotfile")--when no argument was given.

So, let's say we have the following directory structure:

```
repo/
├── package.json
├── .node-version <- with content: `20.0.0`
└── packages/
  └── my-package/ <- I am here
    └── package.json
```

And I'm running the following command:

```sh-session
repo/packages/my-package$ fnm use
```

Then fnm will switch to Node.js v20.0.0.

Without the explicit flag, the value is set to `local`, which will not traverse the directory tree and therefore will print:

```sh-session
repo/packages/my-package$ fnm use
error: Can't find version in dotfiles. Please provide a version manually to the command.
```

### `--corepack-enabled`

**🧪 Experimental**

Runs [`corepack enable`](https://nodejs.org/api/corepack.html#enabling-the-feature) when a new version of Node.js is installed. Experimental due to the fact Corepack itself is experimental.

### `--resolve-engines`

**🧪 Experimental**

Treats `package.json#engines#node` as a valid Node.js version file ("dotfile"). So, if you have a package.json with the following content:

```json
{
  "engines": {
    "node": ">=20 <21"
  }
}
```

Then:

- `fnm install` will install the latest satisfying Node.js 20.x version available in the Node.js dist server
- `fnm use` will use the latest satisfying Node.js 20.x version available on your system, or prompt to install if no version matched.

### `--download-verification`

**✅ On by default**

Sets how `fnm install` checks a Node.js download before it unpacks it. Also settable with `FNM_DOWNLOAD_VERIFICATION`.

- `signature` (default): fetches the release's `SHASUMS256.txt.asc`, checks its signature against the [Node.js release keys](https://github.com/nodejs/release-keys) bundled with fnm, and then checks the archive's SHA-256 against the signed list. On success, fnm prints the signer, for example `Verified signature by Antoine du Hamel <duhamelantoine1995@gmail.com> (5BE8A3F6C8A5C01D106C0AD820B1A390B168D356)`.
- `checksum`: checks the archive against the release's `SHASUMS256.txt` only. Use it for a private mirror that copies the releases but not their signatures.
- `none`: checks nothing.

A mismatch or a bad signature always stops the install, and so does a signature by a revoked key. A missing `SHASUMS256.txt.asc` or `SHASUMS256.txt` stops it too:

- On nodejs.org and unofficial-builds.nodejs.org, which always publish them, the error says that this is unexpected and can mean a publishing incident or an attack, and does not suggest a lower setting.
- Releases older than signed checksums (most v0.x releases before v0.10.0, and v0.10.7) have no `SHASUMS256.txt.asc` on nodejs.org. For them, the error says so: the most fnm can check is `SHASUMS256.txt`, with `--download-verification=checksum`.
- On other mirrors, the error says which setting would skip the missing file, if the mirror is a private one you control.

[unofficial-builds.nodejs.org](https://unofficial-builds.nodejs.org) publishes no signatures, so with `signature` fnm checks its checksums only, and prints a warning. Over plain `http://`, a checksum check protects against corruption, not tampering, and the warning says so.

When a release is signed by a key that fnm doesn't know, fnm fetches the current keys from [nodejs/release-keys](https://github.com/nodejs/release-keys) on GitHub (never from the mirror) and tries again. It takes only new signing subkeys of the keys it trusts, bound by those keys, and keeps them in `$FNM_DIR/release-keys/`. The keys fnm bundles are never changed this way: new releaser keys and revocations take effect with fnm releases. A release signed by a new releaser's key fails until fnm is upgraded, and the error says how to trust the key before that with `--node-release-keys`.

### `--node-release-keys`

**⚠️ Use with care**

An armored `.asc` file of more Node.js release signing keys to trust, in addition to the ones fnm bundles. Also settable with `FNM_NODE_RELEASE_KEYS`. Use it for a new releaser's key before an fnm release bundles it: check the key's full fingerprint against the [Node.js README](https://github.com/nodejs/node#release-keys) and [nodejs/release-keys](https://github.com/nodejs/release-keys) yourself first. Whoever controls a key in that file can sign any Node.js download for you.

These keys are checked like the bundled ones, revocations included. A release signed by one of them says so, for example `Verified signature by <name> (<fingerprint>, user key from /path/to/keys.asc)`. A file that can't be read, has a key that can't be read, or has no keys stops the install with an error that names it. The file can't be `$FNM_DIR/release-keys/keys.asc`, which is fnm's own cache of refreshed keys. A revocation in any copy of a key, in this file or in the bundle, applies to every copy.
