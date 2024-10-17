## 1.31.0 (2022-02-16)

## 1.38.0

### Minor Changes

- [#1265](https://github.com/Schniz/fnm/pull/1265) [`186e4bb`](https://github.com/Schniz/fnm/commit/186e4bbd9204d7658a4f9923955281687a01e8c3) Thanks [@Schniz](https://github.com/Schniz)! - enable `--resolve-engines` by default. out of experimental phase.

  to disable it, add a `--resolve-engines=false` flag, and make sure to open an issue describing _why_.
  It might feel like a breaking change but .nvmrc and .node-version have precedence so it should not.

  I am all in favor of better experience and I believe supporting engines.node is a good direction.

### Patch Changes

- [#1277](https://github.com/Schniz/fnm/pull/1277) [`12cf977`](https://github.com/Schniz/fnm/commit/12cf977beb8581000ea84d7b8d1636dc3ae25db3) Thanks [@deanshub](https://github.com/deanshub)! - Having install and uninstall aliases

## 1.37.2

### Patch Changes

- [#1264](https://github.com/Schniz/fnm/pull/1264) [`364d2a9`](https://github.com/Schniz/fnm/commit/364d2a939c684fede25c43c6a9c14b7c4c6e3fc5) Thanks [@Schniz](https://github.com/Schniz)! - fix: allow to type powershell and power-shell as shell type inputs

- [#1195](https://github.com/Schniz/fnm/pull/1195) [`74d7c33`](https://github.com/Schniz/fnm/commit/74d7c33da9f5ac53a42633012d775015eb774bcb) Thanks [@mattmess1221](https://github.com/mattmess1221)! - When downloading from node-dist, Fallback to .tar.gz download when the .tar.xz download fails

- [#1259](https://github.com/Schniz/fnm/pull/1259) [`1000914`](https://github.com/Schniz/fnm/commit/10009143460e161b76350eb738652f332e8a893c) Thanks [@skirsten](https://github.com/skirsten)! - Fix `--resolve-engines` in combination with `--use-on-cd`

- [#1248](https://github.com/Schniz/fnm/pull/1248) [`e03d345`](https://github.com/Schniz/fnm/commit/e03d345e9f485c4ab0183a3a542bcba20525fff4) Thanks [@bburns](https://github.com/bburns)! - docs: add link for windows terminal startup script configuration

- [#1184](https://github.com/Schniz/fnm/pull/1184) [`fefdf69`](https://github.com/Schniz/fnm/commit/fefdf69870ddaec0ba59b103aef730dd03e56e4a) Thanks [@julescubtree](https://github.com/julescubtree)! - fix panic when list-remote --filter --latest has no results

- [#1255](https://github.com/Schniz/fnm/pull/1255) [`71c1c9a`](https://github.com/Schniz/fnm/commit/71c1c9a9998f7fd0dc512de6ead9244f03fa532d) Thanks [@stefanboca](https://github.com/stefanboca)! - set aliases during install, even if corepack is enabled

- [#1204](https://github.com/Schniz/fnm/pull/1204) [`19bffe5`](https://github.com/Schniz/fnm/commit/19bffe5af6ee1d624095c94f5ec55660fe69ac3b) Thanks [@JLarky](https://github.com/JLarky)! - Document how to use nightly builds

- [#1218](https://github.com/Schniz/fnm/pull/1218) [`345adaf`](https://github.com/Schniz/fnm/commit/345adafb856e2e0ba8feb8acf487a4669cea4055) Thanks [@Schniz](https://github.com/Schniz)! - internal: retry download in case of error in test proxy

- [#1226](https://github.com/Schniz/fnm/pull/1226) [`8e2d37d`](https://github.com/Schniz/fnm/commit/8e2d37d858b15d965d9644b27f5c111478cc59f2) Thanks [@mo8it](https://github.com/mo8it)! - performance optimizations, especially for `fnm env`

- [#1223](https://github.com/Schniz/fnm/pull/1223) [`2b211ef`](https://github.com/Schniz/fnm/commit/2b211ef64a7eb2ff5fe7ac017d1f963db522b6cf) Thanks [@mo8it](https://github.com/mo8it)! - Allow reading non-unicode paths from environment variables

## 1.37.1

### Patch Changes

- [#1164](https://github.com/Schniz/fnm/pull/1164) [`318f86d`](https://github.com/Schniz/fnm/commit/318f86d72938166cb2b3195cae37826f827b6c08) Thanks [@Schniz](https://github.com/Schniz)! - windows: fix shell inference in powershell when using scoop's shims

## 1.37.0

### Minor Changes

- [#1143](https://github.com/Schniz/fnm/pull/1143) [`f76a001`](https://github.com/Schniz/fnm/commit/f76a0011f7a613bb818e47acd224b7de48c5a81f) Thanks [@Schniz](https://github.com/Schniz)! - use XDG conventions in MacOS directories by default

### Patch Changes

- [#1148](https://github.com/Schniz/fnm/pull/1148) [`0b530cc`](https://github.com/Schniz/fnm/commit/0b530cc257e33e9801804b73ed831a1f28489b57) Thanks [@Schniz](https://github.com/Schniz)! - fix ordering in ls-remote

- [#1133](https://github.com/Schniz/fnm/pull/1133) [`a1afe84`](https://github.com/Schniz/fnm/commit/a1afe8436afb29960b71e14e221441398f6a48ca) Thanks [@Schniz](https://github.com/Schniz)! - upgrade all dependencies & maintain lockfile

## 1.36.0

### Minor Changes

- [#1063](https://github.com/Schniz/fnm/pull/1063) [`121128f`](https://github.com/Schniz/fnm/commit/121128f42bc27583c2cf9a77b14f60f456af4a82) Thanks [@ryanccn](https://github.com/ryanccn)! - feat: add remote version sorting and filtering

- [#896](https://github.com/Schniz/fnm/pull/896) [`7b47b3e`](https://github.com/Schniz/fnm/commit/7b47b3ef0a741fe40a39efaa5eca6945b2e11c59) Thanks [@pedrofialho](https://github.com/pedrofialho)! - `fnm install latest` will now tag the `latest` alias

- [#1028](https://github.com/Schniz/fnm/pull/1028) [`66efc5b`](https://github.com/Schniz/fnm/commit/66efc5b90c71f2f24592dbf8d7e28c9f53bcf5f9) Thanks [@eblocha](https://github.com/eblocha)! - Show a progress bar when downloading and extracting node

### Patch Changes

- [#1014](https://github.com/Schniz/fnm/pull/1014) [`6be23c8`](https://github.com/Schniz/fnm/commit/6be23c868f5ddb2a4508a1f302c345298401d674) Thanks [@tottoto](https://github.com/tottoto)! - Disable unused chrono features (#1014)

- [#1050](https://github.com/Schniz/fnm/pull/1050) [`d6c132a`](https://github.com/Schniz/fnm/commit/d6c132adfd1c29c48acb0b9de42538146e23cf18) Thanks [@JakeHandsome](https://github.com/JakeHandsome)! - Fix `cd /D` on windows with `--use-on-cd`

- [#1109](https://github.com/Schniz/fnm/pull/1109) [`8f3acbb`](https://github.com/Schniz/fnm/commit/8f3acbb8ee4991b838ce3fe146d62864aaa290b9) Thanks [@Vinfall](https://github.com/Vinfall)! - support `x64-musl` arch by adding a `--arch x64-musl` to fnm env

- [#1125](https://github.com/Schniz/fnm/pull/1125) [`d9af62f`](https://github.com/Schniz/fnm/commit/d9af62ff432e57398efdd812e218f3fb390c5243) Thanks [@Schniz](https://github.com/Schniz)! - make nicer styling in progress bar (add newline, make it unicode)

- [#1058](https://github.com/Schniz/fnm/pull/1058) [`734df47`](https://github.com/Schniz/fnm/commit/734df47795c3b71d104ec9638034447cc4ef47cc) Thanks [@aquacash5](https://github.com/aquacash5)! - fix: return default version if canonicalize fails

- [#1066](https://github.com/Schniz/fnm/pull/1066) [`9ff98da`](https://github.com/Schniz/fnm/commit/9ff98da93c69013274322c3a63dfdb1beaf73875) Thanks [@floh1695](https://github.com/floh1695)! - Fixes a bug when running `eval $(fnm env)` in sh when there a spaces in the $PATH

## 1.35.1

### Patch Changes

- [#1010](https://github.com/Schniz/fnm/pull/1010) [`b185446`](https://github.com/Schniz/fnm/commit/b185446cfa08d4d6e2552db53937762a3c38d4db) Thanks [@quixoten](https://github.com/quixoten)! - fix: panic on `fnm completions`

## 1.35.0

### Minor Changes

- [#839](https://github.com/Schniz/fnm/pull/839) [`97be792`](https://github.com/Schniz/fnm/commit/97be792a4410d8f121e03a1f81f60c48cbfdee2c) Thanks [@amitdahan](https://github.com/amitdahan)! - Support resolving `engines.node` field via experimental `--resolve-engines` flag

### Patch Changes

- [#991](https://github.com/Schniz/fnm/pull/991) [`b19eb29`](https://github.com/Schniz/fnm/commit/b19eb29b26323f0b9fb427d3d9271c9cc13a58f8) Thanks [@amitdahan](https://github.com/amitdahan)! - Bump Clap 3 -> 4

## 1.34.0

### Minor Changes

- Add --corepack-enabled flag for automatically enabling corepack on fnm install ([#960](https://github.com/Schniz/fnm/pull/960))

### Patch Changes

- modernize tty check (#973 by @tottoto) ([`48b2611`](https://github.com/Schniz/fnm/commit/48b2611e4b1c205f07dcbd50f2fff436becb77c1))

- use cygwinpath to make the path posix-like on Windows Bash usage ([#960](https://github.com/Schniz/fnm/pull/960))

- capitalize "n" to show default (#963 by @Joshuahuahua) ([`48b2611`](https://github.com/Schniz/fnm/commit/48b2611e4b1c205f07dcbd50f2fff436becb77c1))

## 1.33.1

### Patch Changes

- remove upx compression from arm binaries because it fails to build on latest rust ([#875](https://github.com/Schniz/fnm/pull/875))

## 1.33.0

### Minor Changes

- Replace `semver` with `node_semver` ([#816](https://github.com/Schniz/fnm/pull/816))

- support `fnm install --latest` to install the latest Node.js version ([#859](https://github.com/Schniz/fnm/pull/859))

## 1.32.0

### Minor Changes

- Add `--json` to `fnm env` to output the env vars as JSON ([#800](https://github.com/Schniz/fnm/pull/800))

### Patch Changes

- This updates the Changesets configurations. ([#783](https://github.com/Schniz/fnm/pull/783))

- fix test: Use correct PATH for npm install test ([#768](https://github.com/Schniz/fnm/pull/768))

- Make installation script respect `$XDG_DATA_HOME` ([#614](https://github.com/Schniz/fnm/pull/614))

## 1.31.1

### Patch Changes

- 6e6bdd8: Add changesets

#### Bugfix 🐛

- [#671](https://github.com/Schniz/fnm/pull/671) fix native-creds errors (using the unpublished version of reqwest) ([@Schniz](https://github.com/Schniz))
- [#663](https://github.com/Schniz/fnm/pull/663) remove .unwrap() and .expect() in shell code ([@Schniz](https://github.com/Schniz))
- [#656](https://github.com/Schniz/fnm/pull/656) Remove unwrap in fnm env, make error more readable ([@Schniz](https://github.com/Schniz))

#### Committers: 1

- Gal Schlezinger ([@Schniz](https://github.com/Schniz))

## v1.30.1 (2022-01-30)

#### Bugfix 🐛

- [#652](https://github.com/Schniz/fnm/pull/652) Fix `fnm completions` ([@Schniz](https://github.com/Schniz))

#### Committers: 1

- Gal Schlezinger ([@Schniz](https://github.com/Schniz))

## v1.30.0 (2022-01-30)

#### Bugfix 🐛

- [#625](https://github.com/Schniz/fnm/pull/625) Revert from using sysinfo, but only on Unix machines ([@Schniz](https://github.com/Schniz))
- [#638](https://github.com/Schniz/fnm/pull/638) Use local cache directory instead of temp directory for symlinks ([@Schniz](https://github.com/Schniz))

#### Internal 🛠

- [#617](https://github.com/Schniz/fnm/pull/617) fix(deps): update rust crate clap to v3 ([@renovate[bot]](https://github.com/apps/renovate))
- [#630](https://github.com/Schniz/fnm/pull/630) Replace `snafu` with `thiserror` ([@Schniz](https://github.com/Schniz))

#### Documentation 📝

- [#637](https://github.com/Schniz/fnm/pull/637) [Documentation] Adds Additional info regarding .node-version ([@uchihamalolan](https://github.com/uchihamalolan))

#### Committers: 2

- Gal Schlezinger ([@Schniz](https://github.com/Schniz))
- Malolan B ([@uchihamalolan](https://github.com/uchihamalolan))

## v1.29.2 (2022-01-06)

#### Bugfix 🐛

- [#626](https://github.com/Schniz/fnm/pull/626) Create the multishells directory if it is missing ([@Schniz](https://github.com/Schniz))

#### Documentation 📝

- [#598](https://github.com/Schniz/fnm/pull/598) Update README.md file emoji ([@lorensr](https://github.com/lorensr))
- [#616](https://github.com/Schniz/fnm/pull/616) Use on cd by default ([@matthieubosquet](https://github.com/matthieubosquet))

#### Committers: 3

- Gal Schlezinger ([@Schniz](https://github.com/Schniz))
- Loren ☺️ ([@lorensr](https://github.com/lorensr))
- Matthieu Bosquet ([@matthieubosquet](https://github.com/matthieubosquet))

## v1.29.1 (2021-12-28)

#### Bugfix 🐛

- [#613](https://github.com/Schniz/fnm/pull/613) Don't warn on `~/.fnm` yet ([@Schniz](https://github.com/Schniz))

#### Committers: 1

- Gal Schlezinger ([@Schniz](https://github.com/Schniz))

## v1.29.0 (2021-12-28)

#### New Feature 🎉

- [#607](https://github.com/Schniz/fnm/pull/607) Allow recursive version lookups ([@Schniz](https://github.com/Schniz))
- [#416](https://github.com/Schniz/fnm/pull/416) Respect \$XDG_DATA_HOME ([@samhh](https://github.com/samhh))

#### Bugfix 🐛

- [#603](https://github.com/Schniz/fnm/pull/603) (re)Include feature for (native) certificates ([@pfiaux](https://github.com/pfiaux))
- [#605](https://github.com/Schniz/fnm/pull/605) Add a user-agent header ([@Schniz](https://github.com/Schniz))

#### Internal 🛠

- [#606](https://github.com/Schniz/fnm/pull/606) Migrate to Rust 2021 edition ([@Schniz](https://github.com/Schniz))

#### Committers: 3

- Gal Schlezinger ([@Schniz](https://github.com/Schniz))
- Patrick Fiaux ([@pfiaux](https://github.com/pfiaux))
- Sam A. Horvath-Hunt ([@samhh](https://github.com/samhh))

## v1.28.2 (2021-12-05)

#### Bugfix 🐛

- [#586](https://github.com/Schniz/fnm/pull/586) Revert the change to ureq ([@Schniz](https://github.com/Schniz))

#### Documentation 📝

- [#584](https://github.com/Schniz/fnm/pull/584) Add warning to symlink creation ([@Schniz](https://github.com/Schniz))

#### Committers: 1

- Gal Schlezinger ([@Schniz](https://github.com/Schniz))

## v1.28.1 (2021-11-17)

#### Bugfix 🐛

- [#576](https://github.com/Schniz/fnm/pull/576) First fix for slowess in fnm env ([@Schniz](https://github.com/Schniz))

#### Committers: 1

- Gal Schlezinger ([@Schniz](https://github.com/Schniz))

## v1.28.0 (2021-11-16)

#### New Feature 🎉

- [#556](https://github.com/Schniz/fnm/pull/556) Allow aliasing to the system version ([@Schniz](https://github.com/Schniz))
- [#547](https://github.com/Schniz/fnm/pull/547) Replace reqwest with ureq for less dependencies and smaller file size ([@dnaka91](https://github.com/dnaka91))

#### Bugfix 🐛

- [#573](https://github.com/Schniz/fnm/pull/573) Infer shell with `sysinfo` crate ([@Schniz](https://github.com/Schniz))
- [#553](https://github.com/Schniz/fnm/pull/553) Fix Windows CMD `cd` failure on paths with spaces (`use-on-cd`) ([@Schniz](https://github.com/Schniz))

#### Internal 🛠

- [#554](https://github.com/Schniz/fnm/pull/554) Fix clippy & use musl target on Rust compiler for static compilation ([@dnaka91](https://github.com/dnaka91))

#### Documentation 📝

- [#545](https://github.com/Schniz/fnm/pull/545) docs(cli): Fix typo ([@SanchithHegde](https://github.com/SanchithHegde))
- [#544](https://github.com/Schniz/fnm/pull/544) Replace error message with a more useful one ([@yonifra](https://github.com/yonifra))
- [#537](https://github.com/Schniz/fnm/pull/537) Show log level options in help and error message ([@lucasweng](https://github.com/lucasweng))

#### Committers: 5

- Dominik Nakamura ([@dnaka91](https://github.com/dnaka91))
- Gal Schlezinger ([@Schniz](https://github.com/Schniz))
- Jonathan Fraimorice ([@yonifra](https://github.com/yonifra))
- Lucas Weng ([@lucasweng](https://github.com/lucasweng))
- Sanchith Hegde ([@SanchithHegde](https://github.com/SanchithHegde))

## v1.27.0 (2021-09-17)

#### New Feature 🎉

- [#519](https://github.com/Schniz/fnm/pull/519) Windows: Use junctions rather than symlinks ([@davidaurelio](https://github.com/davidaurelio))
- [#489](https://github.com/Schniz/fnm/pull/489) Add unalias command ([@AlexMunoz](https://github.com/AlexMunoz))

#### Bugfix 🐛

- [#528](https://github.com/Schniz/fnm/pull/528) installation script: Use `=` instead of `==` ([@develoot](https://github.com/develoot))
- [#512](https://github.com/Schniz/fnm/pull/512) fix(dep): Revert "Include feature for (native) certificates #468" ([@itotallyrock](https://github.com/itotallyrock))
- [#514](https://github.com/Schniz/fnm/pull/514) Invoke fnm use on startup for PowerShell ([@naoey](https://github.com/naoey))

#### Documentation 📝

- [#529](https://github.com/Schniz/fnm/pull/529) Add Chocolatey installation instructions ([@CMeeg](https://github.com/CMeeg))
- [#496](https://github.com/Schniz/fnm/pull/496) install.sh: print an exit message for missing deps ([@waldyrious](https://github.com/waldyrious))
- [#516](https://github.com/Schniz/fnm/pull/516) More informative log level error message ([@waldyrious](https://github.com/waldyrious))
- [#493](https://github.com/Schniz/fnm/pull/493) Add instructions to setup the file for Cmder ([@Lunchb0ne](https://github.com/Lunchb0ne))

#### Committers: 8

- Abhishek Aryan ([@Lunchb0ne](https://github.com/Lunchb0ne))
- Alex Munoz ([@AlexMunoz](https://github.com/AlexMunoz))
- Chris Meagher ([@CMeeg](https://github.com/CMeeg))
- David Aurelio ([@davidaurelio](https://github.com/davidaurelio))
- Jeffrey Meyer ([@itotallyrock](https://github.com/itotallyrock))
- Kitae Kim ([@develoot](https://github.com/develoot))
- Waldir Pimenta ([@waldyrious](https://github.com/waldyrious))
- [@naoey](https://github.com/naoey)

## v1.26.0 (2021-07-15)

#### Bugfix 🐛

- [#484](https://github.com/Schniz/fnm/pull/484) fix: --install-if-missing when using version alias ([@AlexMunoz](https://github.com/AlexMunoz))
- [#468](https://github.com/Schniz/fnm/pull/468) Include feature for (native) certificates ([@pfiaux](https://github.com/pfiaux))

#### Documentation 📝

- [#467](https://github.com/Schniz/fnm/pull/467) Add instructions for removing fnm ([@binyamin](https://github.com/binyamin))
- [#461](https://github.com/Schniz/fnm/pull/461) Fix missing list-remote in command doc ([@binhonglee](https://github.com/binhonglee))

#### Committers: 4

- Alex Munoz ([@AlexMunoz](https://github.com/AlexMunoz))
- BinHong Lee ([@binhonglee](https://github.com/binhonglee))
- Binyamin Aron Green ([@binyamin](https://github.com/binyamin))
- Patrick Fiaux ([@pfiaux](https://github.com/pfiaux))

## v1.25.0 (2021-05-17)

#### New Feature 🎉

- [#436](https://github.com/Schniz/fnm/pull/436) feat: use arm for aarch64-darwin-node@16 platforms ([@pckilgore](https://github.com/pckilgore))

#### Documentation 📝

- [#432](https://github.com/Schniz/fnm/pull/432) Auto-generate command documentation markdown ([@Schniz](https://github.com/Schniz))

#### Committers: 2

- Gal Schlezinger ([@Schniz](https://github.com/Schniz))
- Patrick Kilgore ([@pckilgore](https://github.com/pckilgore))

## v1.24.0 (2021-04-02)

#### New Feature 🎉

- [#421](https://github.com/Schniz/fnm/pull/421) Adding FNM_ARCH as an exported env var from `fnm env` ([@Schniz](https://github.com/Schniz))
- [#417](https://github.com/Schniz/fnm/pull/417) Support Apple M1 by installing Rosetta Node builds ([@pckilgore](https://github.com/pckilgore))

#### Bugfix 🐛

- [#422](https://github.com/Schniz/fnm/pull/422) Create version symlinks in a nested directory ([@Schniz](https://github.com/Schniz))

#### Committers: 2

- Gal Schlezinger ([@Schniz](https://github.com/Schniz))
- Patrick Kilgore ([@pckilgore](https://github.com/pckilgore))

## v1.23.2 (2021-03-24)

#### Bugfix 🐛

- [#413](https://github.com/Schniz/fnm/pull/413) Improve "version not found" error message ([@waldyrious](https://github.com/waldyrious))
- [#414](https://github.com/Schniz/fnm/pull/414) More meaningful error message when a subprocess is killed ([@Schniz](https://github.com/Schniz))

#### Committers: 2

- Gal Schlezinger ([@Schniz](https://github.com/Schniz))
- Waldir Pimenta ([@waldyrious](https://github.com/waldyrious))

## v1.23.1 (2021-03-21)

#### Bugfix 🐛

- [#411](https://github.com/Schniz/fnm/pull/411) Call the fnm use-on-cd hook on fish initialization ([@Schniz](https://github.com/Schniz))
- [#404](https://github.com/Schniz/fnm/pull/404) Ignore LogLevel on `fnm ls` ([@Schniz](https://github.com/Schniz))

#### Internal 🛠

- [#408](https://github.com/Schniz/fnm/pull/408) Fix ARM builds CI ([@Schniz](https://github.com/Schniz))

#### Documentation 📝

- [#405](https://github.com/Schniz/fnm/pull/405) Fix a couple typos ([@waldyrious](https://github.com/waldyrious))

#### Committers: 2

- Gal Schlezinger ([@Schniz](https://github.com/Schniz))
- Waldir Pimenta ([@waldyrious](https://github.com/waldyrious))

## v1.23.0 (2021-03-02)

#### New Feature 🎉

- [#403](https://github.com/Schniz/fnm/pull/403) Allow `fnm use` to use a file path ([@Schniz](https://github.com/Schniz))
- [#401](https://github.com/Schniz/fnm/pull/401) Allow using filenames in --using= ([@Schniz](https://github.com/Schniz))

#### Bugfix 🐛

- [#398](https://github.com/Schniz/fnm/pull/398) `fnm exec`: Don't require double-dash ([@Schniz](https://github.com/Schniz))
- [#389](https://github.com/Schniz/fnm/pull/389) Chore - Use a tier 1 target for Linux binary ([@kaioduarte](https://github.com/kaioduarte))
- [#384](https://github.com/Schniz/fnm/pull/384) Do not list hidden directories as installed versions ([@scadu](https://github.com/scadu))

#### Documentation 📝

- [#397](https://github.com/Schniz/fnm/pull/397) Fix minor typos ([@leafrogers](https://github.com/leafrogers))
- [#385](https://github.com/Schniz/fnm/pull/385) README: add symlink support on Windows ([@scadu](https://github.com/scadu))
- [#377](https://github.com/Schniz/fnm/pull/377) Improvements to the README ([@waldyrious](https://github.com/waldyrious))

#### Committers: 5

- Gal Schlezinger ([@Schniz](https://github.com/Schniz))
- Kaio Duarte ([@kaioduarte](https://github.com/kaioduarte))
- Leaf Rogers ([@leafrogers](https://github.com/leafrogers))
- Waldir Pimenta ([@waldyrious](https://github.com/waldyrious))
- Łukasz Jendrysik ([@scadu](https://github.com/scadu))

## v1.22.9 (2021-01-22)

#### Bugfix 🐛

- [#368](https://github.com/Schniz/fnm/pull/368) Update 'ring' to '0.16.17' ([@gucheen](https://github.com/gucheen))
- [#347](https://github.com/Schniz/fnm/pull/347) Check if \$ZDOTDIR exists to find correct path to .zshrc ([@thales-maciel](https://github.com/thales-maciel))

#### Documentation 📝

- [#341](https://github.com/Schniz/fnm/pull/341) Update installation instructions ([@Schniz](https://github.com/Schniz))
- [#329](https://github.com/Schniz/fnm/pull/329) Add scoop support ([@Armaldio](https://github.com/Armaldio))
- [#334](https://github.com/Schniz/fnm/pull/334) Add installation instructions from cargo ([@zhmushan](https://github.com/zhmushan))

#### Committers: 5

- Cheng Gu ([@gucheen](https://github.com/gucheen))
- Gal Schlezinger ([@Schniz](https://github.com/Schniz))
- Quentin Goinaud ([@Armaldio](https://github.com/Armaldio))
- Thales Maciel ([@thales-maciel](https://github.com/thales-maciel))
- 木杉 ([@zhmushan](https://github.com/zhmushan))

## v1.22.8 (2020-11-10)

#### Documentation 📝

- [#327](https://github.com/Schniz/fnm/pull/327) Make ls and ls-remote aliases visible ([@Schniz](https://github.com/Schniz))

#### Committers: 1

- Gal Schlezinger ([@Schniz](https://github.com/Schniz))

## v1.22.7 (2020-11-09)

#### New Feature 🎉

- [#315](https://github.com/Schniz/fnm/pull/315) Add `list` alias for `ls` ([@probablykasper](https://github.com/probablykasper))

#### Bugfix 🐛

- [#326](https://github.com/Schniz/fnm/pull/326) Make config arguments globally available on all commands ([@Schniz](https://github.com/Schniz))
- [#323](https://github.com/Schniz/fnm/pull/323) Escape `cd` calls in Bash's use-on-cd ([@Schniz](https://github.com/Schniz))
- [#322](https://github.com/Schniz/fnm/pull/322) Run `fnm use` on shell initialization in Bash ([@Schniz](https://github.com/Schniz))

#### Committers: 2

- Gal Schlezinger ([@Schniz](https://github.com/Schniz))
- Kasper ([@probablykasper](https://github.com/probablykasper))

## v1.22.6 (2020-11-04)

#### Bugfix 🐛

- [#317](https://github.com/Schniz/fnm/pull/317) Bring back --using-file flag with a deprecation warning. ([@bjornua](https://github.com/bjornua))

#### Committers: 1

- Bjørn Arnholtz ([@bjornua](https://github.com/bjornua))

## v1.22.5 (2020-10-29)

#### Bugfix 🐛

- [#310](https://github.com/Schniz/fnm/pull/310) Better error handling in symlink replacement ([@Schniz](https://github.com/Schniz))

#### Documentation 📝

- [#307](https://github.com/Schniz/fnm/pull/307) Refer to homebrew/core formula instead of custom tap ([@jameschensmith](https://github.com/jameschensmith))

#### Committers: 2

- Gal Schlezinger ([@Schniz](https://github.com/Schniz))
- James Chen-Smith ([@jameschensmith](https://github.com/jameschensmith))

## v1.22.4 (2020-10-28)

#### New Feature 🎉

- [#304](https://github.com/Schniz/fnm/pull/304) Make the first installed version the default one ([@Schniz](https://github.com/Schniz))

#### Bugfix 🐛

- [#308](https://github.com/Schniz/fnm/pull/308) Allow unsuccessful symlink deletion in `fnm use` ([@Schniz](https://github.com/Schniz))
- [#303](https://github.com/Schniz/fnm/pull/303) Add ARM handling to installation script ([@Schniz](https://github.com/Schniz))
- [#289](https://github.com/Schniz/fnm/pull/289) Remove logs from automatic version switching in `--use-on-cd` ([@maxjacobson](https://github.com/maxjacobson))
- [#301](https://github.com/Schniz/fnm/pull/301) UserVersion: Fix different parsing for leading `v` ([@Schniz](https://github.com/Schniz))

#### Documentation 📝

- [#306](https://github.com/Schniz/fnm/pull/306) Throw an error instead of panicking when can't infer shell ([@Schniz](https://github.com/Schniz))
- [#297](https://github.com/Schniz/fnm/pull/297) Update CI badge ([@jameschensmith](https://github.com/jameschensmith))

#### Committers: 3

- Gal Schlezinger ([@Schniz](https://github.com/Schniz))
- James Chen-Smith ([@jameschensmith](https://github.com/jameschensmith))
- Max Jacobson ([@maxjacobson](https://github.com/maxjacobson))

## v1.22.3 (2020-10-26)

#### New Feature 🎉

- [#292](https://github.com/Schniz/fnm/pull/292) Add uninstall command ([@Schniz](https://github.com/Schniz))
- [#276](https://github.com/Schniz/fnm/pull/276) Add pre-built binaries for ARM32 and ARM64 ([@Schniz](https://github.com/Schniz))

#### Bugfix 🐛

- [#290](https://github.com/Schniz/fnm/pull/290) Fix shell inference in VSCode ([@Schniz](https://github.com/Schniz))

#### Internal 🛠

- [#287](https://github.com/Schniz/fnm/pull/287) Remove `--multi` from install script ([@jameschensmith](https://github.com/jameschensmith))

#### Documentation 📝

- [#295](https://github.com/Schniz/fnm/pull/295) Add a warning if MULTISHELL env var is not in PATH ([@Schniz](https://github.com/Schniz))
- [#293](https://github.com/Schniz/fnm/pull/293) Add detailed error instead of panic regarding `fnm env` ([@Schniz](https://github.com/Schniz))

#### Committers: 2

- Gal Schlezinger ([@Schniz](https://github.com/Schniz))
- James Chen-Smith ([@jameschensmith](https://github.com/jameschensmith))

## v1.22.2 (2020-10-25)

#### Bugfix 🐛

- [#284](https://github.com/Schniz/fnm/pull/284) Fix npm not working because of wrong file copying ([@Schniz](https://github.com/Schniz))

#### Documentation 📝

- [#282](https://github.com/Schniz/fnm/pull/282) site: proxy the installation script ([@Schniz](https://github.com/Schniz))
- [#271](https://github.com/Schniz/fnm/pull/271) Update README for Windows instructions ([@Schniz](https://github.com/Schniz))
- [#281](https://github.com/Schniz/fnm/pull/281) docs: removed mentions of --multi from README ([@folke](https://github.com/folke))

#### Committers: 2

- Folke Lemaitre ([@folke](https://github.com/folke))
- Gal Schlezinger ([@Schniz](https://github.com/Schniz))

## v1.22.1 (2020-10-25)

#### Internal 🛠

- [#279](https://github.com/Schniz/fnm/pull/279) Remove UPX binary compression for now ([@Schniz](https://github.com/Schniz))

#### Committers: 1

- Gal Schlezinger ([@Schniz](https://github.com/Schniz))

## v1.22.0 (2020-10-25)

#### Bugfix 🐛

- [#275](https://github.com/Schniz/fnm/pull/275) Fix moving a node installation across mounting points ([@jaythomas](https://github.com/jaythomas))

#### Internal 🛠

- [#274](https://github.com/Schniz/fnm/pull/274) Add ARM arch support in downloads ([@jaythomas](https://github.com/jaythomas))
- [#266](https://github.com/Schniz/fnm/pull/266) Use separate config file for fish config ([@wesbaker](https://github.com/wesbaker))

#### Committers: 3

- Gal Schlezinger ([@Schniz](https://github.com/Schniz))
- Jay Thomas ([@jaythomas](https://github.com/jaythomas))
- Wes Baker ([@wesbaker](https://github.com/wesbaker))

## v1.22.0-beta-1 (2020-10-07)

#### New Feature 🎉

- [#244](https://github.com/Schniz/fnm/pull/244) Allow using homebrew to install with the installation script ([@Schniz](https://github.com/Schniz))

#### Bugfix 🐛

- [#225](https://github.com/Schniz/fnm/pull/225) Remove unused condition ([@joliss](https://github.com/joliss))

#### Internal 🛠

- [#246](https://github.com/Schniz/fnm/pull/246) Rewrite fnm in Rust (merge fnm.rs into fnm) — adding Windows support! ([@Schniz](https://github.com/Schniz))
- [#243](https://github.com/Schniz/fnm/pull/243) Add installation script testing ([@Schniz](https://github.com/Schniz))

#### Documentation 📝

- [#247](https://github.com/Schniz/fnm/pull/247) fixed a typo ([@0xflotus](https://github.com/0xflotus))
- [#245](https://github.com/Schniz/fnm/pull/245) Shorten the installation script ([@Schniz](https://github.com/Schniz))
- [#237](https://github.com/Schniz/fnm/pull/237) docs: add explanation of uninstall command to README ([@kazushisan](https://github.com/kazushisan))
- [#235](https://github.com/Schniz/fnm/pull/235) Mention fnm omf plugin for Fish users ([@idkjs](https://github.com/idkjs))

#### Committers: 5

- 0xflotus ([@0xflotus](https://github.com/0xflotus))
- Alain Armand ([@idkjs](https://github.com/idkjs))
- Gal Schlezinger ([@Schniz](https://github.com/Schniz))
- Jo Liss ([@joliss](https://github.com/joliss))
- Kazushi Konosu ([@kazushisan](https://github.com/kazushisan))

## v1.21.0 (2020-06-07)

#### New Feature 🎉

- [#220](https://github.com/Schniz/fnm/pull/220) Add `current` command to retrieve the current Node version ([@vladimyr](https://github.com/vladimyr))
- [#210](https://github.com/Schniz/fnm/pull/210) Allow aliasing and removing an alias ([@Schniz](https://github.com/Schniz))

#### Bugfix 🐛

- [#217](https://github.com/Schniz/fnm/pull/217) Set version on new shell initialization in fish ([@jaredramirez](https://github.com/jaredramirez))
- [#207](https://github.com/Schniz/fnm/pull/207) Format dotfiles versions before comparison ([@amitdahan](https://github.com/amitdahan))

#### Committers: 4

- Amit Dahan ([@amitdahan](https://github.com/amitdahan))
- Dario Vladović ([@vladimyr](https://github.com/vladimyr))
- Gal Schlezinger ([@Schniz](https://github.com/Schniz))
- Jared Ramirez ([@jaredramirez](https://github.com/jaredramirez))

## v1.20.0 (2020-03-22)

#### New Feature 🎉

- [#201](https://github.com/Schniz/fnm/pull/201) Create alias from version range (Closes [#185](https://github.com/Schniz/fnm/issues/185)) ([@tatchi](https://github.com/tatchi))

#### Bugfix 🐛

- [#199](https://github.com/Schniz/fnm/pull/199) Fix typo criterias -> criteria ([@waldyrious](https://github.com/waldyrious))

#### Documentation 📝

- [#204](https://github.com/Schniz/fnm/pull/204) Improve eval expression in `README.md` ([@loliee](https://github.com/loliee))

#### Committers: 3

- Corentin Leruth ([@tatchi](https://github.com/tatchi))
- Maxime Loliée ([@loliee](https://github.com/loliee))
- Waldir Pimenta ([@waldyrious](https://github.com/waldyrious))

## v1.19.0 (2020-03-09)

#### New Feature 🎉

- [#194](https://github.com/Schniz/fnm/pull/194) Add `fnm exec` to run commands with the fnm environment ([@Schniz](https://github.com/Schniz))
- [#191](https://github.com/Schniz/fnm/pull/191) uninstall allow prefix (Closes [#121](https://github.com/Schniz/fnm/issues/121)) ([@tatchi](https://github.com/tatchi))
- [#190](https://github.com/Schniz/fnm/pull/190) Add majorVersion option to ls-remote ([@tatchi](https://github.com/tatchi))

#### Bugfix 🐛

- [#200](https://github.com/Schniz/fnm/pull/200) Do not repeat installation prompt for unrecognized versions ([@tatchi](https://github.com/tatchi))
- [#197](https://github.com/Schniz/fnm/pull/197) Fix capitalization of message in Use.re ([@waldyrious](https://github.com/waldyrious))

#### Internal 🛠

- [#202](https://github.com/Schniz/fnm/pull/202) Update rely ([@tatchi](https://github.com/tatchi))
- [#192](https://github.com/Schniz/fnm/pull/192) Bump ocaml to 4.08 ([@tatchi](https://github.com/tatchi))

#### Committers: 3

- Corentin Leruth ([@tatchi](https://github.com/tatchi))
- Gal Schlezinger ([@Schniz](https://github.com/Schniz))
- Waldir Pimenta ([@waldyrious](https://github.com/waldyrious))

## v1.18.1 (2019-12-30)

#### Bugfix 🐛

- [#182](https://github.com/Schniz/fnm/pull/182) Closes [#181](https://github.com/Schniz/fnm/issues/181) by dropping `exit(1)` ([@Schniz](https://github.com/Schniz))

#### Committers: 1

- Gal Schlezinger ([@Schniz](https://github.com/Schniz))

## v1.18.0 (2019-12-23)

#### New Feature 🎉

- [#176](https://github.com/Schniz/fnm/pull/176) Specify the release to the install script ([@chrisdaley](https://github.com/chrisdaley))

#### Bugfix 🐛

- [#177](https://github.com/Schniz/fnm/pull/177) Fix "illegal instruction" errors on some CPUs ([@Schniz](https://github.com/Schniz))

#### Internal 🛠

- [#179](https://github.com/Schniz/fnm/pull/179) Bump lwt version ([@Schniz](https://github.com/Schniz))

#### Committers: 2

- Chris Daley ([@chrisdaley](https://github.com/chrisdaley))
- Gal Schlezinger ([@Schniz](https://github.com/Schniz))

## v1.17.0 (2019-11-26)

#### Bugfix 🐛

- [#169](https://github.com/Schniz/fnm/pull/169) Support spaces in directory name in Bash ([@Schniz](https://github.com/Schniz))

#### Internal 🛠

- [#172](https://github.com/Schniz/fnm/pull/172) Revert into using `ocaml-tls` instead of `ocaml-ssl` ([@Schniz](https://github.com/Schniz))

#### Committers: 1

- Gal Schlezinger ([@Schniz](https://github.com/Schniz))

## v1.16.0 (2019-10-27)

#### New Feature 🎉

- [#146](https://github.com/Schniz/fnm/pull/146) Add support for `lts/*` ([@Schniz](https://github.com/Schniz))

#### Bugfix 🐛

- [#150](https://github.com/Schniz/fnm/pull/150) Add missing log level to `env` output for Fish shell ([@thomsj](https://github.com/thomsj))

#### Internal 🛠

- [#154](https://github.com/Schniz/fnm/pull/154) Run all feature tests ([@Schniz](https://github.com/Schniz))
- [#148](https://github.com/Schniz/fnm/pull/148) Make `env` smoke test pass when run with fish ([@thomsj](https://github.com/thomsj))
- [#153](https://github.com/Schniz/fnm/pull/153) Add missing `exit 1`s to `partial_semver` tests ([@thomsj](https://github.com/thomsj))
- [#151](https://github.com/Schniz/fnm/pull/151) Add a CI check for code formatting ([@Schniz](https://github.com/Schniz))
- [#149](https://github.com/Schniz/fnm/pull/149) Update to v6.17.1 in `partial_semver` feature test ([@thomsj](https://github.com/thomsj))
- [#143](https://github.com/Schniz/fnm/pull/143) Try to install new deps ([@Schniz](https://github.com/Schniz))
- [#142](https://github.com/Schniz/fnm/pull/142) Drop buildsInSource ([@Schniz](https://github.com/Schniz))

#### Documentation 📝

- [#157](https://github.com/Schniz/fnm/pull/157) Rename `--base-dir` to `--fnm-dir` in README ([@thomsj](https://github.com/thomsj))
- [#158](https://github.com/Schniz/fnm/pull/158) Uncapitalise "Node" in `--multi` description ([@thomsj](https://github.com/thomsj))

#### Committers: 2

- Gal Schlezinger ([@Schniz](https://github.com/Schniz))
- James Thomson ([@thomsj](https://github.com/thomsj))

## v1.15.0 (2019-09-23)

#### Bugfix 🐛

- [#138](https://github.com/Schniz/fnm/pull/138) Do uninstallation in steps ([@Schniz](https://github.com/Schniz))

#### Documentation 📝

- [#132](https://github.com/Schniz/fnm/pull/132) Fix spelling of availability in install.sh ([@trevershick](https://github.com/trevershick))

#### Committers: 2

- Gal Schlezinger ([@Schniz](https://github.com/Schniz))
- Trever Shick ([@trevershick](https://github.com/trevershick))

## v1.14.0 (2019-08-20)

#### New Feature 🎉

- [#134](https://github.com/Schniz/fnm/pull/134) Alias -v to --version ([@Schniz](https://github.com/Schniz))

#### Bugfix 🐛

- [#131](https://github.com/Schniz/fnm/pull/131) Deprecates MacOS installation using the script in favor of Homebrew ([@Schniz](https://github.com/Schniz))

#### Internal 🛠

- [#133](https://github.com/Schniz/fnm/pull/133) Fix Windows build once again! ([@Schniz](https://github.com/Schniz))

#### Committers: 1

- Gal Schlezinger ([@Schniz](https://github.com/Schniz))

## v1.13.0 (2019-07-15)

#### New Feature 🎉

- [#129](https://github.com/Schniz/fnm/pull/129) Alias latest versions on installation ([@Schniz](https://github.com/Schniz))

#### Bugfix 🐛

- [#125](https://github.com/Schniz/fnm/pull/125) format versions in `uninstall` ([@Schniz](https://github.com/Schniz))
- [#114](https://github.com/Schniz/fnm/pull/114) installation script: use $INSTALL_DIR instead of hard-coded $HOME/.fnm ([@Schniz](https://github.com/Schniz))

#### Documentation 📝

- [#130](https://github.com/Schniz/fnm/pull/130) Fix issues related to help pages ([@Schniz](https://github.com/Schniz))
- [#123](https://github.com/Schniz/fnm/pull/123) Fix typo in successfully ([@pavelloz](https://github.com/pavelloz))
- [#118](https://github.com/Schniz/fnm/pull/118) Add note about upgrading ([@pavelloz](https://github.com/pavelloz))
- [#119](https://github.com/Schniz/fnm/pull/119) Remove "Homebrew" from future plans as it is done ([@pavelloz](https://github.com/pavelloz))

#### Committers: 2

- Gal Schlezinger ([@Schniz](https://github.com/Schniz))
- Paweł Kowalski ([@pavelloz](https://github.com/pavelloz))

## v1.12.0 (2019-06-06)

#### New Feature 🎉

- [#106](https://github.com/Schniz/fnm/pull/106) Add `default`, as a shortcut for `alias default` ([@dangdennis](https://github.com/dangdennis))
- [#104](https://github.com/Schniz/fnm/pull/104) Add a 'debug' log level ([@Schniz](https://github.com/Schniz))

#### Internal 🛠

- [#88](https://github.com/Schniz/fnm/pull/88) Successfully build on Windows ([@ulrikstrid](https://github.com/ulrikstrid))

#### Committers: 3

- Dennis Dang ([@dangdennis](https://github.com/dangdennis))
- Gal Schlezinger ([@Schniz](https://github.com/Schniz))
- Ulrik Strid ([@ulrikstrid](https://github.com/ulrikstrid))

## v1.11.0 (2019-05-27)

#### New Feature 🎉

- [#98](https://github.com/Schniz/fnm/pull/98) Add `uninstall` command ([@tatchi](https://github.com/tatchi))
- [#97](https://github.com/Schniz/fnm/pull/97) Add the ability to use system version of Node ([@Schniz](https://github.com/Schniz))

#### Bugfix 🐛

- [#103](https://github.com/Schniz/fnm/pull/103) Fix missing aliases due to newer `realpath` ([@Schniz](https://github.com/Schniz))
- [#99](https://github.com/Schniz/fnm/pull/99) fix EACCES error when installing an already downloaded version ([@tatchi](https://github.com/tatchi))

#### Internal 🛠

- [#101](https://github.com/Schniz/fnm/pull/101) Move from base to core ([@ulrikstrid](https://github.com/ulrikstrid))
- [#102](https://github.com/Schniz/fnm/pull/102) Implement `realpath` instead of binding to C library ([@Schniz](https://github.com/Schniz))
- [#100](https://github.com/Schniz/fnm/pull/100) Add Semver to library, an simple non-spec implementation of semver ([@Schniz](https://github.com/Schniz))

#### Committers: 3

- Corentin Leruth ([@tatchi](https://github.com/tatchi))
- Gal Schlezinger ([@Schniz](https://github.com/Schniz))
- Ulrik Strid ([@ulrikstrid](https://github.com/ulrikstrid))

## v1.10.0 (2019-05-01)

#### New Feature 🎉

- [#93](https://github.com/Schniz/fnm/pull/93) Add support for log level (Closes [#33](https://github.com/Schniz/fnm/issues/33)) ([@ohana54](https://github.com/ohana54))

#### Documentation 📝

- [#95](https://github.com/Schniz/fnm/pull/95) Shorten installation script url ([@vladimyr](https://github.com/vladimyr))

#### Committers: 2

- Dario Vladović ([@vladimyr](https://github.com/vladimyr))
- Tomer Ohana ([@ohana54](https://github.com/ohana54))

## v1.9.1 (2019-04-14)

#### Bugfix 🐛

- [#91](https://github.com/Schniz/fnm/pull/91) Fix `fnm env` for fish shell. ([@hwartig](https://github.com/hwartig))
- [#90](https://github.com/Schniz/fnm/pull/90) Installation script doesn't use GitHub API, but a link to the latest directly ([@Schniz](https://github.com/Schniz))

#### Committers: 2

- Gal Schlezinger ([@Schniz](https://github.com/Schniz))
- Harald Wartig ([@hwartig](https://github.com/hwartig))

## v1.9.0 (2019-03-18)

#### New Feature 🎉

- [#86](https://github.com/Schniz/fnm/pull/86) Add support for interactive installation for use ([@Schniz](https://github.com/Schniz))

#### Documentation 📝

- [#85](https://github.com/Schniz/fnm/pull/85) Update README.md ([@Schniz](https://github.com/Schniz))

#### Committers: 1

- Gal Schlezinger ([@Schniz](https://github.com/Schniz))

## v1.8.0 (2019-03-13)

#### Bugfix 🐛

- [#83](https://github.com/Schniz/fnm/pull/83) fix: remove unmatched quote written in the fish config file ([@thomasmarcel](https://github.com/thomasmarcel))

#### Internal 🛠

- [#84](https://github.com/Schniz/fnm/pull/84) Strip binaries to make them smaller ([@Schniz](https://github.com/Schniz))

#### Committers: 2

- Gal Schlezinger ([@Schniz](https://github.com/Schniz))
- Thomas Alcala Schneider ([@thomasmarcel](https://github.com/thomasmarcel))

## v1.7.2 (2019-03-07)

#### Bugfix 🐛

- [#79](https://github.com/Schniz/fnm/pull/79) Guard from more non-existent directories errors ([@Schniz](https://github.com/Schniz))

#### Committers: 1

- Gal Schlezinger ([@Schniz](https://github.com/Schniz))

## v1.7.1 (2019-03-05)

#### Bugfix 🐛

- [#77](https://github.com/Schniz/fnm/pull/77) Fix "command not found: elsif" error ([@jletey](https://github.com/jletey))

#### Internal 🛠

- [#78](https://github.com/Schniz/fnm/pull/78) Add a test to `use-on-cd` when `.node-version` is found ([@Schniz](https://github.com/Schniz))

#### Committers: 2

- Gal Schlezinger ([@Schniz](https://github.com/Schniz))
- John Letey ([@jletey](https://github.com/jletey))

## v1.7.0 (2019-03-04)

#### New Feature 🎉

- [#68](https://github.com/Schniz/fnm/pull/68) Infer shells automatically, and `use` versions based on the current working directory (optional) ([@Schniz](https://github.com/Schniz))

#### Committers: 1

- Gal Schlezinger ([@Schniz](https://github.com/Schniz))

## v1.6.2 (2019-03-04)

#### Bugfix 🐛

- [#72](https://github.com/Schniz/fnm/pull/72) Fix alias paths ([@Schniz](https://github.com/Schniz))

#### Documentation 📝

- [#70](https://github.com/Schniz/fnm/pull/70) Fix installation script parameters docs ([@Schniz](https://github.com/Schniz))

#### Committers: 1

- Gal Schlezinger ([@Schniz](https://github.com/Schniz))

## v1.6.1 (2019-02-26)

#### Bugfix 🐛

- [#69](https://github.com/Schniz/fnm/pull/69) Fix version inference by throwing on http 404 again ([@Schniz](https://github.com/Schniz))

#### Committers: 1

- Gal Schlezinger ([@Schniz](https://github.com/Schniz))

## v1.6.0 (2019-02-25)

#### New Feature 🎉

- [#57](https://github.com/Schniz/fnm/pull/57) Switch to cohttp(lwt) instead of curl ([@tatchi](https://github.com/tatchi))

#### Bugfix 🐛

- [#64](https://github.com/Schniz/fnm/pull/64) Throw on errors in installation script ([@Schniz](https://github.com/Schniz))

#### Internal 🛠

- [#67](https://github.com/Schniz/fnm/pull/67) Use `perl-utils` instead of custom written `shasum` ([@Schniz](https://github.com/Schniz))
- [#66](https://github.com/Schniz/fnm/pull/66) Use newer esy ([@Schniz](https://github.com/Schniz))

#### Committers: 2

- Corentin Leruth ([@tatchi](https://github.com/tatchi))
- Gal Schlezinger ([@Schniz](https://github.com/Schniz))

## v1.5.1 (2019-02-22)

#### Bugfix 🐛

- [#61](https://github.com/Schniz/fnm/pull/61) Fix a bug where `fnm env --multi` didn't used the default alias ([@Schniz](https://github.com/Schniz))

#### Committers: 1

- Gal Schlezinger ([@Schniz](https://github.com/Schniz))

## v1.5.0 (2019-02-21)

#### New Feature 🎉

- [#60](https://github.com/Schniz/fnm/pull/60) Disable colors for non-tty devices ([@Schniz](https://github.com/Schniz))
- [#48](https://github.com/Schniz/fnm/pull/48) Add parameters to the install script, enabling custom installs (`--install-dir` and `--skip-shell`) ([@from-nibly](https://github.com/from-nibly))
- [#54](https://github.com/Schniz/fnm/pull/54) Infer complete semver (`vX.X.X`) out of partial input (`vX`/`vX.X`). ([@Schniz](https://github.com/Schniz))

#### Bugfix 🐛

- [#58](https://github.com/Schniz/fnm/pull/58) Adding check for OSX during writing for bash shell ([@maxknee](https://github.com/maxknee))
- [#56](https://github.com/Schniz/fnm/pull/56) Correct status code on `install` failures ([@ranyitz](https://github.com/ranyitz))

#### Internal 🛠

- [#55](https://github.com/Schniz/fnm/pull/55) Make tests faster by using cnpmjs as Node.js mirror in tests ([@Schniz](https://github.com/Schniz))

#### Documentation 📝

- [#49](https://github.com/Schniz/fnm/pull/49) Add a `--fnm-dir` option to `fnm env` ([@Schniz](https://github.com/Schniz))
- [#50](https://github.com/Schniz/fnm/pull/50) Added CHANGELOG ([@Schniz](https://github.com/Schniz))

#### Committers: 4

- Gal Schlezinger ([@Schniz](https://github.com/Schniz))
- Jordan Davidson ([@from-nibly](https://github.com/from-nibly))
- Max Knee ([@maxknee](https://github.com/maxknee))
- Ran Yitzhaki ([@ranyitz](https://github.com/ranyitz))

## v1.4.0 (2019-02-18)

#### New Feature 🎉

- [#45](https://github.com/Schniz/fnm/pull/45) Use exit code 1 on errors on `fnm use` ([@Schniz](https://github.com/Schniz))
- [#42](https://github.com/Schniz/fnm/pull/42) Add support for .node-version files ([@Dean177](https://github.com/Dean177))

#### Documentation 📝

- [#44](https://github.com/Schniz/fnm/pull/44) Quick fix for the dev environment setup ([@AdamGS](https://github.com/AdamGS))

#### Committers: 3

- Adam Gutglick ([@AdamGS](https://github.com/AdamGS))
- Dean Merchant ([@Dean177](https://github.com/Dean177))
- Gal Schlezinger ([@Schniz](https://github.com/Schniz))

## v1.3.0 (2019-02-14)

#### New Feature 🎉

- [#36](https://github.com/Schniz/fnm/pull/36) Support Node.js mirrors ([@Schniz](https://github.com/Schniz))
- [#30](https://github.com/Schniz/fnm/pull/30) Aliases and multishell support ([@Schniz](https://github.com/Schniz))
- [#37](https://github.com/Schniz/fnm/pull/37) Don't throw on existing installation ([@Schniz](https://github.com/Schniz))
- [#27](https://github.com/Schniz/fnm/pull/27) skip installation if the version is already installed ([@kentac55](https://github.com/kentac55))

#### Documentation 📝

- [#22](https://github.com/Schniz/fnm/pull/22) Add a LICENSE file ([@Schniz](https://github.com/Schniz))

#### Committers: 2

- Gal Schlezinger ([@Schniz](https://github.com/Schniz))
- [@kentac55](https://github.com/kentac55)

## v1.2.1 (2019-02-11)

#### Bugfix 🐛

- [#25](https://github.com/Schniz/fnm/pull/25) CI (fnm-linux => fnm) ([@Schniz](https://github.com/Schniz))

#### Internal 🛠

- [#21](https://github.com/Schniz/fnm/pull/21) Add feature test for Fish shell ([@Schniz](https://github.com/Schniz))

#### Documentation 📝

- [#23](https://github.com/Schniz/fnm/pull/23) Add installation script ([@Schniz](https://github.com/Schniz))

#### Committers: 1

- Gal Schlezinger ([@Schniz](https://github.com/Schniz))

## v1.2.0 (2019-01-30)

#### New Feature 🎉

- [#17](https://github.com/Schniz/fnm/pull/17) Use xz files instead of gz ([@Schniz](https://github.com/Schniz))

#### Bugfix 🐛

- [#16](https://github.com/Schniz/fnm/pull/16) Make `fnm --version` show the correct version ([@Schniz](https://github.com/Schniz))
- [#15](https://github.com/Schniz/fnm/pull/15) Don't throw in nonexistent directory on `fnm ls` ([@Schniz](https://github.com/Schniz))

#### Documentation 📝

- [#13](https://github.com/Schniz/fnm/pull/13) Added short docs to the README ([@Schniz](https://github.com/Schniz))

#### Committers: 1

- Gal Schlezinger ([@Schniz](https://github.com/Schniz))

## v1.1.0 (2019-01-27)

#### New Feature 🎉

- [#10](https://github.com/Schniz/fnm/pull/10) Add fish shell setup to `env` command and README ([@elliottsj](https://github.com/elliottsj))

#### Committers: 1

- Spencer Elliott ([@elliottsj](https://github.com/elliottsj))
