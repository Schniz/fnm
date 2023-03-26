use crate::version_file_strategy::VersionFileStrategy;

use super::shell::Shell;
use dirs::home_dir;
use indoc::formatdoc;
use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf, MAIN_SEPARATOR},
};

#[derive(Debug, Default)]
pub struct Nushell {
    nu_env_script: Option<File>,
    nu_config_script: Option<File>,
}

fn nu_friendly_path_str(path: &PathBuf) -> String {
    let home_dir = home_dir().unwrap();
    if let Ok(path) = path.strip_prefix(home_dir) {
        let path = &path
            .to_str()
            .unwrap()
            .replace(MAIN_SEPARATOR, "/")
            .to_string();
        format!("~/{}", path)
    } else {
        path.to_str()
            .unwrap()
            .replace(MAIN_SEPARATOR, "/")
            .to_string()
    }
}

impl Shell for Nushell {
    fn env_init(&mut self, config: &crate::config::FnmConfig) -> anyhow::Result<String> {
        let base_dir = config.base_dir_with_default();
        let nu_env_script_path = base_dir.join("fnm_env.nu");
        let nu_config_script_path = base_dir.join("fnm_config.nu");

        self.nu_env_script = Some(File::create(&nu_env_script_path).unwrap());
        self.nu_config_script = Some(File::create(&nu_config_script_path).unwrap());

        let nu_env_script_path = nu_friendly_path_str(&nu_env_script_path);
        let nu_config_script_path = nu_friendly_path_str(&nu_config_script_path);

        writeln!(
            self.nu_env_script.as_ref().unwrap(),
            "fnm env --json | from json | load-env"
        )?;

        if std::env::consts::OS == "windows" {
            writeln!(
                self.nu_env_script.as_ref().unwrap(),
                "let-env Path = ($env.Path | prepend $env.FNM_MULTISHELL_PATH)"
            )?;
        } else {
            writeln!(
                self.nu_env_script.as_ref().unwrap(),
                "let-env PATH = ($env.PATH | prepend $env.FNM_MULTISHELL_PATH)"
            )?;
        }

        Ok(formatdoc!(
            r#"
            # Append to $nu.env-path
            # source-env {nu_env_script_path}

            # Append to $nu.config-path
            # source {nu_config_script_path}

            # For convenience:
            echo "source-env {nu_env_script_path}" | save $nu.env-path --append
            echo "source {nu_config_script_path}" | save $nu.config-path --append
        "#
        ))
    }

    fn to_clap_shell(&self) -> clap_complete::Shell {
        panic!("Shell completion is not supported for Nushell (yet.) See https://github.com/clap-rs/clap/issues/2778 for updates.");
    }

    fn path(&mut self, _: &Path, _: &crate::config::FnmConfig) -> anyhow::Result<String> {
        // PATH entries are always sourced in fnm_env.nu
        Ok("".to_string())
    }

    fn set_env_var(&mut self, _: &str, _: &str, _: &crate::config::FnmConfig) -> String {
        // Env vars are always sourced through json in fnm_env.nu
        "".to_string()
    }

    fn use_on_cd(&mut self, config: &crate::config::FnmConfig) -> anyhow::Result<String> {
        let nu_config_script = &mut self.nu_config_script.as_ref().unwrap();
        match config.version_file_strategy() {
            VersionFileStrategy::Local => nu_config_script.write(formatdoc!(
                r#"
                    let-env config = ($env.config | upsert hooks.env_change.PWD {{
                        [
                            {{
                                condition: {{|_, after| (($after | path join .node-version | path exists) or ($after | path join .nvmrc | path exists)) }}
                                code: "fnm use --silent-if-unchanged"
                            }}
                        ]
                    }})
                "#
            ).as_bytes()).unwrap(),
            VersionFileStrategy::Recursive => nu_config_script.write(formatdoc!(
                r#"
                    let-env config = ($env.config | upsert hooks.env_change.PWD {{
                        [
                            {{
                                code: "fnm use --silent-if-unchanged"
                            }}
                        ]
                    }})
                "#
            ).as_bytes()).unwrap(),
        };

        Ok("".to_string())
    }
}
