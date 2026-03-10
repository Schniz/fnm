# fnm (fast node manager)

## End-to-end tests

### What they do

E2E tests validate fnm behavior inside real shells, including shell integration, PATH updates, and cross-version workflows.
They are used for user-facing behavior that unit tests cannot fully cover (for example: `fnm env`, `fnm use`, global package visibility, and shell-specific behavior).

### How they work

- E2E tests live under `e2e/*.test.ts` and repro cases can live under `e2e/repros/*.test.ts`.
- Tests are usually run in a shell matrix (commonly `Bash`, `Zsh`, `Fish`, `PowerShell`, `WinCmd`) via `describe(shell, () => { ... })`.
- `script(shell)` builds a shell script step-by-step and executes it in an isolated temp directory.
- `shell.env({...})` injects fnm shell setup and feature flags.
- `shell.call("cmd", ["arg1", "arg2"])` appends command lines to the script.
- `shell.scriptOutputContains(...)` and `shell.hasCommandOutput(...)` are the main assertion helpers.
- The harness sets `FNM_NODE_DIST_MIRROR=http://localhost:8080` (proxy-backed), so tests should rely on existing mirrored behavior and avoid introducing unrelated network dependencies.

### Where to look

- Interface entrypoints:
  - `e2e/shellcode/script.ts` (`script(shell)`, execution model, env wiring)
  - `e2e/shellcode/shells.ts` (shell matrix definitions and capabilities)
  - `e2e/shellcode/shells/types.ts` (shared shell/script interfaces)
- Assertion helpers:
  - `e2e/shellcode/shells/output-contains.ts` (`shell.scriptOutputContains`)
  - `e2e/shellcode/shells/expect-command-output.ts` (`shell.hasCommandOutput`)
  - `e2e/shellcode/shells/cmdCall.ts` (`shell.call` command construction)
- Shell coverage behavior:
  - `e2e/describe.ts` (shell-specific `describe`, WinCmd skip behavior)
- Concrete test examples:
  - `e2e/basic.test.ts` (core install/use/version assertions)
  - `e2e/use-on-cd.test.ts` (directory-driven switching)
  - `e2e/exec.test.ts` (`fnm exec` scenarios)
  - `e2e/repros/issue-1527.test.ts` (issue repro pattern)

### How to build one

1. Start from an existing e2e test file and copy the style.
2. Keep the scenario deterministic:
   - prefer asserting concrete behavior (`found` vs `missing`, explicit versions, exact command output).
   - avoid shell-specific probes when a small Node script can validate behavior uniformly.
3. Use the shell matrix unless the behavior is explicitly shell-specific.
4. Keep setup minimal and local to the test (write only files required for the scenario).
5. Name tests by behavior, and for repros include the issue id in the test name.

### Example scenarios to copy

- Version resolution from files: `e2e/basic.test.ts` (`.nvmrc`, `.node-version`, `package.json` engines)
- Cross-shell behavior checks: `e2e/use-on-cd.test.ts`
- Feature flags via env: `e2e/corepack.test.ts` (`shell.env({ corepackEnabled: true })`)
- Repro that verifies state before and after a command: `e2e/repros/issue-1527.test.ts`

### Minimal template

```ts
import { script } from "./shellcode/script.js"
import { Bash, Fish, PowerShell, WinCmd, Zsh } from "./shellcode/shells.js"
import describe from "./describe.js"

for (const shell of [Bash, Zsh, Fish, PowerShell, WinCmd]) {
  describe(shell, () => {
    test("my behavior", async () => {
      await script(shell)
        .then(shell.env({}))
        .then(shell.call("fnm", ["install", "18"]))
        .then(shell.call("fnm", ["use", "18"]))
        .then(shell.scriptOutputContains(shell.call("node", ["-v"]), "v18"))
        .execute(shell)
    })
  })
}
```

### Running tests

- Single file: `pnpm test -- e2e/repros/issue-1527.test.ts`
- Full e2e suite: `pnpm test -- e2e`
