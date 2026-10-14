---
"fnm": minor
---

fnm can be used as a library. The `fnm` program is unchanged; it is now a thin layer over a library crate, so another Rust application can manage Node.js versions without requiring fnm to be installed:

```rust
let fnm = fnm::Fnm::new();
let node = fnm.install("lts")?;
let path = fnm.path_for("22")?; // PATH for a child process running Node.js 22
```

`fnm::Fnm` covers listing installed and downloadable versions, reading the version a directory asks for, installing, uninstalling, and setting the default. Its functions return values instead of printing, and never exit the process.

The command-line program and the dependencies only it needs (progress bars, shell detection, completions, the Windows manifest) are behind a `cli` feature that is on by default. A library user turns it off:

```toml
fnm = { version = "1", default-features = false }
```
