let readdir = dir => {
  let items = ref([]);
  let%lwt dir = Lwt_unix.opendir(dir);
  let iterate = () => {
    let%lwt _ =
      while%lwt (true) {
        let%lwt value = Lwt_unix.readdir(dir);
        if (value.[0] != '.') {
          items := [value, ...items^];
        };
        Lwt.return();
      };

    Lwt.return([]);
  };

  let%lwt items =
    try%lwt(iterate()) {
    | End_of_file => Lwt.return(items^)
    };

  let%lwt _ = Lwt_unix.closedir(dir);
  Lwt.return(items);
};

let writeFile = (path, contents) => {
  let%lwt targetFile = Lwt_io.open_file(~mode=Lwt_io.Output, path);

  let%lwt () = Lwt_io.write(targetFile, contents);
  let%lwt () = Lwt_io.close(targetFile);

  Lwt.return();
};

let exists = path => {
  try%lwt(Lwt_unix.file_exists(path)) {
  | Unix.Unix_error(_, _, _) => Lwt.return(false)
  };
};

let readlink = path =>
  try%lwt(Lwt_unix.readlink(path) |> Lwt.map(x => Ok(x))) {
  | err => Lwt.return_error(err)
  };

[@deriving show]
type file_type =
  | File(string)
  | Dir(string);

// Based on: https://github.com/fastpack/fastpack/blob/9f6aa7d5b83ffef03e73a15679200576ff9dbcb7/FastpackUtil/FS.re#L94
let rec listDirRecursively = dir => {
  switch%lwt (Lwt_unix.lstat(dir)) {
  | {st_kind: Lwt_unix.S_DIR, _} =>
    Lwt_unix.files_of_directory(dir)
    |> Lwt_stream.map_list_s(
         fun
         | "."
         | ".." => Lwt.return([])
         | filename => Filename.concat(dir, filename) |> listDirRecursively,
       )
    |> Lwt_stream.to_list
    |> Lwt.map(xs => List.append(xs, [Dir(dir)]))
  | _ => Lwt.return([File(dir)])
  | exception (Unix.Unix_error(Unix.ENOENT, _, _)) => Lwt.return([])
  };
};

let rmdir = dir => {
  let%lwt entities = listDirRecursively(dir);

  entities
  |> Lwt_list.map_s(
       fun
       | Dir(dir) => Lwt_unix.rmdir(dir)
       | File(file) => Lwt_unix.unlink(file),
     )
  |> Lwt.map(_ => ());
};

type path =
  | Exists(string)
  | Missing(string);

let rec realpath = path => {
  switch%lwt (readlink(path)) {
  | Ok(path) => realpath(path)
  | Error(_) =>
    switch%lwt (exists(path)) {
    | true => Exists(path) |> Lwt.return
    | false => Missing(path) |> Lwt.return
    }
  };
};
