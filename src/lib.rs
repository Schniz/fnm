//! fnm as a library.
//!
//! The `fnm` program is a thin layer over this crate: it parses arguments,
//! prints, and exits. An application that wants to manage Node.js versions
//! itself can skip all of that and call [`Fnm`], which returns values
//! instead of printing them and never ends the process.
//!
//! ```no_run
//! let fnm = fnm::Fnm::new();
//! let node = fnm.install("lts")?;
//! println!("{} lives in {}", node.version, node.bin_dir.display());
//! # Ok::<(), fnm::Error>(())
//! ```
//!
//! The command-line program, and the crates only it needs (progress bars,
//! shell detection, completions), sit behind the `cli` feature. It is on by
//! default; a library user turns it off with `default-features = false`.

#![warn(rust_2018_idioms, clippy::all, clippy::pedantic)]
#![allow(
    clippy::enum_variant_names,
    clippy::large_enum_variant,
    clippy::module_name_repetitions,
    clippy::similar_names
)]
// Without the command-line program, the helpers only it calls go unused.
#![cfg_attr(not(feature = "cli"), allow(dead_code, unused_macros))]

mod alias;
mod arch;
mod archive;
mod choose_version_for_user_input;
#[cfg(feature = "cli")]
mod cli;
#[cfg(feature = "cli")]
mod commands;
mod config;
mod current_version;
mod directory_portal;
mod downloader;
mod fs;
mod http;
mod installed_versions;
mod lts;
mod package_json;
mod path_ext;
#[cfg(feature = "cli")]
mod progress;
mod remote_node_index;
#[cfg(feature = "cli")]
mod shell;
mod system_info;
mod system_version;
mod user_version;
mod user_version_reader;
mod version;
mod version_file_strategy;
mod version_files;

#[macro_use]
mod log_level;
mod default_version;
mod directories;
mod pretty_serde;

pub mod api;

pub use api::{Error, Fnm, InstalledVersion, RemoteVersion};

/// Runs the `fnm` command-line program: parses the process arguments, runs
/// the command, and exits the process if it fails.
///
/// This is the body of the `fnm` binary. It is not part of the library API;
/// an application embedding fnm wants [`Fnm`].
#[cfg(feature = "cli")]
#[doc(hidden)]
pub fn run_cli() {
    env_logger::init();
    let value = crate::cli::parse();
    value.subcmd.call(value.config);
}
