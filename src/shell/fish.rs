use crate::version_file_strategy::VersionFileStrategy;

use super::shell::Shell;
use indoc::{formatdoc, indoc};
use std::path::Path;

#[derive(Debug)]
pub struct Fish;

impl Shell for Fish {
    fn to_clap_shell(&self) -> clap_complete::Shell {
        clap_complete::Shell::Fish
    }

    fn path(&self, path: &Path) -> String {
        let temp_dir = std::env::temp_dir().join("fnm_multishells");
        formatdoc!(
            r#"
                set -l new_path {path}
                for p in $PATH
                    if ! echo $p | grep -q "{temp_dir}"
                        set new_path $new_path $p
                    end
                end
                set -gx PATH $new_path
            "#,
            temp_dir = temp_dir.display(),
            path = path.to_str().unwrap()
        )
    }

    fn set_env_var(&self, name: &str, value: &str) -> String {
        format!("set -gx {name} {value:?};", name = name, value = value)
    }

    fn use_on_cd(&self, config: &crate::config::FnmConfig) -> String {
        let autoload_hook = match config.version_file_strategy() {
            VersionFileStrategy::Local => indoc!(
                r#"
                    if test -f .node-version -o -f .nvmrc
                        fnm use --silent-if-unchanged
                    end
                "#
            ),
            VersionFileStrategy::Recursive => r#"fnm use --silent-if-unchanged"#,
        };
        formatdoc!(
            r#"
                function _fnm_autoload_hook --on-variable PWD --description 'Change Node version on directory change'
                    status --is-command-substitution; and return
                    {autoload_hook}
                end

                _fnm_autoload_hook
            "#,
            autoload_hook = autoload_hook
        )
    }
}
