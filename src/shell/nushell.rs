use super::shell::Shell;
use std::path::Path;

#[derive(Debug)]
pub struct Nushell;

impl Shell for Nushell {
    fn to_clap_shell(&self) -> clap_complete::Shell {
        panic!("Shell completion is not supported for Nushell (yet.) See https://github.com/clap-rs/clap/issues/2778 for updates.");
    }

    fn path(&self, path: &Path) -> anyhow::Result<String> {
        let path = path
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("Can't convert path to string"))?;
        Ok(format!("let-env PATH = ($env.PATH | prepend {:?})", path))
    }

    fn set_env_var(&self, name: &str, value: &str) -> String {
        format!("let-env {} = {:?}", name, value)
    }

    fn use_on_cd(&self, _config: &crate::config::FnmConfig) -> anyhow::Result<String> {
        panic!("TODO: sypport use_on_cd for nushell")
    }
}
