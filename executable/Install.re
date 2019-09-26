open Fnm;

let mkDownloadsDir = () => {
  let exists = Lwt_unix.file_exists(Directories.downloads);
  if%lwt (exists |> Lwt.map(x => !x)) {
    Logger.debug(
      <Pastel>
        "Creating "
        <Pastel color=Pastel.Cyan> Directories.downloads </Pastel>
        " for the first time"
      </Pastel>,
    );
    let%lwt _ = System.mkdirp(Directories.downloads);
    Lwt.return();
  } else {
    Lwt.return();
  };
};

let download = (~version, ~filepath) => {
  let tarDestination =
    Filename.concat(
      Directories.downloads,
      version ++ Versions.Remote.downloadFileSuffix,
    );

  Logger.debug(
    <Pastel>
      "Downloading "
      <Pastel color=Pastel.Cyan> filepath </Pastel>
      " to "
      <Pastel color=Pastel.Cyan> tarDestination </Pastel>
    </Pastel>,
  );

  let%lwt _ = System.mkdirp(Filename.dirname(tarDestination));
  let%lwt _ = Http.download(filepath, ~into=tarDestination);
  let extractionDestination =
    Filename.concat(Directories.nodeVersions, version);

  Logger.debug(
    <Pastel>
      "Extracting "
      <Pastel color=Pastel.Cyan> tarDestination </Pastel>
      " to "
      <Pastel color=Pastel.Cyan> extractionDestination </Pastel>
    </Pastel>,
  );

  Logger.info(
    <Pastel>
      "Version "
      <Pastel color=Pastel.Cyan> version </Pastel>
      " was successfully downloaded"
    </Pastel>,
  );

  let%lwt _ =
    Compression.extractFile(tarDestination, ~into=extractionDestination);
  Lwt.return_unit;
};

let main = (~version as versionName) => {
  let%lwt os = System.NodeOS.get()
  and arch = System.NodeArch.get()
  and versionName =
    switch (versionName) {
    | Some(versionName) => Lwt.return(versionName)
    | None => Dotfiles.getVersion()
    };

  let versionName = Versions.format(versionName);

  let%lwt versionName =
    switch (versionName) {
    | "latest-*" =>
      switch%lwt (VersionListing.getLatestLts()) {
      | Error(err) =>
        raise(VersionListing.Problem_with_finding_latest_lts(err))
      | Ok({VersionListing.lts, _}) =>
        Printf.sprintf("latest-%s", lts) |> Lwt.return
      }
    | _ => Lwt.return(versionName)
    };

  Logger.debug(
    <Pastel>
      "Looking for node "
      <Pastel color=Pastel.Cyan> versionName </Pastel>
      " for "
      <Pastel color=Pastel.Cyan>
        {System.NodeOS.toString(os)}
        " "
        {System.NodeArch.toString(arch)}
      </Pastel>
    </Pastel>,
  );

  let%lwt (fullVersionName, filepath) =
    Versions.getFileToDownload(~version=versionName, ~os, ~arch);

  let%lwt isAlreadyInstalled = Versions.isInstalled(fullVersionName);

  let%lwt _ =
    if (isAlreadyInstalled) {
      Logger.error(
        <Pastel>
          "Version "
          <Pastel color=Pastel.Cyan> fullVersionName </Pastel>
          " is already installed."
        </Pastel>,
      );
      Lwt.return_unit;
    } else {
      download(~version=fullVersionName, ~filepath);
    };

  let%lwt _ =
    if (Base.String.is_prefix(versionName, ~prefix="latest")) {
      let%lwt _ = Alias.run(~name=versionName, ~version=fullVersionName);
      Lwt.return_unit;
    } else {
      Lwt.return_unit;
    };

  Lwt.return();
};

let run = (~version) =>
  try%lwt(main(~version)) {
  | Versions.No_Download_For_System(os, arch) =>
    Logger.error(
      <Pastel>
        "Version exists, but can't find a file for your system:\n"
        "  OS:           "
        <Pastel color=Pastel.Cyan> {System.NodeOS.toString(os)} </Pastel>
        "\n"
        "  Architecture: "
        <Pastel color=Pastel.Cyan> {System.NodeArch.toString(arch)} </Pastel>
      </Pastel>,
    );
    exit(1);
  | Versions.Version_not_found(version) =>
    Logger.error(
      <Pastel>
        "Version "
        <Pastel color=Pastel.Cyan> version </Pastel>
        " not found!"
      </Pastel>,
    );
    exit(1);
  | VersionListing.Problem_with_finding_latest_lts(
      VersionListing.Cant_find_latest_lts,
    ) =>
    Logger.error(<Pastel color=Pastel.Red> "Can't find latest LTS" </Pastel>);
    exit(1);
  | VersionListing.Problem_with_finding_latest_lts(
      VersionListing.Cant_parse_remote_version_listing(reason),
    ) =>
    Logger.error(
      <Pastel color=Pastel.Red>
        "Can't parse json of the response:\n"
        reason
      </Pastel>,
    );
    exit(1);
  };
