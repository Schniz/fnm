open Fnm;

let lwtIgnore = lwt => Lwt.catch(() => lwt, _ => Lwt.return());

exception Version_Not_Installed(string);

let log = (~quiet, arg) =>
  if (!quiet) {
    Console.log(arg);
  };

let switchVersion = (~version, ~quiet) => {
  open Lwt;
  let log = log(~quiet);
  let%lwt parsedVersion =
    Versions.parse(version) >>= Opt.toLwt(Version_Not_Installed(version));

  let%lwt versionPath =
    switch (parsedVersion) {
    | Local(version) => Versions.Local.toDirectory(version) |> Lwt.return
    | Alias(alias) => Versions.Aliases.toDirectory(alias) |> Lwt.return
    };

  let destination = Filename.concat(versionPath, "installation");
  let source = Directories.currentVersion;

  log(
    <Pastel>
      "Linking "
      <Pastel color=Pastel.Cyan> source </Pastel>
      " to "
      <Pastel color=Pastel.Cyan> destination </Pastel>
    </Pastel>,
  );

  let%lwt _ = Lwt_unix.unlink(Directories.currentVersion) |> lwtIgnore;
  let%lwt _ = Lwt_unix.symlink(destination, Directories.currentVersion)
  and defaultAliasExists = Lwt_unix.file_exists(Directories.defaultVersion);

  let%lwt _ =
    if (!defaultAliasExists) {
      Versions.Aliases.set(~alias="default", ~versionPath=destination);
    } else {
      Lwt.return();
    };

  log(
    <Pastel> "Using " <Pastel color=Pastel.Cyan> version </Pastel> </Pastel>,
  );

  Lwt.return();
};

let main = (~version as providedVersion, ~quiet) => {
  let%lwt version =
    switch (providedVersion) {
    | Some(version) => Lwt.return(version)
    | None => Nvmrc.getVersion()
    };
  switchVersion(~version, ~quiet);
};

let run = (~version, ~quiet) =>
  try%lwt (main(~version, ~quiet)) {
  | Version_Not_Installed(version) =>
    log(
      ~quiet,
      <Pastel color=Pastel.Red>
        "The following version is not installed: "
        version
      </Pastel>,
    )
    |> Lwt.return
  | Nvmrc.Version_Not_Provided =>
    log(
      ~quiet,
      <Pastel color=Pastel.Red>
        "No .nvmrc was found in the current directory. Please provide a version number."
      </Pastel>,
    )
    |> Lwt.return
  };
