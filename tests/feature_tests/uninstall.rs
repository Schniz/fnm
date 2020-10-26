#[allow(unused_imports)]
use crate::shellcode::*;

test_shell!(Bash, Zsh, Fish, PowerShell; {
    EvalFnmEnv::default()
        .then(Call::new("fnm", vec!["install", "12.0.0"]))
        .then(Call::new("fnm", vec!["alias", "12.0.0", "hello"]))
        .then(OutputContains::new(
            OutputContains::new(Call::new("fnm", vec!["ls"]), "v12.0.0"),
            "hello",
        ))
        .then(Call::new("fnm", vec!["uninstall", "hello"]))
        .then(ExpectCommandOutput::new(Call::new("fnm", vec!["ls"]), "* system", "fnm ls"))
});
