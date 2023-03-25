use crate::version_file_strategy::VersionFileStrategy;

use super::shell::Shell;
use indoc::formatdoc;
use std::{fs::File, io::Write, path::Path};

#[derive(Debug)]
struct Nushell {
    nu_env_script: Option<File>,
    nu_config_script: Option<File>,
}

impl Shell for Nushell {
    fn init(&mut self, config: &crate::config::FnmConfig) {
        self.nu_env_script =
            Some(File::create(config.base_dir_with_default().join("fnm_env.nu")).unwrap());
        self.nu_config_script =
            Some(File::create(config.base_dir_with_default().join("fnm_config.nu")).unwrap());
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
                &mut self.nu_env_script.unwrap(),
                "let-env Path = ($env.Path | prepend {:?})",
                path
            );
        } else {
            writeln!(
                &mut self.nu_env_script.unwrap(),
                "let-env PATH = ($env.Path | prepend {:?})",
                path
            );
        }

        Ok("".to_string())
    }

    fn set_env_var(&mut self, name: &str, value: &str, _: &crate::config::FnmConfig) -> String {
        // Nushell gets fnm environment variables in predefined script
        format!("# {} = {:?}", name, value)
    }

    fn use_on_cd(&self, config: &crate::config::FnmConfig) -> anyhow::Result<String> {
        Ok(match config.version_file_strategy() {
            VersionFileStrategy::Local => formatdoc!(
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
            ),
            VersionFileStrategy::Recursive => formatdoc!(
                r#"
                    let-env config = ($env.config | upsert hooks.env_change.PWD {{
                        [
                            {{
                                code: "fnm use --silent-if-unchanged"
                            }}
                        ]
                    }})
                "#
            ),
        })
    }
}
