open Fnm;

let rec getTempFileName = () => {
  let suggestedName =
    Filename.concat(
      Filename.get_temp_dir_name(),
      "fnm-shell-"
      ++ (Random.int32(9999999 |> Int32.of_int) |> Int32.to_string),
    );

  let%lwt exists = Lwt_unix.file_exists(suggestedName);

  if (exists) {
    getTempFileName();
  } else {
    Lwt.return(suggestedName);
  };
};

let run = (~shell, ~multishell) => {
  let%lwt path =
    multishell
      ? getTempFileName() : Lwt.return(Directories.globalCurrentVersion);

  switch (shell) {
  | System.Shell.Bash =>
    Printf.sprintf("export PATH=%s/bin:$PATH", path) |> Console.log;
    Printf.sprintf("export FNM_MULTISHELL_PATH=%s", path) |> Console.log;
  | System.Shell.Fish =>
    Printf.sprintf("set PATH %s/bin $PATH", path) |> Console.log;
    Printf.sprintf("set FNM_MULTISHELL_PATH %s", path) |> Console.log;
  };

  Lwt.return();
};