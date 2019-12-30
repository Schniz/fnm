open Fnm;

let run = () => {
  Console.log("Looking for some node versions upstream...");

  let%lwt versions = Versions.getRemoteVersions()
  and currentVersion = Versions.getCurrentVersion();

  versions
  |> List.iter(version => {
       open Versions.Remote;
       let str = "* " ++ version.name;
       let color =
         switch (currentVersion, version.installed) {
         | (Some({name: currentVersionName, _}), _)
             when currentVersionName == version.name =>
           Some(Pastel.Cyan)
         | (_, true) => Some(Pastel.Green)
         | (_, false) => None
         };

       Console.log(<Pastel ?color> str </Pastel>);
     });

  Lwt.return_ok();
};
