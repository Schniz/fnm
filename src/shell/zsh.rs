use super::shell::Shell;
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

    fn use_on_cd(&self, _config: &crate::config::FnmConfig) -> String {
        indoc!(
            r#"
                autoload -U add-zsh-hook
                _fnm_autoload_hook () {
                    if [[ -f .node-version && -r .node-version ]]; then
                        echo "fnm: Found .node-version"
                        fnm use
                    elif [[ -f .nvmrc && -r .nvmrc ]]; then
                        echo "fnm: Found .nvmrc"
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
