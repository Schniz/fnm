use super::shell::{path_entries, PathOrder, Shell};
use std::path::Path;

#[derive(Debug)]
pub struct WindowsCmd;

impl Shell for WindowsCmd {
    fn to_clap_shell(&self) -> clap_complete::Shell {
        // TODO: move to Option
        panic!("Shell completion is not supported for Windows Command Prompt. Maybe try using PowerShell for a better experience?");
    }

    fn path(
        &self,
        path: &Path,
        order: PathOrder,
        multishell_storage: &Path,
    ) -> anyhow::Result<String> {
        std::env::var_os("path").ok_or_else(|| anyhow::anyhow!("Can't read PATH env var"))?;
        let new_path = std::env::join_paths(path_entries(path, order, multishell_storage))
            .map_err(|err| anyhow::anyhow!("Can't join paths: {err}"))?;
        let new_path = new_path
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("Can't convert path to string"))?;
        Ok(format!("SET PATH={new_path}"))
    }

    fn set_env_var(&self, name: &str, value: &str) -> String {
        format!("SET {name}={value}")
    }

    fn use_on_cd(&self, config: &crate::config::FnmConfig) -> anyhow::Result<String> {
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
        Ok(format!("doskey cd=\"{path}\" $*"))
    }
}

fn create_cd_file_at(path: &std::path::Path) -> std::io::Result<()> {
    use std::io::Write;
    let cmd_contents = include_bytes!("./cd.cmd");
    let mut file = std::fs::File::create(path)?;
    file.write_all(cmd_contents)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(windows)]
    #[test]
    fn renders_prepend_and_append_path_order() {
        let storage = std::env::temp_dir().join("fnm-cmd-path-order-storage");
        let new_path = storage.join("new-session");
        let current_path = std::env::var_os("PATH").unwrap();
        let current_entries: Vec<_> = std::env::split_paths(&current_path).collect();

        let mut prepended = vec![new_path.clone()];
        prepended.extend(current_entries.clone());
        let expected_prepend = format!(
            "SET PATH={}",
            std::env::join_paths(prepended).unwrap().to_string_lossy()
        );
        assert_eq!(
            WindowsCmd
                .path(&new_path, PathOrder::Prepend, &storage)
                .unwrap(),
            expected_prepend
        );

        let mut appended = current_entries;
        appended.push(new_path.clone());
        let expected_append = format!(
            "SET PATH={}",
            std::env::join_paths(appended).unwrap().to_string_lossy()
        );
        assert_eq!(
            WindowsCmd
                .path(&new_path, PathOrder::Append, &storage)
                .unwrap(),
            expected_append
        );
    }

    #[cfg(windows)]
    #[test]
    fn append_path_survives_cmd_setup_evaluation() {
        let directory = tempfile::tempdir().unwrap();
        let output_path = directory.path().join("fnm-env.txt");
        let script_path = directory.path().join("append-path.cmd");
        let value = r"C:\safe;C:\new";
        std::fs::write(&output_path, format!("SET PATH={value}\r\n")).unwrap();
        std::fs::write(
            &script_path,
            format!(
                "@FOR /f \"usebackq tokens=*\" %%z IN (\"{}\") DO CALL %%z\r\n@ECHO %PATH%\r\n",
                output_path.display()
            ),
        )
        .unwrap();
        let output = std::process::Command::new("cmd.exe")
            .args(["/D", "/C"])
            .arg(script_path)
            .output()
            .unwrap();

        assert!(output.status.success());
        assert_eq!(
            String::from_utf8(output.stdout)
                .unwrap()
                .lines()
                .last()
                .unwrap()
                .trim(),
            value
        );
    }

    #[test]
    fn use_on_cd_quotes_macro_path() {
        let base_dir = std::env::temp_dir().join("fnm cmd test with spaces");
        std::fs::create_dir_all(&base_dir).unwrap();
        let config = crate::config::FnmConfig::default().with_base_dir(Some(base_dir.clone()));
        let output = WindowsCmd.use_on_cd(&config).unwrap();

        assert!(output.starts_with("doskey cd=\""));
        assert!(output.ends_with("\" $*"));
        assert!(output.contains("with spaces"));
        assert!(output.contains("cd.cmd"));

        std::fs::remove_file(base_dir.join("cd.cmd")).unwrap();
        std::fs::remove_dir_all(base_dir).unwrap();
    }
}
