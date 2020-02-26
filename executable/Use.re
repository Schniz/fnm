open Fnm;

let lwtIgnore = lwt => Lwt.catch(() => lwt, _ => Lwt.return());

exception Version_Not_Installed(string);

let info = (~quiet, arg) =>
  if (!quiet) {
    Logger.info(arg);
  };

let debug = (~quiet, arg) =>
  if (!quiet) {
    Logger.debug(arg);
  };

let error = (~quiet, arg) =>
  if (!quiet) {
    Logger.error(arg);
  };

let getVersion = version => {
  let%lwt parsed = Versions.parse(version);
  let%lwt resultWithLts =
    switch (parsed) {
    | Ok(x) => Lwt.return_ok(x)
    | Error("latest-*") =>
      switch%lwt (VersionListingLts.getLatest()) {
      | Error(_) => Lwt.return_error(Version_Not_Installed(version))
      | Ok({VersionListingLts.lts, _}) =>
        Versions.Alias("latest-" ++ lts) |> Lwt.return_ok
      }
    | _ => Version_Not_Installed(version) |> Lwt.return_error
    };
  resultWithLts |> Result.fold(Lwt.fail, Lwt.return);
};

let switchVersion = (~version, ~quiet) => {
  let info = info(~quiet);
  let debug = debug(~quiet);
  let%lwt parsedVersion = getVersion(version);

  let%lwt versionPath =
    switch (parsedVersion) {
    | Local(version) => Versions.Local.toDirectory(version) |> Lwt.return
    | Alias(alias) => Versions.Aliases.toDirectory(alias) |> Lwt.return
    | System => Versions.Local.systemVersion.fullPath |> Lwt.return
    };

  let versionName =
    switch (parsedVersion) {
    | Local(v) => v
    | Alias(v) => v
    | System => "system"
    };

  let source = Directories.currentVersion;

  debug(
    <Pastel>
      "Linking "
      <Pastel color=Pastel.Cyan> source </Pastel>
      " to "
      <Pastel color=Pastel.Cyan> versionPath </Pastel>
    </Pastel>,
  );

  let%lwt _ = Lwt_unix.unlink(Directories.currentVersion) |> lwtIgnore;
  let%lwt _ = Lwt_unix.symlink(versionPath, Directories.currentVersion)
  and defaultAliasExists = Lwt_unix.file_exists(Directories.defaultVersion);

  let%lwt _ =
    if (!defaultAliasExists) {
      Versions.Aliases.set(~alias="default", ~versionPath);
    } else {
      Lwt.return();
    };

  info(
    <Pastel>
      "Using "
      <Pastel color=Pastel.Cyan> versionName </Pastel>
    </Pastel>,
  );

  Lwt.return();
};

let main = (~version as providedVersion, ~quiet) => {
  let%lwt version =
    switch (providedVersion) {
    | Some(version) => Lwt.return(version)
    | None => Dotfiles.getVersion()
    };
  let%lwt () = switchVersion(~version, ~quiet);
  Lwt.return_ok();
};

let rec askIfInstall = (~version, ~quiet, retry) => {
  let%lwt _ =
    Lwt_io.write(
      Lwt_io.stderr,
      <Pastel color=Pastel.Red> "Do you want to install it? [y/n] " </Pastel>,
    );
  switch%lwt (Lwt_io.read_line(Lwt_io.stdin)) {
  | "Y"
  | "y" =>
    let%lwt res = Install.run(~version);
    switch (res) {
    | Error(_) => Lwt.return_error(1)
    | Ok(_) => retry(~version, ~quiet)
    };
  | "N"
  | "n" =>
    let%lwt () = Lwt_io.write_line(Lwt_io.stderr, "Not installing!");
    Lwt.return_ok();
  | _ =>
    let%lwt _ =
      Lwt_io.write_line(Lwt_io.stderr, "Invalid response. Please try again:");
    askIfInstall(~version, ~quiet, retry);
  };
};

let rec run = (~version, ~quiet) =>
  try%lwt(main(~version, ~quiet)) {
  | Version_Not_Installed(versionString) =>
    error(
      ~quiet,
      <Pastel color=Pastel.Red>
        "The following version is not installed: "
        versionString
      </Pastel>,
    );
    if (Fnm.Config.FNM_INTERACTIVE_CLI.get()) {
      askIfInstall(~version, ~quiet, run);
    } else {
      Lwt.return_error(1);
    };
  | Dotfiles.Conflicting_Dotfiles_Found(v1, v2) =>
    error(
      ~quiet,
      <Pastel color=Pastel.Red>
        "Can't infer version from dotfiles: .node-version and .nvmrc have differing version strings:\n"
        "  * "
        v1
        "\n"
        "  * "
        v2
      </Pastel>,
    );
    Lwt.return_error(1);
  | Dotfiles.Version_Not_Provided =>
    error(
      ~quiet,
      <Pastel color=Pastel.Red>
        "No .nvmrc or .node-version file was found in the current directory. Please provide a version number."
      </Pastel>,
    );
    Lwt.return_error(1);
  };
