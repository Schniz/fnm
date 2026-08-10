use log::debug;
use std::path::Path;
use std::time::{Duration, Instant};

const DEFAULT_BUDGET_MS: u64 = 5_000;
const BACKOFF_STEP_MS: u64 = 10;
const MAX_BACKOFF_MS: u64 = 100;

fn budget_from_env() -> Duration {
    let millis = std::env::var("FNM_RENAME_RETRY_TIMEOUT_MS")
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .unwrap_or(DEFAULT_BUDGET_MS);
    Duration::from_millis(millis)
}

/// On Windows a directory rename is denied while any file inside the tree has an open handle
/// that does not grant `FILE_SHARE_DELETE` -- which is what a real-time A/V scanner holds on a
/// freshly extracted `node.exe`. The scan releases it on its own, so these are transient.
#[cfg(windows)]
fn is_transient_lock(err: &std::io::Error) -> bool {
    // ERROR_ACCESS_DENIED (5), ERROR_SHARING_VIOLATION (32), ERROR_LOCK_VIOLATION (33)
    matches!(err.raw_os_error(), Some(5 | 32 | 33))
}

#[cfg(not(windows))]
fn is_transient_lock(_err: &std::io::Error) -> bool {
    false
}

/// `std::fs::rename`, retried for as long as it keeps failing with a transient lock.
///
/// The budget is read from `FNM_RENAME_RETRY_TIMEOUT_MS` (milliseconds, `0` disables retrying).
/// No error is transient outside Windows, so everywhere else this is a plain `std::fs::rename`.
pub fn rename_with_retry(from: &Path, to: &Path) -> std::io::Result<()> {
    rename_with_budget(from, to, budget_from_env())
}

/// Same as [`rename_with_retry`], but with an explicit budget so tests don't have to mutate
/// process-global environment variables.
fn rename_with_budget(from: &Path, to: &Path, budget: Duration) -> std::io::Result<()> {
    let start = Instant::now();
    let mut backoff = Duration::ZERO;

    loop {
        match std::fs::rename(from, to) {
            Ok(()) => return Ok(()),
            Err(err) => {
                if !is_transient_lock(&err) || start.elapsed() >= budget {
                    return Err(err);
                }

                debug!(
                    "Moving {} into {} is blocked ({}), retrying in {}ms",
                    from.display(),
                    to.display(),
                    err,
                    backoff.as_millis()
                );

                std::thread::sleep(backoff);
                backoff = (backoff + Duration::from_millis(BACKOFF_STEP_MS))
                    .min(Duration::from_millis(MAX_BACKOFF_MS));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn renames_without_retrying_when_nothing_blocks() {
        let root = tempfile::tempdir().expect("Can't generate a temp directory");
        let source = root.path().join("source");
        let target = root.path().join("target");
        std::fs::create_dir(&source).expect("Can't create the source directory");

        rename_with_budget(&source, &target, Duration::ZERO)
            .expect("An unblocked rename must not need the retry budget");

        assert!(target.is_dir());
    }
}

#[cfg(all(test, windows))]
mod windows_tests {
    use super::*;
    use std::fs::{File, OpenOptions};
    use std::os::windows::fs::OpenOptionsExt;
    use std::path::PathBuf;

    /// `FILE_SHARE_READ` alone deliberately withholds `FILE_SHARE_DELETE`, which is what makes
    /// the rename of the parent directory fail the way it fails behind an A/V scanner.
    const FILE_SHARE_READ: u32 = 0x0000_0001;

    fn locked_tree() -> (tempfile::TempDir, PathBuf, File) {
        let root = tempfile::tempdir().expect("Can't generate a temp directory");
        let source = root.path().join("source");
        std::fs::create_dir_all(source.join("inner")).expect("Can't create the source tree");

        let victim = source.join("inner").join("node.exe");
        std::fs::write(&victim, b"stub").expect("Can't write the stub file");
        let handle = OpenOptions::new()
            .read(true)
            .share_mode(FILE_SHARE_READ)
            .open(&victim)
            .expect("Can't open the stub file with a scanner-like share mode");

        (root, source, handle)
    }

    #[test_log::test]
    fn retries_until_the_handle_is_released() {
        let (root, source, handle) = locked_tree();
        let target = root.path().join("target");

        let releaser = std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(300));
            drop(handle);
        });

        rename_with_budget(&source, &target, Duration::from_secs(5))
            .expect("The retry budget should outlast a 300ms lock");

        releaser.join().expect("Can't join the releasing thread");
        assert!(target.join("inner").join("node.exe").exists());
    }

    #[test_log::test]
    fn does_not_retry_when_the_budget_is_zero() {
        let (root, source, handle) = locked_tree();
        let target = root.path().join("target");

        let err = rename_with_budget(&source, &target, Duration::ZERO)
            .expect_err("A zero budget must return the original error immediately");

        // Windows reports ERROR_ACCESS_DENIED (5) here; ERROR_SHARING_VIOLATION (32) is the
        // same class of transient lock, so accept either.
        assert!(
            matches!(err.raw_os_error(), Some(5 | 32)),
            "unexpected error: {err:?}"
        );

        drop(handle);
        drop(root);
    }
}
