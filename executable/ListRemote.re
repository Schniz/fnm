open Fnm;

let run = (~version as maybeVersionName) => {
  Console.log("Looking for some node versions upstream...");

  let%lwt versions = Versions.getRemoteVersions()
  and currentVersion = Versions.getCurrentVersion();

  let versions =
    switch (maybeVersionName) {
    | None => versions
    | Some(versionName) =>
      let formattedVersionName = Versions.format(versionName);
      versions
      |> Versions.(
           List.filter(v =>
             isVersionFitsPrefix(formattedVersionName, Remote.(v.name))
             || v.name == formattedVersionName
           )
         );
    };

  switch (versions) {
  | [] =>
    Console.log(
      <Pastel color=Pastel.Red>
        "No versions found that match your criterias."
      </Pastel>,
    );
    Lwt.return_error(1);

  | _ =>
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
         ();
         Console.log(<Pastel ?color> str </Pastel>);
       });
    Lwt.return_ok();
  };
};
