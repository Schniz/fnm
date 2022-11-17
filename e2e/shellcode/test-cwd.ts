import { mkdirSync } from "node:fs"
import path from "node:path"
import testTmpDir from "./test-tmp-dir.js"

export default function testCwd() {
  const dir = path.join(testTmpDir(), "cwd")
  mkdirSync(dir, { recursive: true })
  return dir
}
