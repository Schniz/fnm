// @ts-check

import { createServer } from "node:http"
import path from "node:path"
import fs from "node:fs"
import crypto from "node:crypto"
import fetch from "node-fetch"
import chalk from "chalk"
import pRetry from "p-retry"

const baseDir = path.join(process.cwd(), ".proxy")
try {
  fs.mkdirSync(baseDir, { recursive: true })
} catch (e) {}

/** @type {Map<string, Promise<{ headers: Record<string, string>, body: ArrayBuffer }>>} */
const cache = new Map()

/**
 * @param {object} opts
 * @param {string} opts.pathname
 * @param {string} opts.headersFilename
 * @param {string} opts.filename
 */
const download = async ({ pathname, filename, headersFilename }) => {
  const response = await fetch(
    "https://nodejs.org/dist/" + pathname.replace(/^\/+/, ""),
    {
      compress: false,
    },
  )
  const headers = Object.fromEntries(response.headers.entries())
  headers.__status__ = String(response.status)
  const body = await response.arrayBuffer()
  fs.writeFileSync(headersFilename, JSON.stringify(headers))
  fs.writeFileSync(filename, Buffer.from(body))
  return { headers, body }
}

/**
 * Test-only path prefixes that make the proxy misbehave like a broken mirror.
 * `/__test__/corrupt/` flips a byte in every archive,
 * `/__test__/no-signature/` has no `SHASUMS256.txt.asc`, and
 * `/__test__/bare/` has neither `SHASUMS256.txt` nor its signature.
 * @param {string} url
 */
const parseTestMode = (url) => {
  const match = url.match(/^\/__test__\/(corrupt|no-signature|bare)(\/.*)$/)
  return match
    ? { mode: match[1], pathname: match[2] }
    : { mode: null, pathname: url }
}

/** @param {string | null} mode @param {string} pathname */
const isMissingInMode = (mode, pathname) =>
  (mode === "no-signature" && pathname.endsWith("/SHASUMS256.txt.asc")) ||
  (mode === "bare" && /\/SHASUMS256\.txt(\.asc)?$/.test(pathname))

/** @param {string | null} mode @param {string} pathname */
const isCorruptedInMode = (mode, pathname) =>
  mode === "corrupt" && /\.(tar\.xz|tar\.gz|zip)$/.test(pathname)

/** @param {Buffer} body */
const corrupt = (body) => {
  const copy = Buffer.from(body)
  copy[copy.length >> 1] ^= 0xff
  return copy
}

export const server = createServer((req, res) => {
  const { mode, pathname } = parseTestMode(req.url ?? "/")
  if (isMissingInMode(mode, pathname)) {
    console.log(chalk.yellow.dim(`[proxy] ${mode}: 404 for ${pathname}`))
    res.writeHead(404)
    res.end()
    return
  }
  const shouldCorrupt = isCorruptedInMode(mode, pathname)
  const hash = crypto
    .createHash("sha1")
    .update(pathname ?? "/")
    .digest("hex")
  const extension = path.extname(pathname)
  const filename = path.join(baseDir, hash) + extension
  const headersFilename = path.join(baseDir, hash) + ".headers.json"
  try {
    const { __status__ = "200", ...headers } = JSON.parse(
      fs.readFileSync(headersFilename, "utf-8"),
    )
    const status = parseInt(__status__, 10)
    console.log(chalk.green.dim(`[proxy] hit: ${pathname} -> ${filename}`))
    if (shouldCorrupt) {
      const body = corrupt(fs.readFileSync(filename))
      res.writeHead(status, headers)
      res.end(body)
      return
    }
    const body = fs.createReadStream(filename)
    res.writeHead(status, headers)
    body.pipe(res)
  } catch {
    let promise = cache.get(filename)
    if (!promise) {
      console.log(chalk.red.dim(`[proxy] miss: ${pathname} -> ${filename}`))
      promise = pRetry(
        () => download({ pathname, headersFilename, filename }),
        {
          retries: 5,
          maxTimeout: 5000,
          onFailedAttempt: (error) => {
            console.error(
              chalk.red(
                `[proxy] ${chalk.bold("error")}: ${error.message}, retries left: ${error.retriesLeft}`,
              ),
            )
          },
        },
      )
      cache.set(filename, promise)
      promise.finally(() => cache.delete(filename))
    }

    promise.then(
      ({ headers: { __status__ = "200", ...headers }, body }) => {
        res.writeHead(parseInt(__status__, 10), headers)
        const buffer = Buffer.from(body)
        res.end(shouldCorrupt ? corrupt(buffer) : buffer)
      },
      (err) => {
        console.error(err)
        res.writeHead(500)
        res.end()
      },
    )
  }
})
