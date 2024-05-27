// @ts-check

import { createServer } from "node:http"
import path from "node:path"
import fs from "node:fs"
import crypto from "node:crypto"
import fetch from "node-fetch"
import chalk from "chalk"

const baseDir = path.join(process.cwd(), ".proxy")
try {
  fs.mkdirSync(baseDir, { recursive: true })
} catch (e) {}

/** @type {Map<string, Promise<{ headers: Record<string, string>, body: ArrayBuffer }>>} */
const cache = new Map()

export const server = createServer((req, res) => {
  const pathname = req.url ?? "/"
  const hash = crypto
    .createHash("sha1")
    .update(pathname ?? "/")
    .digest("hex")
  const extension = path.extname(pathname)
  const filename = path.join(baseDir, hash) + extension
  const headersFilename = path.join(baseDir, hash) + ".headers.json"
  try {
    const headers = JSON.parse(fs.readFileSync(headersFilename, "utf-8"))
    const body = fs.createReadStream(filename)
    console.log(chalk.green.dim(`[proxy] hit: ${pathname} -> ${filename}`))
    res.writeHead(200, headers)
    body.pipe(res)
  } catch {
    let promise = cache.get(filename)
    if (!promise) {
      console.log(chalk.red.dim(`[proxy] miss: ${pathname} -> ${filename}`))
      promise = fetch(
        "https://nodejs.org/dist/" + pathname.replace(/^\/+/, ""),
        {
          compress: false,
        }
      ).then(async (response) => {
        const headers = Object.fromEntries(response.headers.entries())
        const body = await response.arrayBuffer()
        fs.writeFileSync(headersFilename, JSON.stringify(headers))
        fs.writeFileSync(filename, Buffer.from(body))
        return { headers, body }
      })
      cache.set(filename, promise)
      promise.finally(() => cache.delete(filename))
    }

    promise.then(
      ({ headers, body }) => {
        res.writeHead(200, headers)
        res.end(Buffer.from(body))
      },
      (err) => {
        console.error(err)
        res.writeHead(500)
        res.end()
      }
    )
  }
})
