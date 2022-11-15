mod aliases;
mod current;
mod uninstall;

use crate::shellcode::*;

// mod node_dist_mirror {
// test_shell!(Zsh, Bash, Fish, PowerShell, {
//     EvalFnmEnv::default()
//         .node_dist_mirror(Some("https://npm.taobao.org/mirrors/node"))
//         .then(Call::new("fnm", vec!["install", "v8.11.3"]))
//         .then(Call::new("fnm", vec!["use", "v8.11.3"]))
//         .then(test_node_version("v8.11.3"))
// });
// }

mod system_node {
    test_shell!(Bash, Zsh, Fish, PowerShell; |path: &std::path::Path| {
        use std::io::Write;
        let custom_node_dir = path.join("bin");
        std::fs::create_dir(&custom_node_dir).unwrap();
        std::fs::write(custom_node_dir.join("node.cmd"), b"echo custom node").unwrap();
        let mut f = std::fs::File::create(custom_node_dir.join("node")).unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut permissions = f.metadata().unwrap().permissions();
            permissions.set_mode(0o766);
            f.set_permissions(permissions).expect("Can't set file permissions");
        }
        writeln!(f, "#!/bin/sh").expect("Can't write file");
        writeln!(f, r#"echo "custom node""#).expect("Can't write file");

        EvalFnmEnv::default()
            .then(Call::new("fnm", vec!["install", "10"]))
            .then(Call::new("fnm", vec!["use", "10"]))
            .then(Call::new("fnm", vec!["use", "system"]))
            .then(test_node_version("custom node"))
    });
}

mod use_nvmrc_lts {
    test_shell!(Bash, Zsh, Fish, PowerShell; {
        EvalFnmEnv::default()
            .then(WriteFile::new(".nvmrc", "lts/dubnium"))
            .then(Call::new("fnm", vec!["install"]))
            .then(Call::new("fnm", vec!["use"]))
            .then(OutputContains::new(Call::new("fnm", vec!["ls"]), "lts-dubnium"))
    });
}

mod partial_semver {
    test_shell!(Bash, Zsh, Fish, PowerShell, WinCmd; {
        EvalFnmEnv::default()
            .then(Call::new("fnm", vec!["install", "6"])) // unsupported version, no new versions should be issued
            .then(Call::new("fnm", vec!["use", "6"]))
            .then(test_node_version("v6.17.1"))
    });
}

mod log_level_quiet {
    test_shell!(Bash, Zsh, Fish, PowerShell; {
        EvalFnmEnv::default()
            .log_level(Some("quiet"))
            .then(ExpectCommandOutput::new(Call::new("fnm", vec!["install", "v8.11.3"]), "", "fnm install"))
            .then(ExpectCommandOutput::new(Call::new("fnm", vec!["use", "v8.11.3"]), "", "fnm use"))
            .then(ExpectCommandOutput::new(Call::new("fnm", vec!["alias", "v8.11.3", "something"]), "", "fnm alias"))
    });
}

mod log_level_error {
    test_shell!(Bash, Zsh, Fish, PowerShell; {
        EvalFnmEnv::default()
            .log_level(Some("error"))
            .then(ExpectCommandOutput::new(Call::new("fnm", vec!["install", "v8.11.3"]).then(Call::new("echo", vec!["empty"])), "empty", "fnm install"))
            .then(ExpectCommandOutput::new(Call::new("fnm", vec!["use", "v8.11.3"]).then(Call::new("echo", vec!["empty"])), "empty", "fnm use"))
            .then(ExpectCommandOutput::new(Call::new("fnm", vec!["alias", "v8.11.3", "something"]).then(Call::new("echo", vec!["empty"])), "empty", "fnm alias"))
            .then(OutputContains::new(IgnoreErrors::new(GetStderr::new(Call::new("fnm", vec!["alias", "abcd", "efg"]))), "Can't find requested version"))
    });
}

mod list_local_with_nothing_installed {
    test_shell!(Bash, Zsh, Fish, PowerShell, WinCmd; {
        EvalFnmEnv::default()
            .then(Call::new("fnm", vec!["ls"]))
    });
}

mod latest_lts {
    test_shell!(Bash, Zsh, Fish, PowerShell; {
        EvalFnmEnv::default()
            .then(Call::new("fnm", vec!["install", "--lts"]))
            .then(OutputContains::new(Call::new("fnm", vec!["ls"]), "lts-latest"))
            .then(Call::new("fnm", vec!["use", "'lts/*'"]))
    });
}

mod matching_dotfiles {
    test_shell!(Bash, Zsh, Fish, PowerShell, WinCmd; {
        EvalFnmEnv::default()
            .then(WriteFile::new(".nvmrc", "11.10.0"))
            .then(WriteFile::new(".node-version", "11.10.0"))
            .then(Call::new("fnm", vec!["install"]))
            .then(Call::new("fnm", vec!["use"]))
            .then(test_node_version("v11.10.0"))
    });
}

mod use_alias_install_if_missing {
    test_shell!(Bash, Zsh, Fish, PowerShell; {
        EvalFnmEnv::default()
            .then(WriteFile::new(".node-version", "lts/*"))
            .then(Call::new("fnm", vec!["use", "--install-if-missing"]))
            .then(OutputContains::new(Call::new("fnm", vec!["ls"]), "lts-latest"))
    });
}

mod use_alias_not_installed {
    test_shell!(Bash, Zsh, Fish, PowerShell; {
        EvalFnmEnv::default()
            .log_level(Some("error"))
            .then(WriteFile::new(".node-version", "lts/*"))
            .then(OutputContains::new(IgnoreErrors::new(GetStderr::new(Call::new("fnm", vec!["use"]))),"Requested version lts-latest is not currently installed"))
    });
}

mod unalias {
    test_shell!(Bash, Zsh, Fish, PowerShell; {
        EvalFnmEnv::default()
            .then(Call::new("fnm", vec!["install", "11.10.0"]))
            .then(Call::new("fnm", vec!["install", "8.11.3"]))
            .then(Call::new("fnm", vec!["alias", "8.11.3", "version8"]))
            .then(OutputContains::new(Call::new("fnm", vec!["ls"]), "version8"))
            .then(Call::new("fnm", vec!["unalias", "version8"]))
            .then(OutputContains::new(IgnoreErrors::new(GetStderr::new(Call::new("fnm", vec!["use", "version8"]))),  "Requested version version8 is not currently installed"))
    });
}

mod unalias_error {
    test_shell!(Bash, Zsh, Fish, PowerShell; {
        EvalFnmEnv::default()
            .log_level(Some("error"))
            .then(OutputContains::new(IgnoreErrors::new(GetStderr::new(Call::new("fnm", vec!["unalias", "lts"]))),  "Requested alias lts not found"))
    });
}

mod alias_system {
    test_shell!(Bash, Zsh, Fish, PowerShell; {
        EvalFnmEnv::default()
            .then(Call::new("fnm", vec!["alias", "system", "my_system"]))
            .then(OutputContains::new(Call::new("fnm", vec!["ls"]), "my_system"))
            .then(Call::new("fnm", vec!["alias", "system", "default"]))
            .then(Call::new("fnm", vec!["alias", "my_system", "my_system2"]))
            .then(OutputContains::new(Call::new("fnm", vec!["ls"]), "my_system2"))
            .then(OutputContains::new(Call::new("fnm", vec!["use", "my_system"]), "Bypassing fnm"))
            .then(Call::new("fnm", vec!["unalias", "my_system"]))
            .then(OutputContains::new(IgnoreErrors::new(GetStderr::new(Call::new("fnm", vec!["use", "my_system"]))),  "Requested version my_system is not currently installed"))
    });
}
