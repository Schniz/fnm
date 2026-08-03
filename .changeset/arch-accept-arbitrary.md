---
"fnm": minor
---

Accept any user-provided `FNM_ARCH` / `--arch` value. The string is passed through to the download URL, allowing use of mirrors (such as `https://unofficial-builds.nodejs.org/download/release/`) that publish targets beyond Node's official set — for example `FNM_ARCH=linux-riscv64`. Well-known values (`x64`, `arm64`, …) keep their existing behaviour.
