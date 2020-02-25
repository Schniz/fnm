open Fnm;

let startsWith = (~prefix, str) =>
  Base.String.prefix(str, String.length(prefix)) != prefix;

let unsafeRun = (~cmd, ~version as maybeVersion, ~useFileVersion) => {
  let%lwt version =
    switch (maybeVersion, useFileVersion) {
    | (None, false) => Lwt.return_none
    | (Some(_), true) =>
      Lwt.fail_with("Please provide --using or --using-file, not both.")
    | (None, true) => Fnm.Dotfiles.getVersion() |> Lwt.map(x => Some(x))
    | (Some(version), false) => Lwt.return_some(version)
    };
  let currentVersion =
    switch (version) {
    | Some(version) => Versions.Local.toDirectory(version)
    | None => Directories.currentVersion
    };
  let fnmPath = Filename.concat(currentVersion, "bin");
  Console.log(fnmPath);
  let path = Opt.(Sys.getenv_opt("PATH") or "");
  let pathEnv = Printf.sprintf("PATH=%s:%s", fnmPath, path);
  let env =
    Unix.environment()
    |> Array.copy
    |> Base.Array.filter(~f=startsWith(~prefix="PATH="))
    |> Array.append([|pathEnv|]);
  let%lwt exitCode =
    Lwt_process.exec(
      ~stdin=`Keep,
      ~stdout=`Keep,
      ~stderr=`Keep,
      ~env,
      ("", cmd),
    );

  switch (exitCode) {
  | Unix.WEXITED(0) => Lwt.return_ok()
  | Unix.WEXITED(x)
  | Unix.WSTOPPED(x)
  | Unix.WSIGNALED(x) => Lwt.return_error(x)
  };
};

let run = (~cmd, ~version, ~useFileVersion) => {
  try%lwt(unsafeRun(~cmd, ~version, ~useFileVersion)) {
  | Dotfiles.Version_Not_Provided =>
    Console.error(
      <Pastel color=Pastel.Red>
        "No .nvmrc or .node-version file was found in the current directory. Please provide a version number."
      </Pastel>,
    );
    Lwt.return_error(1);
  };
};
