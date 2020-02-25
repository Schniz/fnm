exception Version_Not_Installed(string);

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
