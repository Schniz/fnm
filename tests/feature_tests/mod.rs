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
