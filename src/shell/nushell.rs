use crate::version_file_strategy::VersionFileStrategy;

use super::shell::Shell;
use indoc::formatdoc;
use std::path::Path;

#[derive(Debug)]
pub struct Nushell;

impl Shell for Nushell {
    fn path(&self, path: &Path) -> anyhow::Result<String> {
        let path = path
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("Path is not valid UTF-8"))?;
        let path =
            super::windows_compat::maybe_fix_windows_path(path).unwrap_or_else(|| path.to_string());
        Ok(format!("$env.Path = ({path:?} | append $env.Path)"))
    }

    fn set_env_var(&self, name: &str, value: &str) -> String {
        format!("$env.{name} = {value:?}")
    }

    fn use_on_cd(&self, config: &crate::config::FnmConfig) -> anyhow::Result<String> {
        return Ok("# Currently nushell doesn't support `use_on_cd` check github for how to activate `use_on_cd`".to_string());

        let version_file_exists_condition = if config.resolve_engines() {
            "[ .node-version, .nvmrc, package.json ]"
        } else {
            "[ .node-version, .nvmrc ]"
        };
        let autoload_hook = match config.version_file_strategy() {
            VersionFileStrategy::Local => formatdoc!(
                r"
                    {version_file_exists_condition} | path exists | any {{}}
                ",
                version_file_exists_condition = version_file_exists_condition,
            ),
            VersionFileStrategy::Recursive => String::from(r"true"),
        };
        Ok(formatdoc!(
            r"
                  $env.config = (
                    $env.config?
                    | default {{}}
                    | upsert hooks {{ default {{}} }}
                    | upsert hooks.env_change {{ default {{}} }}
                    | upsert hooks.env_change.PWD {{ default [] }}
                  )
                  $env.config.hooks.env_change.PWD = ($env.config.hooks.env_change.PWD | append {{
                    condition: {{|_, after| {autoload_hook}}},
                    code: {{|_, _| fnm use --silent-if-unchanged}}
                  }})
            ",
            autoload_hook = autoload_hook
        ))
    }
}
