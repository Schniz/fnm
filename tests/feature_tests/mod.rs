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
