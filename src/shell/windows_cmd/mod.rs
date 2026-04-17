use super::shell::Shell;
use std::path::Path;

#[derive(Debug)]
pub struct WindowsCmd;

impl Shell for WindowsCmd {
    fn to_clap_shell(&self) -> clap_complete::Shell {
        // TODO: move to Option
        panic!("Shell completion is not supported for Windows Command Prompt. Maybe try using PowerShell for a better experience?");
    }

    fn path(&self, path: &Path) -> anyhow::Result<String> {
        let current_path =
            std::env::var_os("path").ok_or_else(|| anyhow::anyhow!("Can't read PATH env var"))?;
        let mut split_paths: Vec<_> = std::env::split_paths(&current_path).collect();
        split_paths.insert(0, path.to_path_buf());
        let new_path = std::env::join_paths(split_paths)
            .map_err(|err| anyhow::anyhow!("Can't join paths: {}", err))?;
        let new_path = new_path
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("Can't convert path to string"))?;
        Ok(format!("SET PATH={new_path}"))
    }

    fn set_env_var(&self, name: &str, value: &str) -> String {
        format!("SET {name}={value}")
    }

    fn use_on_cd(
        &self,
        config: &crate::config::FnmConfig,
        install_if_missing: bool,
    ) -> anyhow::Result<String> {
        let path = config.base_dir_with_default().join("cd.cmd");
        create_cd_file_at(&path, install_if_missing).map_err(|source| {
            anyhow::anyhow!(
                "Can't create cd.cmd file for use-on-cd at {}: {}",
                path.display(),
                source
            )
        })?;
        let path = path
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("Can't read path to cd.cmd"))?;
        Ok(format!("doskey cd=\"{path}\" $*",))
    }
}

fn create_cd_file_at(path: &std::path::Path, install_if_missing: bool) -> std::io::Result<()> {
    use std::io::Write;
    let extra_flags = if install_if_missing {
        " --install-if-missing"
    } else {
        ""
    };
    let cmd_contents = format!(
        "@echo off\r\ncd /d %*\r\nif \"%FNM_VERSION_FILE_STRATEGY%\" == \"recursive\" (\r\n  fnm use --silent-if-unchanged{extra_flags}\r\n) else (\r\n  if exist .nvmrc (\r\n    fnm use --silent-if-unchanged{extra_flags}\r\n  ) else (\r\n    if exist .node-version (\r\n      fnm use --silent-if-unchanged{extra_flags}\r\n    )\r\n  )\r\n)\r\n@echo on\r\n",
        extra_flags = extra_flags,
    );
    let mut file = std::fs::File::create(path)?;
    file.write_all(cmd_contents.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn use_on_cd_quotes_macro_path() {
        let base_dir = std::env::temp_dir().join("fnm cmd test with spaces");
        std::fs::create_dir_all(&base_dir).unwrap();
        let config = crate::config::FnmConfig::default().with_base_dir(Some(base_dir.clone()));
        let output = WindowsCmd.use_on_cd(&config, false).unwrap();

        assert!(output.starts_with("doskey cd=\""));
        assert!(output.ends_with("\" $*"));
        assert!(output.contains("with spaces"));
        assert!(output.contains("cd.cmd"));

        std::fs::remove_file(base_dir.join("cd.cmd")).unwrap();
        std::fs::remove_dir_all(base_dir).unwrap();
    }

    #[test]
    fn use_on_cd_without_install_if_missing() {
        let base_dir = std::env::temp_dir().join("fnm cmd test no install");
        std::fs::create_dir_all(&base_dir).unwrap();
        let config = crate::config::FnmConfig::default().with_base_dir(Some(base_dir.clone()));
        WindowsCmd.use_on_cd(&config, false).unwrap();

        let cd_cmd = std::fs::read_to_string(base_dir.join("cd.cmd")).unwrap();
        assert!(cd_cmd.contains("fnm use --silent-if-unchanged"));
        assert!(!cd_cmd.contains("--install-if-missing"));

        std::fs::remove_file(base_dir.join("cd.cmd")).unwrap();
        std::fs::remove_dir_all(base_dir).unwrap();
    }

    #[test]
    fn use_on_cd_with_install_if_missing() {
        let base_dir = std::env::temp_dir().join("fnm cmd test install");
        std::fs::create_dir_all(&base_dir).unwrap();
        let config = crate::config::FnmConfig::default().with_base_dir(Some(base_dir.clone()));
        WindowsCmd.use_on_cd(&config, true).unwrap();

        let cd_cmd = std::fs::read_to_string(base_dir.join("cd.cmd")).unwrap();
        assert!(cd_cmd.contains("fnm use --silent-if-unchanged --install-if-missing"));

        std::fs::remove_file(base_dir.join("cd.cmd")).unwrap();
        std::fs::remove_dir_all(base_dir).unwrap();
    }
}
