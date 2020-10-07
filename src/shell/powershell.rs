use super::Shell;
use indoc::indoc;

#[derive(Debug)]
pub struct PowerShell;

impl Shell for PowerShell {
    fn path(&self, path: &std::path::PathBuf) -> String {
        let current_path = std::env::var_os("PATH").expect("Can't read PATH env var");
        let mut split_paths: Vec<_> = std::env::split_paths(&current_path).collect();
        split_paths.insert(0, path.to_path_buf());
        let new_path = std::env::join_paths(split_paths).expect("Can't join paths");
        self.set_env_var("PATH", new_path.to_str().expect("Can't read PATH"))
    }

    fn set_env_var(&self, name: &str, value: &str) -> String {
        format!(r#"$env:{} = "{}""#, name, value)
    }

    fn use_on_cd(&self, _config: &crate::config::FnmConfig) -> String {
        indoc!(r#"
            function Set-LocationWithFnm { param($path); Set-Location $path; If ((Test-Path .nvmrc) -Or (Test-Path .node-version)) { & fnm use } }
            Set-Alias cd_with_fnm Set-LocationWithFnm -Force
            Remove-Item alias:\cd
            New-Alias cd Set-LocationWithFnm
        "#).into()
    }
    fn into_structopt_shell(&self) -> clap::Shell {
        clap::Shell::PowerShell
    }
}
