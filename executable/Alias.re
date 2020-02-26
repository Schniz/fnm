open Fnm;

let run = (~name, ~version) => {
  let version = Versions.format(version);
  let%lwt matchingLocalVersions =
    LocalVersionResolver.getMatchingLocalVersions(version);

  switch (Base.List.hd(matchingLocalVersions)) {
  | Some(latestMatchingLocalVersion) =>
    Logger.info(
      <Pastel>
        "Aliasing "
        <Pastel color=Pastel.Cyan> name </Pastel>
        " to "
        <Pastel color=Pastel.Cyan> {latestMatchingLocalVersion.name} </Pastel>
      </Pastel>,
    );

    let%lwt () =
      Versions.Aliases.set(
        ~alias=name,
        ~versionPath=latestMatchingLocalVersion.fullPath,
      );
    Lwt.return_ok();
  | None =>
    Logger.error(
      <Pastel color=Pastel.Red>
        "No installed versions found that match your criteria."
      </Pastel>,
    );
    Lwt.return_error(1);
  };
};
