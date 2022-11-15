import { mkdirSync } from "node:fs"
import { tmpdir } from "node:os"

export default function testTmpDir(): string {
  const testName = (expect.getState().currentTestName ?? "unknown")
    .toLowerCase()
    .replace(/[^a-z0-9]/gi, "_")
    .replace(/_+/g, "_")
  const tmpDir = `${tmpdir()}/shellcode/${testName}`
  mkdirSync(tmpDir, { recursive: true })
  return tmpDir
}
