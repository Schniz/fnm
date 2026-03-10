# fnm (fast node manager)

## End-to-end tests

### What they do

E2E tests validate user-facing fnm behavior inside real shells (`fnm env`, `fnm use`, PATH updates, global binaries, cross-version workflows).

### How they work

- Tests live in `e2e/*.test.ts`; issue repros in `e2e/repros/*.test.ts`.
- Most tests run a shell matrix via `describe(shell, () => { ... })`.
- `script(shell)` builds and runs a shell script in isolated temp dirs.
- Use `shell.env({...})`, `shell.call(...)`, `shell.scriptOutputContains(...)`, and `shell.hasCommandOutput(...)`.
- Harness uses `FNM_NODE_DIST_MIRROR=http://localhost:8080`; avoid unrelated network dependencies.

### Where to look

- Core interfaces: `e2e/shellcode/script.ts`, `e2e/shellcode/shells.ts`, `e2e/shellcode/shells/types.ts`.
- Assertions/helpers: `e2e/shellcode/shells/output-contains.ts`, `e2e/shellcode/shells/expect-command-output.ts`, `e2e/shellcode/shells/cmdCall.ts`.
- Shell coverage behavior: `e2e/describe.ts`.
- Example tests: `e2e/basic.test.ts`, `e2e/use-on-cd.test.ts`, `e2e/exec.test.ts`, `e2e/repros/issue-1527.test.ts`.

### How to build one

1. Copy an existing e2e test style.
2. Keep assertions deterministic (`found`/`missing`, explicit versions, exact outputs).
3. Prefer shell-agnostic checks (often a small Node script) over shell-specific probing.
4. Use shell matrix unless behavior is explicitly shell-specific.
5. Keep setup local/minimal and include issue id in repro test names.

### Example scenarios to copy

- Version resolution from files: `e2e/basic.test.ts` (`.nvmrc`, `.node-version`, `package.json` engines)
- Cross-shell behavior checks: `e2e/use-on-cd.test.ts`
- Feature flags via env: `e2e/corepack.test.ts` (`shell.env({ corepackEnabled: true })`)
- Repro that verifies state before and after a command: `e2e/repros/issue-1527.test.ts`

### Running tests

- Single file: `pnpm test -- e2e/repros/issue-1527.test.ts`
- Full e2e suite: `pnpm test -- e2e`
