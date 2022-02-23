use crate::config::FnmConfig;
use crate::directory_portal::DirectoryPortal;
use crate::version::Version;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Can't spawn `node-build`. Do you have it installed? {source}")]
    SpawningNodeBuild {
        #[source]
        source: std::io::Error,
    },
    #[error("Can't access `node-build` stdout. Please retry.")]
    NodeBuildStdoutIsInaccessible,
    #[error(
        "`node-build` has failed.\n\n   stdout:\n{}\n\n   stderr: \n{}",
        stdout,
        stderr
    )]
    ExecutingNodeBuild { stdout: String, stderr: String },
    #[error("Can't move built version to {target}. {source}")]
    MovingBuiltVersion {
        target: std::path::PathBuf,
        #[source]
        source: std::io::Error,
    },
}

pub fn compile_node_with_node_build(config: &FnmConfig, version: &Version) -> Result<(), Error> {
    let path = {
        let mut path = version.installation_path(config);
        path.pop();
        path
    };
    let temp_installations_dir = tempfile::tempdir().unwrap();
    let portal = DirectoryPortal::new_in(&temp_installations_dir, &path);
    let installation_path = portal.join("installation");

    let portal_path_fixed = installation_path.to_str().unwrap().replace(' ', "\\ ");
    let version_str = version.v_str();
    let version_str = version_str.strip_prefix("v").unwrap();

    log::debug!(
        "Going to run `node-build` to compile node {} in {}",
        version_str,
        portal_path_fixed
    );

    let mut build_process = Command::new("node-build")
        .arg(version_str)
        .arg(portal_path_fixed)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .stdin(Stdio::inherit())
        .spawn()
        .map_err(|source| Error::SpawningNodeBuild { source })?;

    let stdout = build_process
        .stdout
        .take()
        .ok_or(Error::NodeBuildStdoutIsInaccessible)?;

    let stderr = build_process
        .stderr
        .take()
        .ok_or(Error::NodeBuildStdoutIsInaccessible)?;

    let stdout_reader = read_output("node-build stdout", stdout);
    let stderr_reader = read_output("node-build stderr", stderr);

    let [stdout_reader, stderr_reader] = [stdout_reader, stderr_reader].map(|thread| {
        thread.join().map_err(|_source| Error::SpawningNodeBuild {
            source: std::io::Error::from(std::io::ErrorKind::UnexpectedEof),
        })
    });

    let exit_status = build_process
        .wait()
        .map_err(|source| Error::SpawningNodeBuild { source })?;

    if !exit_status.success() {
        return Err(Error::ExecutingNodeBuild {
            stdout: stdout_reader.unwrap_or("[error reading]".to_string()),
            stderr: stderr_reader.unwrap_or("[error reading]".to_string()),
        });
    }

    portal
        .teleport()
        .map_err(|source| Error::MovingBuiltVersion {
            target: path.to_path_buf(),
            source,
        })?;

    Ok(())
}

fn read_output<'a, Reader: 'static + std::io::Read + Sync + Send>(
    tag: &'static str,
    stream: Reader,
) -> std::thread::JoinHandle<String> {
    std::thread::spawn(move || {
        let mut buffer = String::new();
        let bufread = BufReader::new(stream);
        for line in bufread.lines().flatten() {
            log::info!("[{}] {}", tag, line);
            buffer.push_str(&line);
            buffer.push_str("\n");
        }
        buffer
    })
}
