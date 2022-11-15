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
