open Fnm;

let run = (~name, ~version) => {
  let version = Versions.format(version);
  let versionPath =
    Filename.concat(
      Filename.concat(Directories.nodeVersions, version),
      "installation",
    );
  let%lwt versionInstalled = Lwt_unix.file_exists(versionPath);

  if (!versionInstalled) {
    Logger.error(
      <Pastel color=Pastel.Red>
        "Can't find a version installed in "
        versionPath
      </Pastel>,
    );
    exit(1);
  };

  Logger.log(
    <Pastel>
      "Aliasing "
      <Pastel color=Pastel.Cyan> name </Pastel>
      " to "
      <Pastel color=Pastel.Cyan> version </Pastel>
    </Pastel>,
  );

  let%lwt () = Versions.Aliases.set(~alias=name, ~versionPath);

  Lwt.return();
};
