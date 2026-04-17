# fnm (fast node manager)

this project is focused on optimizations. the fact it is written in Rust is NOT the reason it's performant.
"what we do" and "what we do not do" are the main reasons for fnm's performance. we have a very specific scope and we have to do it well.

use tools like `hyperfine` to measure performance.
add end to end tests to avoid regressions and to prove that features actually work.

## End-to-end tests

E2E tests validate real-shell user behavior (`fnm env`, `fnm use`, PATH updates, global binaries, cross-version workflows).

- Tests: `e2e/*.test.ts`; repros: `e2e/repros/*.test.ts`.
- Use shell matrix with `describe(shell, () => { ... })` unless behavior is shell-specific.
- Build scripts with `script(shell)` and use `shell.env({...})`, `shell.call(...)`, `shell.scriptOutputContains(...)`, `shell.hasCommandOutput(...)`.
- Keep assertions deterministic (`found`/`missing`, explicit versions, exact outputs); prefer shell-agnostic checks.
- Write JavaScript code to disk and evaluate with Node.js if you need complex assertions as they are more maintainable than shell scripts.
- Keep setup local/minimal and include issue id in repro test names.
- Mirror is `FNM_NODE_DIST_MIRROR=http://localhost:8080`; avoid unrelated network dependencies.

- Interfaces: `e2e/shellcode/script.ts`, `e2e/shellcode/shells.ts`, `e2e/shellcode/shells/types.ts`.
- Helpers: `e2e/shellcode/shells/output-contains.ts`, `e2e/shellcode/shells/expect-command-output.ts`, `e2e/shellcode/shells/cmdCall.ts`.
- Shell coverage behavior: `e2e/describe.ts`.
- Examples: `e2e/basic.test.ts`, `e2e/use-on-cd.test.ts`, `e2e/exec.test.ts`, `e2e/corepack.test.ts`, `e2e/repros/issue-1527.test.ts`.

- Run one file: `pnpm test -- e2e/repros/issue-1527.test.ts`
- Run all e2e: `pnpm test -- e2e`

## Features, Bugfixes and Changes

Every code change requires a changeset in <repo_root>/.changeset/some-random-name.md
We should avoid breaking changes. Therefore 99.9% of changesets should be "patch" or "minor".
Be concise and clear in the changeset title, and provide examples or links to related issues if necessary.
See ./.changeset/EXAMPLE.md for an example of a changeset format.
