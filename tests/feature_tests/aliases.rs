use crate::shellcode::*;

fn installed_versions() -> Call {
    Call::new("fnm", vec!["ls"])
}

test_shell!(Bash, Zsh, Fish, PowerShell; {
    EvalFnmEnv::default()
        .then(Call::new("fnm", vec!["install", "6.11.3"]))
        .then(Call::new("fnm", vec!["install", "8.11.3"]))
        .then(Call::new("fnm", vec!["alias", "8.11", "oldie"]))
        .then(Call::new("fnm", vec!["alias", "6", "older"]))
        .then(Call::new("fnm", vec!["default", "older"]))
        .then(OutputContains::new(
            OutputContains::new(installed_versions(), "8.11.3"),
            "oldie",
        ))
        .then(OutputContains::new(
            OutputContains::new(OutputContains::new(installed_versions(), "6.11.3"), "older"),
            "default",
        ))
        .then(Call::new("fnm", vec!["use", "older"]))
        .then(test_node_version("v6.11.3"))
        .then(Call::new("fnm", vec!["use", "oldie"]))
        .then(test_node_version("v8.11.3"))
        .then(Call::new("fnm", vec!["use", "default"]))
        .then(test_node_version("v6.11.3"))
});
