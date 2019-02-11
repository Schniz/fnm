open Fnm;

let lwtIgnore = lwt => Lwt.catch(() => lwt, _ => Lwt.return());

let run = (~name, ~version) => {
  let version = Versions.format(version);
  let versionPath = Filename.concat(Directories.nodeVersions, version);
  let%lwt versionInstalled = Lwt_unix.file_exists(versionPath);

  if (!versionInstalled) {
    Console.error(
      <Pastel color=Pastel.Red>
        "Can't find a version installed in "
        versionPath
      </Pastel>,
    );
    exit(1);
  };

  Console.log(
    <Pastel>
      "Aliasing "
      <Pastel color=Pastel.Cyan> name </Pastel>
      " to "
      <Pastel color=Pastel.Cyan> version </Pastel>
    </Pastel>,
  );

  let aliasPath = Filename.concat(Directories.aliases, name);

  let%lwt _ = System.mkdirp(Directories.aliases);
  let%lwt _ = Lwt_unix.unlink(aliasPath) |> lwtIgnore;
  let%lwt _ = Lwt_unix.symlink(versionPath, aliasPath);

  Lwt.return();
};
