use super::shell::Shell;
use crate::log_level::LogLevel;
use indoc::indoc;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Zsh;

impl Shell for Zsh {
    fn into_structopt_shell(&self) -> structopt::clap::Shell {
        structopt::clap::Shell::Zsh
    }

    fn path(&self, path: &PathBuf) -> String {
        format!("export PATH={:?}:$PATH", path.to_str().unwrap())
    }

    fn set_env_var(&self, name: &str, value: &str) -> String {
        format!("export {}={:?}", name, value)
    }

    fn use_on_cd(&self, config: &crate::config::FnmConfig) -> String {
        format!(
            indoc!(
                "
                    autoload -U add-zsh-hook
                    _fnm_autoload_hook () {{
                        if [[ -f .node-version && -r .node-version ]]; then
                            {}
                            fnm use
                        elif [[ -f .nvmrc && -r .nvmrc ]]; then
                            {}
                            fnm use
                        fi
                    }}

                    add-zsh-hook chpwd _fnm_autoload_hook \
                        && _fnm_autoload_hook
                ",
            ),
            self.echo_found(".node-version", config)
                .unwrap_or_else(|| "".into()),
            self.echo_found(".nvmrc", config)
                .unwrap_or_else(|| "".into()),
        )
        .into()
    }

    fn echo_found(&self, file_name: &str, config: &crate::config::FnmConfig) -> Option<String> {
        match config.log_level() {
            LogLevel::Info => Some(format!(r#"echo "fnm: {}""#, file_name).into()),
            _ => None,
        }
    }
}
