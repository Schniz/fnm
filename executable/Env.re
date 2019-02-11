open Fnm;

let run = shell => {
  let env =
    switch (shell) {
    | System.Shell.Bash =>
      Printf.sprintf("export PATH=%s/bin:$PATH", Directories.currentVersion)
    | System.Shell.Fish =>
      Printf.sprintf("set PATH %s/bin $PATH", Directories.currentVersion)
    };

  Console.log(env) |> Lwt.return;
};
