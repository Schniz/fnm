open Fnm;

let symlinkExists = path => {
  try%lwt (Lwt_unix.lstat(path) |> Lwt.map(_ => true)) {
  | _ => Lwt.return(false)
  };
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
    let%lwt _ =
      Lwt_unix.symlink(
        Filename.concat(Directories.defaultVersion, "installation"),
        suggestedName,
      );
    Lwt.return(suggestedName);
  };
};

let run = (~shell, ~multishell) => {
  open Lwt;

  Random.self_init();

  let%lwt path =
    multishell
      ? makeTemporarySymlink() : Lwt.return(Directories.globalCurrentVersion);

  switch (shell) {
  | System.Shell.Bash =>
    Printf.sprintf("export PATH=%s/bin:$PATH", path) |> Console.log;
    Printf.sprintf("export FNM_MULTISHELL_PATH=%s", path) |> Console.log;
  | System.Shell.Fish =>
    Printf.sprintf("set -e FNM_MULTISHELL_PATH;") |> Console.log;
    Printf.sprintf("set -x PATH %s/bin $PATH;", path) |> Console.log;
    Printf.sprintf("set -x FNM_MULTISHELL_PATH %s;", path) |> Console.log;
  };

  Lwt.return();
};
