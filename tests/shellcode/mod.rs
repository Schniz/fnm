mod call;
mod die_on_errors;
mod eval_fnm_env;
mod expect_command_output;
mod expression;
mod get_stderr;
mod ignore_errors;
mod line_separated_expressions;
mod nothing;
mod output_contains;
mod raw;
mod shell;
mod sub_shell;
mod test_node_version;
mod write_file;

#[allow(unused)]
pub use call::*;
#[allow(unused)]
pub use die_on_errors::*;
#[allow(unused)]
pub use eval_fnm_env::*;
#[allow(unused)]
pub use expect_command_output::*;
#[allow(unused)]
pub use expression::*;
#[allow(unused)]
pub use get_stderr::*;
#[allow(unused)]
pub use ignore_errors::*;
#[allow(unused)]
#[allow(unused)]
pub use line_separated_expressions::*;
#[allow(unused)]
pub use nothing::*;
#[allow(unused)]
pub use output_contains::*;
#[allow(unused)]
pub use raw::*;
#[allow(unused)]
pub use shell::*;
#[allow(unused)]
pub use sub_shell::*;
#[allow(unused)]
pub use test_node_version::*;
#[allow(unused)]
pub use write_file::*;

use std::path::Path;

pub(crate) fn run_test_file(dir: &Path, shell: &impl Shell, code: &str) {
    let fnm_dir = tempfile::tempdir().unwrap();
    let target_dir = std::path::PathBuf::from(env!("CARGO_BIN_EXE_fnm"))
        .parent()
        .unwrap()
        .to_path_buf();
    let path_str = {
        let path_env = std::env::var("PATH").unwrap();
        let mut path_split: Vec<_> = std::env::split_paths(&path_env).collect();
        path_split.insert(0, target_dir);
        path_split.insert(0, dir.join("bin")); // for custom binaries
        std::env::join_paths(path_split).unwrap()
    };
    duct::cmd(shell.binary_name(), shell.launch_args())
        .env("PATH", path_str)
        .env("FNM_DIR", fnm_dir.path())
        .env("HOME", tempfile::tempdir().unwrap().path())
        .env_remove("FNM_MULTISHELL_PATH")
        .dir(dir)
        .stdin_bytes(code)
        .run()
        .unwrap();
}

#[macro_export]
macro_rules! test_shell {
    ($($shell:ident),+; $block:block) => {
        test_shell!($($shell),+; |_path| $block);
    };
    ($($shell:ident),+; $f:expr) => {
        $(
            #[test]
            #[serial_test::serial]
            #[allow(non_snake_case)]
            fn $shell() {
                use super::*;
                #[allow(unused)]
                use pretty_assertions::assert_eq;
                let current_dir = tempfile::tempdir().expect("Can't create a temp dir");
                let shell = $crate::shellcode::$shell;

                let mut source = String::new();
                empty_shell_script(&shell)
                    .then($crate::shellcode::DieOnErrors)
                    .then(($f)(current_dir.path()))
                    .write_shell(&mut source)
                    .expect("Can't create shell script");

                insta::assert_snapshot!(&source.trim());

                if !shell.currently_supported() {
                    return;
                }

                $crate::shellcode::run_test_file(
                    current_dir.path(),
                    &shell,
                    &source.trim()
                );
            }
        )+
    };
}
