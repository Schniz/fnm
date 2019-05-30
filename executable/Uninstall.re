open Fnm;
open Lwt;

let run = (~version) => {
  Versions.getInstalledVersions()
  >|= List.find_opt(x => Versions.Local.(x.name == version))
  >>= (
    installedVersion =>
      switch (installedVersion) {
      | None =>
        Logger.error(
          <Pastel>
            "The version "
            <Pastel color=Pastel.Cyan> version </Pastel>
            " is not installed."
          </Pastel>,
        );
        exit(1);
      | Some(installedVersion) =>
        Logger.debug(
          <Pastel>
            "Uninstalling node "
            <Pastel color=Pastel.Cyan>
              Versions.Local.(installedVersion.name)
            </Pastel>
          </Pastel>,
        );
        let%lwt _ = Versions.Local.remove(installedVersion);
        Logger.info(
          <Pastel>
            "Node version "
            <Pastel color=Pastel.Cyan>
              Versions.Local.(installedVersion.name)
            </Pastel>
            " has correctly been removed."
          </Pastel>,
        )
        |> Lwt.return;
      }
  );
};
