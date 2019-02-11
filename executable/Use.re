open Fnm;

let lwtIgnore = lwt => Lwt.catch(() => lwt, _ => Lwt.return());

exception Version_Not_Installed(string);

let switchVersion = version => {
  let versionDir = Filename.concat(Directories.nodeVersions, version);

  let%lwt _ =
    if%lwt (Lwt_unix.file_exists(versionDir) |> Lwt.map(x => !x)) {
      Lwt.fail(Version_Not_Installed(version));
    };

  let destination = Filename.concat(versionDir, "installation");
  let source = Directories.currentVersion;

  Console.log(
    <Pastel>
      "Linking "
      <Pastel color=Pastel.Cyan> source </Pastel>
      " to "
      <Pastel color=Pastel.Cyan> destination </Pastel>
    </Pastel>,
  );

  let%lwt _ = Lwt_unix.unlink(Directories.currentVersion) |> lwtIgnore;
  let%lwt _ = Lwt_unix.symlink(destination, Directories.currentVersion);

  Console.log(
    <Pastel> "Using " <Pastel color=Pastel.Cyan> version </Pastel> </Pastel>,
  );

  Lwt.return();
};

let main = (~version as providedVersion) => {
  let%lwt version =
    switch (providedVersion) {
    | Some(version) => Lwt.return(version)
    | None => Nvmrc.getVersion()
    };
  switchVersion(Versions.format(version));
};

let run = (~version) =>
  try%lwt (main(~version)) {
  | Version_Not_Installed(version) =>
    Console.log(
      <Pastel color=Pastel.Red>
        "The following version is not installed: "
        version
      </Pastel>,
    )
    |> Lwt.return
  | Nvmrc.Version_Not_Provided =>
    Console.log(
      <Pastel color=Pastel.Red>
        "No .nvmrc was found in the current directory. Please provide a version number."
      </Pastel>,
    )
    |> Lwt.return
  };
