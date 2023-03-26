use crate::version_file_strategy::VersionFileStrategy;

use super::shell::Shell;
use indoc::formatdoc;
use std::{fs::File, io::Write, path::Path};

#[derive(Debug, Default)]
pub struct Nushell {
    nu_env_script: Option<File>,
    nu_config_script: Option<File>,
}

impl Shell for Nushell {
    fn env_init(&mut self, config: &crate::config::FnmConfig) -> anyhow::Result<String> {
        let base_dir = config.base_dir_with_default();
        let nu_env_script_path = base_dir.join("fnm_env.nu");
        let nu_config_script_path = base_dir.join("fnm_config.nu");

        self.nu_env_script = Some(File::create(nu_env_script_path).unwrap());
        self.nu_config_script = Some(File::create(nu_config_script_path).unwrap());

        Ok("".to_string())
    }

    fn to_clap_shell(&self) -> clap_complete::Shell {
        panic!("Shell completion is not supported for Nushell (yet.) See https://github.com/clap-rs/clap/issues/2778 for updates.");
    }

    fn path(&mut self, path: &Path, _: &crate::config::FnmConfig) -> anyhow::Result<String> {
        let path = path
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("Can't convert path to string"))?;

        if std::env::consts::OS == "windows" {
            writeln!(
                &mut self.nu_env_script.as_ref().unwrap(),
                "let-env Path = ($env.Path | prepend {:?})",
                path
            ).unwrap();
        } else {
            writeln!(
                self.nu_env_script.as_ref().unwrap(),
                "let-env PATH = ($env.Path | prepend {:?})",
                path
            ).unwrap();
        }

        Ok("".to_string())
    }

    fn set_env_var(&mut self, name: &str, value: &str, _: &crate::config::FnmConfig) -> String {
        writeln!(
            &mut self.nu_env_script.as_ref().unwrap(),
            "let-env {} = {:?}",
            name,
            value
        ).unwrap();
        format!("# {} = {:?}", name, value)
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
