use super::shell::Shell;
use indoc::indoc;
use std::path::Path;

#[derive(Debug)]
pub struct Zsh;

impl Shell for Zsh {
    fn to_structopt_shell(&self) -> structopt::clap::Shell {
        structopt::clap::Shell::Zsh
    }

    fn path(&self, path: &Path) -> String {
        format!("export PATH={:?}:$PATH", path.to_str().unwrap())
    }

    fn set_env_var(&self, name: &str, value: &str) -> String {
        format!("export {}={:?}", name, value)
    }

    fn use_on_cd(&self, _config: &crate::config::FnmConfig) -> String {
        indoc!(
            r#"
                autoload -U add-zsh-hook
                _fnm_autoload_hook () {
                    if [[ -f .node-version || -f .nvmrc ]]; then
                        fnm use
                    fi
                }

                add-zsh-hook chpwd _fnm_autoload_hook \
                    && _fnm_autoload_hook
            "#
        )
        .into()
    }
}
