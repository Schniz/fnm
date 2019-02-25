open Fnm;

let symlinkExists = path =>
  try%lwt (Lwt_unix.lstat(path) |> Lwt.map(_ => true)) {
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

let rec printUseOnCd = (~shell) => {
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
  | Fish =>
    Printf.sprintf("# WARNING: use-on-cd isn't implemented for Fish yet")
  | Zsh => {|
      autoload -U add-zsh-hook
      _fnm_autoload_hook () {
        if [[ -f .node-version && -r .node-version ]]; then
          echo "fnm: Found .node-version"
          fnm use
          elsif
        elif [[ -f .nvmrc && -r .nvmrc ]]; then
          echo "fnm: Found .nvmrc"
          fnm use
        fi
      }

      add-zsh-hook chpwd _fnm_autoload_hook \
        && _fnm_autoload_hook
    |}
  };
};

let run =
    (~forceShell as shell, ~multishell, ~nodeDistMirror, ~fnmDir, ~useOnCd) => {
  open Lwt;
  open System.Shell;

  let%lwt shell =
    switch (shell) {
    | None =>
      switch%lwt (System.Shell.infer()) {
      | Some(shell) => Lwt.return(shell)
      | None => Lwt.fail_with("Can't infer shell type")
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
  | Fish =>
    Printf.sprintf("set PATH %s/bin $PATH;", path) |> Console.log;
    Printf.sprintf("set %s %s;", Config.FNM_MULTISHELL_PATH.name, path)
    |> Console.log;
    Printf.sprintf("set %s %s;", Config.FNM_DIR.name, fnmDir) |> Console.log;
    Printf.sprintf(
      "set %s %s",
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
