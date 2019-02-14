open Fnm;

exception Cant_read_local_versions;

let main = () =>
  Versions.Local.(
    {
      let%lwt versions =
        try%lwt (Versions.getInstalledVersions()) {
        | _ => Lwt.fail(Cant_read_local_versions)
        };
      let currentVersion = Versions.getCurrentVersion();

      Console.log("The following versions are installed:");

      versions
      |> List.iter(version => {
           let color =
             switch (currentVersion) {
             | None => None
             | Some(x) when x.name == version.name => Some(Pastel.Cyan)
             | Some(_) => None
             };
           let aliases =
             List.length(version.aliases) === 0
               ? ""
               : Printf.sprintf(
                   " (%s)",
                   version.aliases |> String.concat(", "),
                 );
           Console.log(<Pastel ?color> "* " {version.name} aliases </Pastel>);
         });

      Lwt.return();
    }
  );

let run = () =>
  try%lwt (main()) {
  | Cant_read_local_versions =>
    Console.log("No versions installed!") |> Lwt.return
  };
