exception Version_Not_Provided;

let versionString = fileName => {
  try%lwt (
    Lwt_io.lines_of_file(fileName)
    |> Lwt_stream.to_list
    |> Lwt.map(List.hd)
    |> Lwt.map(String.trim)
    |> Lwt.map(versionString => Some(versionString))
  ) {
  | Unix.Unix_error(Unix.ENOENT, _, _) => Lwt.return(None)
  };
};

let getVersion = () => {
  let%lwt cwd = Lwt_unix.getcwd();
  let%lwt nodeVersion = versionString(Filename.concat(cwd, ".node-version"))
  and nvmrc = versionString(Filename.concat(cwd, ".nvmrc"));

  switch (nodeVersion, nvmrc) {
  | (None, None) => Lwt.fail(Version_Not_Provided)
  | (Some(v1), Some(v2)) when v1 != v2 =>
    Lwt.fail_with(
      "You have both .node-version and .nvmrc with differing version strings!",
    )
  | (Some(version), Some(_))
  | (Some(version), None)
  | (None, Some(version)) => Lwt.return(version)
  };
};
