use std::path::PathBuf;

pub fn path() -> PathBuf {
    let path_as_string = if cfg!(windows) {
        "Z:/_fnm_/Nothing/Should/Be/Here/installation"
    } else {
        "/dev/null/installation"
    };

    PathBuf::from(path_as_string)
}

pub fn display_name() -> &'static str {
    "system"
}
