open Fnm;

let run = (~majorVersion as maybeMajorVersion) => {
  open Lwt.Infix;
  Console.log("Looking for some node versions upstream...");

  let forMajorVersion = maybeMajorVersion =>
    switch (maybeMajorVersion) {
    | None => (_ => true)
    | Some(majorVersion) => Versions.Remote.isMajor(majorVersion)
    };

  let%lwt versions =
    Versions.getRemoteVersions()
    >|= List.filter(forMajorVersion(maybeMajorVersion))
  and currentVersion = Versions.getCurrentVersion();

  switch (versions) {
  | [] =>
    Console.log(
      <Pastel color=Pastel.Red>
        "No versions found that match your criterias."
      </Pastel>,
    )
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
       })
  };

  Lwt.return_ok();
};
