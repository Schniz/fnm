import { mkdirSync } from "node:fs"
import path from "node:path"
import testTmpDir from "./test-tmp-dir.js"

export default function testBinDir() {
  const dir = path.join(testTmpDir(), "bin")
  mkdirSync(dir, { recursive: true })
  return dir
}
