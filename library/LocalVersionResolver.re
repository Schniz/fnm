exception Version_Not_Installed(string);

/** Parse a local version, including lts and aliases */
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

/**
 * Get matches for all versions that match a semver partial
 */
let getMatchingLocalVersions = version => {
  open Versions.Local;

  let%lwt installedVersions = Versions.getInstalledVersions();
  let formattedVersionName = Versions.format(version);

  let matchingVersions =
    installedVersions
    |> List.filter(v =>
         Versions.isVersionFitsPrefix(formattedVersionName, v.name)
         || v.name == formattedVersionName
       )
    |> List.sort((a, b) => - compare(a.name, b.name));

  Lwt.return(matchingVersions);
};
