open Fnm;
open Lwt;

let run = (~version) => {
  let versionName =
    switch (version) {
    | None =>
      Logger.log(<Pastel> "You should provide a node version" </Pastel>);
      exit(1);
    | Some(versionName) => versionName
    };

  let%lwt _ = {
    let%lwt installedVersion =
      Versions.getInstalledVersions()
      >|= List.find_opt(x => Versions.Local.(x.name == versionName));
    switch (installedVersion) {
    | None =>
      Logger.log(
        <Pastel>
          "The version "
          <Pastel color=Pastel.Cyan> versionName </Pastel>
          " of node is not installed."
        </Pastel>,
      );
      exit(1);
    | Some(installedVersion) =>
      {
        Logger.log(
          <Pastel>
            "Uninstalling node "
            <Pastel color=Pastel.Cyan>
              Versions.Local.(installedVersion.name)
            </Pastel>
          </Pastel>,
        );
      };
      let%lwt _ = Fs.rmdir(Versions.Local.(installedVersion.fullPath));
      Logger.log(
        <Pastel>
          "Node version "
          <Pastel color=Pastel.Cyan>
            Versions.Local.(installedVersion.name)
          </Pastel>
          " has correctly been removed."
        </Pastel>,
      )
      |> Lwt.return;
    };
  };

  Lwt.return();
};