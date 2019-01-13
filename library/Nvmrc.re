exception Version_Not_Provided;

let getVersion = () => {
  let%lwt cwd = Lwt_unix.getcwd();
  let nvmrcFile = Filename.concat(cwd, ".nvmrc");
  try%lwt (
    Lwt_io.lines_of_file(nvmrcFile)
    |> Lwt_stream.to_list
    |> Lwt.map(List.hd)
    |> Lwt.map(String.trim)
  ) {
  | Unix.Unix_error(Unix.ENOENT, _, _) => Lwt.fail(Version_Not_Provided)
  };
};
