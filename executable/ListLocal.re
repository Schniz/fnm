open Fnm;

exception Cant_read_local_versions;

let main = () =>
  Versions.Local.(
    {
      let%lwt versions =
        Versions.getInstalledVersions()
        |> Result.mapError(_ => Cant_read_local_versions)
        |> Result.toLwtErr;
      let currentVersion = Versions.getCurrentVersion();

      Console.log("The following versions are installed:");

      versions
      |> Array.iter(version => {
           let color =
             switch (currentVersion) {
             | None => None
             | Some(x) when x.name == version.name => Some(Pastel.Cyan)
             | Some(_) => None
             };
           Console.log(<Pastel ?color> "* " {version.name} </Pastel>);
         });

      Lwt.return();
    }
  );

let run = () =>
  try%lwt (main()) {
  | Cant_read_local_versions =>
    Console.log("No versions installed!") |> Lwt.return
  };
