use super::shell::Shell;
use anyhow::Result;
use std::path::Path;
use crate::shell::PathFormatter;

#[derive(Debug)]
pub struct WindowsCmd;

impl Shell for WindowsCmd {
    fn to_clap_shell(&self) -> clap_complete::Shell {
        // TODO: move to Option
        panic!("Shell completion is not supported for Windows Command Prompt. Maybe try using PowerShell for a better experience?");
    }

    fn path(&self, path: &Path, _formatter: &PathFormatter) -> Result<String> {
        let current_path =
            std::env::var_os("path").ok_or_else(|| anyhow::anyhow!("Can't read PATH env var"))?;
        let mut split_paths: Vec<_> = std::env::split_paths(&current_path).collect();
        split_paths.insert(0, path.to_path_buf());
        let new_path = std::env::join_paths(split_paths)
            .map_err(|err| anyhow::anyhow!("Can't join paths: {}", err))?;
        let new_path = new_path
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("Can't convert path to string"))?;
        Ok(format!("SET PATH={}", new_path))
    }

    fn set_env_var(&self, name: &str, value: &str) -> String {
        format!("SET {}={}", name, value)
    }

    fn use_on_cd(&self, config: &crate::config::FnmConfig) -> Result<String> {
        let path = config.base_dir_with_default().join("cd.cmd");
        create_cd_file_at(&path).map_err(|source| {
            anyhow::anyhow!(
                "Can't create cd.cmd file for use-on-cd at {}: {}",
                path.display(),
                source
            )
        })?;
        let path = path
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("Can't read path to cd.cmd"))?;
        Ok(format!("doskey cd={} $*", path,))
    }

    fn format_path(&self, path: &Path, _formatter: &PathFormatter) -> Result<String> {
        // cmd only runs on Windows, so we don't need to do path conversion
        Ok(path.to_str().unwrap().to_string())
    }
}

fn create_cd_file_at(path: &Path) -> std::io::Result<()> {
    use std::io::Write;
    let cmd_contents = include_bytes!("./cd.cmd");
    let mut file = std::fs::File::create(path)?;
    file.write_all(cmd_contents)?;
    Ok(())
}
