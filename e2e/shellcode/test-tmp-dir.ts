import { mkdirSync } from "node:fs"
import { tmpdir } from "node:os"
import { join } from "node:path"

export default function testTmpDir(): string {
  const testName = (expect.getState().currentTestName ?? "unknown")
    .toLowerCase()
    .replace(/[^a-z0-9]/gi, "_")
    .replace(/_+/g, "_")
  const tmpDir = join(tmpdir(), `shellcode/${testName}`)
  mkdirSync(tmpDir, { recursive: true })

  return tmpDir
}
