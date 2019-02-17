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
  | (Some(nodeVersionString), Some(nvmrcString)) =>
    if (nodeVersionString === nvmrcString) {
      Lwt.return(nodeVersionString);
    } else {
      Lwt.fail_with(
        "You have both .node-version and .nvmrc with differing version strings!",
      );
    }
  | (Some(version), None)
  | (None, Some(version)) => Lwt.return(version)
  };
};
