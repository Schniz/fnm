open Fnm;

exception System_Version_Not_Supported;
exception Ambiguous_Arguments;

let startsWith = (~prefix, str) =>
  Base.String.prefix(str, String.length(prefix)) != prefix;

let unsafeRun = (~cmd, ~version as maybeVersion, ~useFileVersion) => {
  let%lwt version =
    switch (maybeVersion, useFileVersion) {
    | (None, false) => Lwt.return_none
    | (Some(_), true) => Lwt.fail(Ambiguous_Arguments)
    | (None, true) => Fnm.Dotfiles.getVersion() |> Lwt.map(x => Some(x))
    | (Some(version), false) => Lwt.return_some(version)
    };
  let%lwt currentVersion =
    switch (version) {
    | None => Lwt.return(Directories.currentVersion)
    | Some(version) =>
      let%lwt matchingVersion = LocalVersionResolver.getVersion(version);
      let matchingVersionPath =
        switch (matchingVersion) {
        | Alias(path) => Versions.Aliases.toDirectory(path)
        | Local(path) => Versions.Local.toDirectory(path)
        | System => raise(System_Version_Not_Supported)
        };
      Lwt.return(matchingVersionPath);
    };
  let fnmPath = Filename.concat(currentVersion, "bin");
  let path = Opt.(Sys.getenv_opt("PATH") or "");
  let pathEnv = Printf.sprintf("PATH=%s:%s", fnmPath, path);
  let cmd = cmd |> Array.copy |> Array.append([|"env", pathEnv|]);
  let%lwt exitCode =
    Lwt_process.exec(
      ~stdin=`Keep,
      ~stdout=`Keep,
      ~stderr=`Keep,
      ~env=Unix.environment(),
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
  | Ambiguous_Arguments =>
    Console.error(
      <Pastel color=Pastel.Red>
        <Pastel bold=true> "Error: " </Pastel>
        "You passed both "
        <Pastel color=Pastel.Cyan> "--using" </Pastel>
        " and "
        <Pastel color=Pastel.Cyan> "--using-file" </Pastel>
        ".\n"
        "Please provide only one of them."
      </Pastel>,
    );
    Lwt.return_error(1);
  | System_Version_Not_Supported =>
    Console.error(
      <Pastel color=Pastel.Red>
        <Pastel bold=true> "Error: " </Pastel>
        "System version is not supported in "
        <Pastel color=Pastel.Yellow> "`fnm exec`" </Pastel>
      </Pastel>,
    );
    Lwt.return_error(1);
  | LocalVersionResolver.Version_Not_Installed(versionName) =>
    Console.error(
      <Pastel color=Pastel.Red>
        <Pastel bold=true> "Error: " </Pastel>
        "Version "
        <Pastel color=Pastel.Cyan> versionName </Pastel>
        " is not installed."
      </Pastel>,
    );
    Lwt.return_error(1);
  | Dotfiles.Version_Not_Provided =>
    Console.error(
      <Pastel color=Pastel.Red>
        "No .nvmrc or .node-version file was found in the current directory. Please provide a version number."
      </Pastel>,
    );
    Lwt.return_error(1);
  };
};
