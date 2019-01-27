open Fnm;

let run = isFishShell => {
  if (isFishShell) {
    Console.log(
      Printf.sprintf("set PATH %s/bin $PATH", Directories.currentVersion),
    );
  } else {
    Console.log(
      Printf.sprintf("export PATH=%s/bin:$PATH", Directories.currentVersion),
    );
  }

  Lwt.return();
};
