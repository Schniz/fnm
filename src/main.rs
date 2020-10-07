mod alias;
mod archive;
mod choose_version_for_user_input;
mod cli;
mod commands;
mod config;
mod current_version;
mod directory_portal;
mod downloader;
mod fs;
mod installed_versions;
mod lts;
mod remote_node_index;
mod shell;
mod system_info;
mod system_version;
mod user_version;
mod version;
mod version_files;

#[macro_use]
mod log_level;

fn main() {
    env_logger::init();
    let value = crate::cli::parse();
    value.subcmd.call(value.config);
}
