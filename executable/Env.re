open Fnm;

let rec makeTemporarySymlink = () => {
  let suggestedName =
    Filename.concat(
      Filename.get_temp_dir_name(),
      "fnm-shell-"
      ++ (Random.int32(9999999 |> Int32.of_int) |> Int32.to_string),
    );

  let%lwt exists = Lwt_unix.file_exists(suggestedName);

  if (exists) {
    makeTemporarySymlink();
  } else {
    let%lwt _ =
      Lwt_unix.symlink(
        suggestedName,
        Filename.concat(Directories.defaultVersion, "installation"),
      );
    Lwt.return(suggestedName);
  };
};

let run = (~shell, ~multishell) => {
  open Lwt;

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
