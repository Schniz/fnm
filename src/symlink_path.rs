use std::{fs::create_dir_all, path::PathBuf};

use log::info;
use sysinfo::{get_current_pid, ProcessExt, System, SystemExt};

use crate::config::FnmConfig;

pub fn generate_symlink_token_based_on_pid() -> Result<String, &'static str> {
    let mut system = System::new();
    let current_pid = get_current_pid()?;

    system.refresh_process(current_pid);
    let current_process = system
        .process(current_pid)
        .ok_or("Couldn't find the current process in the tree")?;

    let parent_pid = current_process
        .parent()
        .ok_or("Couldn't find the parent pid in the process tree")?;

    Ok(format!("pid_{}", parent_pid))
}

pub(crate) trait DefaultMultishellPathExt {
    fn multishell_path_or_default(&self) -> Result<PathBuf, Box<dyn std::error::Error>>;
}

impl DefaultMultishellPathExt for FnmConfig {
    fn multishell_path_or_default(&self) -> Result<PathBuf, Box<dyn std::error::Error>> {
        if let Some(given_path) = self.multishell_path() {
            return Ok(given_path.to_path_buf());
        }

        let symlink_token = generate_symlink_token_based_on_pid()?;
        let multishells_path = std::env::temp_dir().join("fnm_multishells");
        let new_path = multishells_path.join(symlink_token);

        info!(
            "No multishell path given, generating a new one at {}",
            new_path.display()
        );

        create_dir_all(&multishells_path)?;

        Ok(new_path)
    }
}
