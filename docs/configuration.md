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

### `--auth-header` / `FNM_AUTH_HEADER`

Provides an HTTP `Authorization` header for requests to the Node.js distribution mirror. This is useful when downloading from private mirrors that require authentication.

**Note:** Unlike other `FNM_*` variables, `FNM_AUTH_HEADER` is intentionally excluded from `fnm env` output to prevent credentials from being propagated to child processes.

**Usage:**

```bash
# Bearer token authentication
export FNM_AUTH_HEADER="Bearer YOUR_TOKEN_HERE"

# Basic authentication
export FNM_AUTH_HEADER="Basic $(echo -n 'user:password' | base64)"
```

**Security Recommendations:**

- **Only use with HTTPS mirrors you know and trust.** Always verify your `FNM_NODE_DIST_MIRROR` uses `https://`.
- Never commit credentials to version control.
- Handle tokens securely and rotate regularly.
- Source credentials securely from outside your shell config:

  ```bash
  # Example: Read from a protected file (chmod 600)
  export FNM_AUTH_HEADER="Bearer $(cat ~/.secrets/fnm-token)"

  # Example: Use a secrets manager
  export FNM_AUTH_HEADER="Bearer $(vault kv get -field=token secret/fnm)"

  # Example: Use system keychain (macOS)
  export FNM_AUTH_HEADER="Bearer $(security find-generic-password -s fnm-token -w)"
  ```
