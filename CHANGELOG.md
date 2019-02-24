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

- [#44](https://github.com/Schniz/fnm/pull/44) Quick fix for the dev enviornment setup ([@AdamGS](https://github.com/AdamGS))

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
