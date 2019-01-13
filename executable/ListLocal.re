open Nsw;

let run = () =>
  Versions.Local.(
    {
      let%lwt versions = Versions.getInstalledVersions() |> Result.toLwt;
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
