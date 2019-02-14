open Fnm;

let run = (isFishShell, nodeDistMirror) => {
  if (isFishShell) {
    Printf.sprintf("set PATH %s/bin $PATH", Directories.currentVersion)
    |> Console.log;
    Printf.sprintf(
      "set %s %s",
      Config.FNM_NODE_DIST_MIRROR.name,
      nodeDistMirror,
    )
    |> Console.log;
  } else {
    Printf.sprintf("export PATH=%s/bin:$PATH", Directories.currentVersion)
    |> Console.log;
    Printf.sprintf(
      "export %s=%s",
      Config.FNM_NODE_DIST_MIRROR.name,
      nodeDistMirror,
    )
    |> Console.log;
  };

  Lwt.return();
};
