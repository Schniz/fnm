let version = Fnm.Fnm__Package.version;

module Commands = {
  let use = (version, quiet) => Lwt_main.run(Use.run(~version, ~quiet));
  let alias = (version, name) => Lwt_main.run(Alias.run(~name, ~version));
  let listRemote = () => Lwt_main.run(ListRemote.run());
  let listLocal = () => Lwt_main.run(ListLocal.run());
  let install = version => Lwt_main.run(Install.run(~version));
  let env = (isFishShell, isMultishell) =>
    Lwt_main.run(
      Env.run(
        ~shell=Fnm.System.Shell.(isFishShell ? Fish : Bash),
        ~multishell=isMultishell,
      ),
    );
};

open Cmdliner;

let help_secs = [
  `S(Manpage.s_common_options),
  `S(Manpage.s_environment),
  `P("These options are common to all commands."),
  `S("MORE HELP"),
  `P("Use `$(mname) $(i,COMMAND) --help' for help on a single command."),
  `Noblank,
  `S(Manpage.s_bugs),
  `P("File bug reports at https://github.com/Schniz/fnm"),
];

let envs = [
  Term.env_info(
    ~doc=
      "The root directory of fnm installations. Defaults to: "
      ++ Fnm.Directories.sfwRoot,
    "FNM_DIR",
  ),
];

let install = {
  let doc = "Install another node version";
  let man = [];

  let selectedVersion = {
    let doc = "Install another version specified in $(docv).";
    Arg.(
      value & pos(0, some(string), None) & info([], ~docv="VERSION", ~doc)
    );
  };

  (
    Term.(const(Commands.install) $ selectedVersion),
    Term.info("install", ~version, ~doc, ~exits=Term.default_exits, ~man),
  );
};

let listLocal = {
  let doc = "List all the installed versions";
  let man = [];

  (
    Term.(app(const(Commands.listLocal), const())),
    Term.info("ls", ~version, ~doc, ~exits=Term.default_exits, ~man),
  );
};

let listRemote = {
  let doc = "List all the versions upstream";
  let man = [];

  (
    Term.(app(const(Commands.listRemote), const())),
    Term.info("ls-remote", ~version, ~doc, ~exits=Term.default_exits, ~man),
  );
};

let use = {
  let doc = "Switch to another installed node version";
  let man = [];

  let quiet = {
    let doc = "Don't print stuff";
    Arg.(value & flag & info(["quiet"], ~doc));
  };

  let selectedVersion = {
    let doc = "Switch to version $(docv).\nLeave empty to look for value from `.nvmrc`";
    Arg.(
      value & pos(0, some(string), None) & info([], ~docv="VERSION", ~doc)
    );
  };

  (
    Term.(const(Commands.use) $ selectedVersion $ quiet),
    Term.info("use", ~version, ~doc, ~exits=Term.default_exits, ~man),
  );
};

let alias = {
  let doc = "Alias a version";
  let sdocs = Manpage.s_common_options;
  let man = help_secs;

  let selectedVersion = {
    let doc = "The version to be aliased";
    Arg.(
      required
      & pos(0, some(string), None)
      & info([], ~docv="VERSION", ~doc)
    );
  };

  let aliasName = {
    let doc = "The alias name";
    Arg.(
      required & pos(1, some(string), None) & info([], ~docv="NAME", ~doc)
    );
  };

  (
    Term.(const(Commands.alias) $ selectedVersion $ aliasName),
    Term.info(
      "alias",
      ~version,
      ~doc,
      ~exits=Term.default_exits,
      ~man,
      ~sdocs,
    ),
  );
};

let env = {
  let doc = "Show env configurations";
  let sdocs = Manpage.s_common_options;
  let man = help_secs;

  let isFishShell = {
    let doc = "Output an env configuration for fish shell.";
    Arg.(value & flag & info(["fish"], ~doc));
  };

  let isMultishell = {
    let doc = "Use different fnm versions for each shell";
    Arg.(value & flag & info(["multi"], ~doc));
  };

  (
    Term.(const(Commands.env) $ isFishShell $ isMultishell),
    Term.info("env", ~version, ~doc, ~exits=Term.default_exits, ~man, ~sdocs),
  );
};

let defaultCmd = {
  let doc = "Manage Node.js installations";
  let sdocs = Manpage.s_common_options;
  let man = help_secs;
  (
    Term.(ret(const(_ => `Help((`Pager, None))) $ const())),
    Term.info(
      "fnm",
      ~envs,
      ~version,
      ~doc,
      ~exits=Term.default_exits,
      ~man,
      ~sdocs,
    ),
  );
};

let _ =
  Term.eval_choice(
    defaultCmd,
    [install, use, alias, listLocal, listRemote, env],
  )
  |> Term.exit;
