open Nsw;

let run = () => {
  Console.log(
    Printf.sprintf("export PATH=%s/bin:$PATH", Directories.currentVersion),
  );

  Lwt.return();
};
