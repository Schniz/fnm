use crate::commands;
use crate::commands::command::Command;
use crate::config::FnmConfig;
use clap::Parser;

#[derive(clap::Parser, Debug)]
pub enum SubCommand {
    /// List all remote Node.js versions
    #[clap(name = "list-remote", bin_name = "list-remote", visible_aliases = &["ls-remote"])]
    LsRemote(commands::ls_remote::LsRemote),

    /// List all locally installed Node.js versions
    #[clap(name = "list", bin_name = "list", visible_aliases = &["ls"])]
    LsLocal(commands::ls_local::LsLocal),

    /// Install a new Node.js version
    #[clap(name = "install", bin_name = "install", visible_aliases = &["i"])]
    Install(commands::install::Install),

    /// Change Node.js version
    #[clap(name = "use", bin_name = "use")]
    Use(commands::r#use::Use),

    /// Print and set up required environment variables for fnm
    ///
    /// This command generates a series of shell commands that
    /// should be evaluated by your shell to create a fnm-ready environment.
    ///
    /// Each shell has its own syntax of evaluating a dynamic expression.
    /// For example, evaluating fnm on Bash and Zsh would look like `eval "$(fnm env)"`.
    /// In Fish, evaluating would look like `fnm env | source`
    #[clap(name = "env", bin_name = "env")]
    Env(commands::env::Env),

    /// Print shell completions to stdout
    #[clap(name = "completions", bin_name = "completions")]
    Completions(commands::completions::Completions),

    /// Alias a version to a common name
    #[clap(name = "alias", bin_name = "alias")]
    Alias(commands::alias::Alias),

    /// Remove an alias definition
    #[clap(name = "unalias", bin_name = "unalias")]
    Unalias(commands::unalias::Unalias),

    /// Set a version as the default version
    ///
    /// This is a shorthand for `fnm alias VERSION default`
    #[clap(name = "default", bin_name = "default")]
    Default(commands::default::Default),

    /// Print the current Node.js version
    #[clap(name = "current", bin_name = "current")]
    Current(commands::current::Current),

    /// Run a command within fnm context
    ///
    /// Example:
    /// --------
    /// fnm exec --using=v12.0.0 node --version
    /// => v12.0.0
    #[clap(name = "exec", bin_name = "exec", verbatim_doc_comment)]
    Exec(commands::exec::Exec),

    /// Uninstall a Node.js version
    ///
    /// > Warning: when providing an alias, it will remove the Node version the alias
    /// is pointing to, along with the other aliases that point to the same version.
    #[clap(name = "uninstall", bin_name = "uninstall", visible_aliases = &["uni"])]
    Uninstall(commands::uninstall::Uninstall),
}

impl SubCommand {
    pub fn call(self, config: FnmConfig) {
        match self {
            Self::LsLocal(cmd) => cmd.call(config),
            Self::LsRemote(cmd) => cmd.call(config),
            Self::Install(cmd) => cmd.call(config),
            Self::Env(cmd) => cmd.call(config),
            Self::Use(cmd) => cmd.call(config),
            Self::Completions(cmd) => cmd.call(config),
            Self::Alias(cmd) => cmd.call(config),
            Self::Default(cmd) => cmd.call(config),
            Self::Current(cmd) => cmd.call(config),
            Self::Exec(cmd) => cmd.call(config),
            Self::Uninstall(cmd) => cmd.call(config),
            Self::Unalias(cmd) => cmd.call(config),
        }
    }
}

/// A fast and simple Node.js manager.
#[derive(clap::Parser, Debug)]
#[clap(name = "fnm", version = env!("CARGO_PKG_VERSION"), bin_name = "fnm")]
pub struct Cli {
    #[clap(flatten)]
    pub config: FnmConfig,
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

pub fn parse() -> Cli {
    Cli::parse()
}
