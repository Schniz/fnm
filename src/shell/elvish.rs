use crate::version_file_strategy::VersionFileStrategy;

use super::shell::Shell;
use indoc::formatdoc;
use std::path::Path;

#[derive(Debug)]
pub struct Elvish;

impl Shell for Elvish {
    fn to_clap_shell(&self) -> clap_complete::Shell {
        clap_complete::Shell::Elvish
    }

    fn path(&self, path: &Path) -> anyhow::Result<String> {
        let path = path
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("Can't convert path to string"))?;
        let path =
            super::windows_compat::maybe_fix_windows_path(path).unwrap_or_else(|| path.to_string());
        Ok(format!("set-env PATH {path:?}:$E:PATH;"))
    }

    fn set_env_var(&self, name: &str, value: &str) -> String {
        format!("set-env {name} {value:?};")
    }

    fn use_on_cd(&self, config: &crate::config::FnmConfig) -> anyhow::Result<String> {
        let version_file_exists_condition = if config.resolve_engines() {
            "test -f .node-version -o -f .nvmrc -o -f package.json"
        } else {
            "test -f .node-version -o -f .nvmrc"
        };
        let autoload_hook = match config.version_file_strategy() {
            VersionFileStrategy::Local => formatdoc!(
                r#"
                try {{
                  {version_file_exists_condition}
                }} catch e {{
                }} else {{
                  fnm use --silent-if-unchanged
                }}
                "#,
                version_file_exists_condition = version_file_exists_condition,
            ),
            VersionFileStrategy::Recursive => String::from(r"fnm use --silent-if-unchanged"),
        };
        Ok(formatdoc!(
            r#"
            set after-chdir = [{{|dir| {autoload_hook} }}]
            "#,
            autoload_hook = autoload_hook
        ))
    }
}
