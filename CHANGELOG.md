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

- [#83](https://github.com/Schniz/fnm/pull/83) fix: remove unmatched quote written in the fish config file ([@ThomasMarcel](https://github.com/ThomasMarcel))

#### Internal 🛠

- [#84](https://github.com/Schniz/fnm/pull/84) Strip binaries to make them smaller ([@Schniz](https://github.com/Schniz))

#### Committers: 2

- Gal Schlezinger ([@Schniz](https://github.com/Schniz))
- Thomas Alcala Schneider ([@ThomasMarcel](https://github.com/ThomasMarcel))

## v1.7.2 (2019-03-07)

#### Bugfix 🐛

- [#79](https://github.com/Schniz/fnm/pull/79) Guard from more non-existent directories errors ([@Schniz](https://github.com/Schniz))

#### Committers: 1

- Gal Schlezinger ([@Schniz](https://github.com/Schniz))

## v1.7.1 (2019-03-05)

#### Bugfix 🐛

- [#77](https://github.com/Schniz/fnm/pull/77) Fix "command not found: elsif" error ([@johnletey](https://github.com/johnletey))

#### Internal 🛠

- [#78](https://github.com/Schniz/fnm/pull/78) Add a test to `use-on-cd` when `.node-version` is found ([@Schniz](https://github.com/Schniz))

#### Committers: 2

- Gal Schlezinger ([@Schniz](https://github.com/Schniz))
- John Letey ([@johnletey](https://github.com/johnletey))

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
