open Fnm;
open Lwt.Infix;

let run = (~version) => {
  let%lwt installedVersions = Versions.getInstalledVersions();

  let formattedVersionName = Versions.format(version);
  let matchingLocalVersions =
    installedVersions
    |> Versions.(
         List.filter(v =>
           isVersionFitsPrefix(formattedVersionName, Local.(v.name))
           || v.name == formattedVersionName
         )
       );

  switch (matchingLocalVersions) {
  | [] =>
    Logger.error(
      <Pastel>
        "The version "
        <Pastel color=Pastel.Cyan> version </Pastel>
        " is not installed."
      </Pastel>,
    );
    Lwt.return_error(1);
  | [installedVersion] =>
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
    );
    Lwt.return_ok();
  | _ =>
    Logger.info(
      <Pastel>
        "There are multiple versions matching your criterias:"
      </Pastel>,
    );
    matchingLocalVersions
    |> List.iter(matchingVersion =>
         Logger.info(
           <Pastel color=Pastel.Cyan>
             Versions.Local.(matchingVersion.name)
           </Pastel>,
         )
       );
    Logger.info(
      <Pastel color=Pastel.Red>
        "\nPlease run the command again with the correct version."
      </Pastel>,
    );
    Lwt.return_error(1);
  };
};
