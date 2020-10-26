use super::shell::Shell;
use crate::log_level::LogLevel;
use indoc::indoc;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Fish;

impl Shell for Fish {
    fn into_structopt_shell(&self) -> structopt::clap::Shell {
        structopt::clap::Shell::Fish
    }

    fn path(&self, path: &PathBuf) -> String {
        format!("set -gx PATH {:?} $PATH;", path.to_str().unwrap())
    }

    fn set_env_var(&self, name: &str, value: &str) -> String {
        format!("set -gx {name} {value:?};", name = name, value = value)
    }

    fn use_on_cd(&self, config: &crate::config::FnmConfig) -> String {
        format!(
            indoc!(
                "
                    function _fnm_autoload_hook --on-variable PWD --description 'Change Node version on directory change'
                        status --is-command-substitution; and return
                        if test -f .node-version
                            {}
                            fnm use
                        else if test -f .nvmrc
                            {}
                            fnm use
                        end
                    end
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
