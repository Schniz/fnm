use crate::shell::Shell;
use std::process::Command;

#[cfg(windows)]
pub fn format_path(path: &str, shell: &str) -> String {
    match shell {
        "cmd" | "pwsh" => String::from(path),
        _ => {
            let output = Command::new("cygpath")
                .arg(path)
                .output()
                .expect("failed to convert Windows path to UNIX");

            match std::str::from_utf8(output.stdout.as_slice()) {
                Ok(s) => String::from(s.trim()),
                Err(_) => panic!("invalid UTF-8 returned from cygpath")
            }
        }
    }
}

#[cfg(unix)]
pub fn format_path(path: &str, shell: Box<dyn Shell>) -> String {
    String::from(path)
}
