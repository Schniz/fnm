use super::shell::Shell;
use std::collections::HashSet;
use std::path::Path;

#[derive(Debug)]
pub struct WindowsCmd;

impl Shell for WindowsCmd {
    fn to_clap_shell(&self) -> clap_complete::Shell {
        // TODO: move to Option
        panic!("Shell completion is not supported for Windows Command Prompt. Maybe try using PowerShell for a better experience?");
    }

    fn path(&self, path: &Path) -> String {
        let current_path = std::env::var_os("PATH").expect("Can't read PATH env var");
        let cache_dir = crate::directories::multishell_storage();
        let mut split_paths: HashSet<_> = std::env::split_paths(&current_path)
            .filter(|p| !p.starts_with(&cache_dir))
            .collect();
        split_paths.insert(path.to_path_buf());
        let new_path = std::env::join_paths(split_paths).expect("Can't join paths");
        self.set_env_var("PATH", new_path.to_str().expect("Can't read PATH"))
    }

    fn set_env_var(&self, name: &str, value: &str) -> String {
        format!("SET {}={}", name, value)
    }

    fn use_on_cd(&self, config: &crate::config::FnmConfig) -> String {
        let path = config.base_dir_with_default().join("cd.cmd");
        create_cd_file_at(&path).expect("Can't create cd.cmd file for use-on-cd");
        format!(
            "doskey cd={} $*",
            path.to_str().expect("Can't read path to cd.cmd")
        )
    }
}

fn create_cd_file_at(path: &std::path::Path) -> std::io::Result<()> {
    use std::io::Write;
    let cmd_contents = include_bytes!("./cd.cmd");
    let mut file = std::fs::File::create(path)?;
    file.write_all(cmd_contents)?;
    Ok(())
}
