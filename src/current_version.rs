use crate::config::FnmConfig;
use crate::system_version;
use crate::version::Version;
use snafu::{OptionExt, ResultExt, Snafu};

pub fn current_version(config: &FnmConfig) -> Result<Option<Version>, Error> {
    let multishell_path = config.multishell_path().context(EnvNotApplied)?;

    if multishell_path.read_link().ok() == Some(system_version::path()) {
        return Ok(Some(Version::Bypassed));
    }

    if let Some(resolved_path) = std::fs::canonicalize(multishell_path).ok() {
        let installation_path = resolved_path
            .parent()
            .expect("multishell path can't be in the root");
        let file_name = installation_path
            .file_name()
            .expect("Can't get filename")
            .to_str()
            .expect("Invalid OS string");
        let version = Version::parse(file_name).context(VersionError {
            version: file_name.clone(),
        })?;
        Ok(Some(version))
    } else {
        Ok(None)
    }
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display(
        "`fnm env` was not applied in this context.\nCan't find fnm's environment variables"
    ))]
    EnvNotApplied,
    #[snafu(display("Can't read the version as a valid semver"))]
    VersionError {
        source: semver::SemVerError,
        version: String,
    },
}
