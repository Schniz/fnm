open Fnm;

let symlinkExists = path =>
  try%lwt(Lwt_unix.lstat(path) |> Lwt.map(_ => true)) {
  | _ => Lwt.return(false)
  };

let rec makeTemporarySymlink = () => {
  let suggestedName =
    Filename.concat(
      Filename.get_temp_dir_name(),
      "fnm-shell-"
      ++ (Random.int32(9999999 |> Int32.of_int) |> Int32.to_string),
    );

  let%lwt exists = symlinkExists(suggestedName);

  if (exists) {
    let%lwt suggestedName = makeTemporarySymlink();
    Lwt.return(suggestedName);
  } else {
    let%lwt _ = Lwt_unix.symlink(Directories.defaultVersion, suggestedName);
    Lwt.return(suggestedName);
  };
};

let printUseOnCd = (~shell) =>
  switch (shell) {
  | System.Shell.Bash => {|
    __fnmcd () {
      cd $@

      if [[ -f .node-version && .node-version ]]; then
        echo "fnm: Found .node-version"
        fnm use
      elif [[ -f .nvmrc && .nvmrc ]]; then
        echo "fnm: Found .nvmrc"
        fnm use
      fi
    }

    alias cd=__fnmcd
  |}
  | Fish => {|
      function _fnm_autoload_hook --on-variable PWD --description 'Change Node version on directory change'
        status --is-command-substitution; and return
        if test -f .node-version
          echo "fnm: Found .node-version"
          fnm use
        else if test -f .nvmrc
          echo "fnm: Found .nvmrc"
          fnm use
        end
      end
    |}
  | Zsh => {|
      autoload -U add-zsh-hook
      _fnm_autoload_hook () {
        if [[ -f .node-version && -r .node-version ]]; then
          echo "fnm: Found .node-version"
          fnm use
        elif [[ -f .nvmrc && -r .nvmrc ]]; then
          echo "fnm: Found .nvmrc"
          fnm use
        fi
      }

      add-zsh-hook chpwd _fnm_autoload_hook \
        && _fnm_autoload_hook
    |}
  };

let run =
    (~forceShell, ~multishell, ~nodeDistMirror, ~fnmDir, ~useOnCd, ~logLevel) => {
  open System.Shell;

  let%lwt shell =
    switch (forceShell) {
    | None =>
      switch%lwt (System.Shell.infer()) {
      | None => Lwt.return(Bash)
      | Some(shell) => Lwt.return(shell)
      }
    | Some(shell) => Lwt.return(shell)
    };

  Random.self_init();

  let%lwt path =
    multishell
      ? makeTemporarySymlink() : Lwt.return(Directories.globalCurrentVersion);

  switch (shell) {
  | Bash
  | Zsh =>
    Printf.sprintf("export PATH=%s/bin:$PATH", path) |> Console.log;
    Printf.sprintf("export %s=%s", Config.FNM_MULTISHELL_PATH.name, path)
    |> Console.log;
    Printf.sprintf("export %s=%s", Config.FNM_DIR.name, fnmDir) |> Console.log;
    Printf.sprintf(
      "export %s=%s",
      Config.FNM_NODE_DIST_MIRROR.name,
      nodeDistMirror,
    )
    |> Console.log;
    Printf.sprintf(
      "export %s=%s",
      Config.FNM_LOGLEVEL.name,
      Fnm.LogLevel.toString(logLevel),
    )
    |> Console.log;
  | Fish =>
    Printf.sprintf("set -gx PATH %s/bin $PATH;", path) |> Console.log;
    Printf.sprintf("set -gx %s %s;", Config.FNM_MULTISHELL_PATH.name, path)
    |> Console.log;
    Printf.sprintf("set -gx %s %s;", Config.FNM_DIR.name, fnmDir)
    |> Console.log;
    Printf.sprintf(
      "set -gx %s %s",
      Config.FNM_NODE_DIST_MIRROR.name,
      nodeDistMirror,
    )
    |> Console.log;
  };

  if (useOnCd) {
    printUseOnCd(~shell) |> Console.log;
  };

  Lwt.return();
};
