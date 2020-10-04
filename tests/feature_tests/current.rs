test_shell!(Bash, Zsh, Fish, PowerShell, WinCmd; {
    EvalFnmEnv::default()
        .then(ExpectCommandOutput::new(
            Call::new("fnm", vec!["current"]),
            "none",
            "currently activated version",
        ))
        .then(Call::new("fnm", vec!["install", "v8.11.3"]))
        .then(Call::new("fnm", vec!["install", "v10.10.0"]))
        .then(Call::new("fnm", vec!["use", "v8.11.3"]))
        .then(ExpectCommandOutput::new(
            Call::new("fnm", vec!["current"]),
            "v8.11.3",
            "currently activated version",
        ))
        .then(Call::new("fnm", vec!["use", "v10.10.0"]))
        .then(ExpectCommandOutput::new(
            Call::new("fnm", vec!["current"]),
            "v10.10.0",
            "currently activated version",
        ))
        .then(Call::new("fnm", vec!["use", "system"]))
        .then(ExpectCommandOutput::new(
            Call::new("fnm", vec!["current"]),
            "system",
            "currently activated version",
        ))
});
