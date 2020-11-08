use super::shell::Shell;
use indoc::indoc;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Bash;

impl Shell for Bash {
    fn into_structopt_shell(&self) -> structopt::clap::Shell {
        structopt::clap::Shell::Bash
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
                __fnm_use_if_file_found() {
                    if [[ -f .node-version || -f .nvmrc ]]; then
                        fnm use
                    fi
                }

                __fnmcd() {
                    \cd "$@" || return $?
                    __fnm_use_if_file_found
                }

                alias cd=__fnmcd
                __fnm_use_if_file_found
            "#
        )
        .into()
    }
}
