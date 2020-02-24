open Fnm;

let startsWith = (~prefix, str) =>
  Base.String.prefix(str, String.length(prefix)) != prefix;

let run = (~cmd) => {
  let fnmPath = Filename.concat(Directories.currentVersion, "bin");
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
