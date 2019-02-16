exception Version_Not_Provided;

let versionString = fileName => {
  Lwt_io.lines_of_file(fileName)
  |> Lwt_stream.to_list
  |> Lwt.map(List.hd)
  |> Lwt.map(String.trim);
};

let getVersion = () => {
  let%lwt cwd = Lwt_unix.getcwd();
  try%lwt (versionString(Filename.concat(cwd, ".node-version"))) {
  | Unix.Unix_error(Unix.ENOENT, _, _) =>
    try%lwt (versionString(Filename.concat(cwd, ".nvmrc"))) {
    | Unix.Unix_error(Unix.ENOENT, _, _) => Lwt.fail(Version_Not_Provided)
    }
  };
};
