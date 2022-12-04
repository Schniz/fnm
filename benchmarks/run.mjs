// @ts-check

import z from "zod"
import os from "node:os"
import path from "node:path"
import fetch from "node-fetch"
import { execa } from "execa"
import { binary, command, flag, option } from "cmd-ts"
import Url from "cmd-ts/dist/cjs/batteries/url.js"
import { run } from "cmd-ts"
import fs from "node:fs/promises"
import { dedent } from "ts-dedent"

const HyperfineResult = z.object({
  results: z.array(
    z.object({
      command: z.string(),
      mean: z.number(),
      stddev: z.number(),
      median: z.number(),
      user: z.number(),
      system: z.number(),
      min: z.number(),
      max: z.number(),
      times: z.array(z.number()),
      exit_codes: z.array(z.literal(0)),
    })
  ),
})

const BenchyResult = z.object({
  data: z.object({
    embed: z.object({
      small: z.string(),
      big: z.string(),

      currentValue: z.number(),
      diff: z
        .object({
          lastValue: z.number(),
          value: z.number(),
          arrowImage: z.string(),
        })
        .optional(),
    }),
  }),
})

const { HttpUrl } = Url

const cmd = command({
  name: "run-benchmarks",
  args: {
    serverUrl: option({
      long: "server-url",
      type: HttpUrl,
      defaultValue: () => new URL("https://benchy.hagever.com"),
      defaultValueIsSerializable: true,
    }),
    githubToken: option({
      long: "github-token",
      env: "GITHUB_TOKEN",
    }),
    shouldStore: flag({
      long: "store",
    }),
  },
  async handler({ serverUrl, githubToken, shouldStore }) {
    const repoName = "fnm"
    const repoOwner = "schniz"

    const file = path.join(os.tmpdir(), `bench-${Date.now()}.json`)
    await execa(
      `hyperfine`,
      [
        `--export-json=${file}`,
        "--warmup=2",
        ...[
          "--command-name=fnm_basic",
          new URL("./basic/fnm", import.meta.url).pathname,
        ],
        // ...[
        //   "--command-name=nvm_basic",
        //   new URL("./basic/nvm", import.meta.url).pathname,
        // ],
      ],
      {
        stdout: process.stderr,
        stderr: process.stderr,
        stdin: "ignore",
      }
    )

    const json = JSON.parse(await fs.readFile(file, "utf8"))
    const parsed = HyperfineResult.safeParse(json)

    if (!parsed.success) {
      console.error(`Can't run benchmarks: wrong data:`, parsed.error.issues)
      process.exitCode = 1
      return
    }

    const { results } = parsed.data

    const url = new URL("/api/metrics", serverUrl)
    const trackedKeys = ["median", "max", "mean", "min"]

    const metrics = results.flatMap((result) => {
      return trackedKeys.map((key) => {
        return {
          key: `${result.command}/${key}`,
          value: result[key] * 1000, // everything is in seconds
        }
      })
    })

    const embeds$ = metrics.map(async ({ key, value }) => {
      const response = await fetch(String(url), {
        method: "PUT",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          coloring: "lower-is-better",
          repoOwner,
          repoName,
          githubToken,
          key,
          value,
        }),
      })

      if (!response.ok) {
        throw new Error(`Response is not okay: ${response.status}`)
      }

      const { data } = BenchyResult.parse(await response.json())
      return {
        key,
        ...data.embed,
      }
    })

    const embeds = await Promise.all(embeds$)

    const table = (() => {
      const rows = embeds
        .map((data) => {
          return dedent`
            <tr>
              <td><code>${data.key}</code></td>
              <td>${
                !data.diff
                  ? ""
                  : `<code>${round(data.diff.lastValue, 2)}</code>`
              }</td>
              <td><code>${round(data.currentValue, 2)}</code></td>
              <td>${
                !data.diff
                  ? ""
                  : dedent`
                  <picture title=${JSON.stringify(
                    data.diff.value > 0 ? "increase" : "decrease"
                  )}>
                    <img width="16" valign="middle" src="${
                      data.diff.arrowImage
                    }">
                  </picture>
                  <code>${data.diff.value > 0 ? "+" : ""}${round(
                      data.diff.value,
                      2
                    )}</code>
                    `
              }</td>
              <td>
                <details><summary><img valign="middle" src="${
                  data.small
                }" /></summary><img src="${data.big}" /></details>
              </td>
            </tr>
          `
        })
        .join("\n")
      return dedent`
        <table>
          <thead>
            <tr>
            <th align="left">benchmark</th>
            <th>last value</th>
            <th>current value</th>
            <th>diff</th>
            <th>trend</th>
            </tr>
          </thead>
          <tbody>
            ${rows}
          </tbody>
        </table>
      `
    })()

    console.log(table)

    if (shouldStore) {
      const response = await fetch(String(url), {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          repoOwner,
          repoName,
          githubToken,
          metrics,
        }),
      })

      if (!response.ok) {
        throw new Error(`Response is not okay: ${response.status}`)
      }

      console.error(await response.json())
    }
  },
})

/**
 * @param {number} number
 * @param {number} digits
 */
function round(number, digits) {
  const pow = Math.pow(10, digits)
  return Math.round(number * pow) / pow
}

run(binary(cmd), process.argv)
