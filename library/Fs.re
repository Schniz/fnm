open Core;

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
    try%lwt (iterate()) {
    | End_of_file => Lwt.return(items^)
    };

  let%lwt _ = Lwt_unix.closedir(dir);
  Lwt.return(items);
};

let writeFile = (path, contents) => {
  let%lwt x = Lwt_unix.openfile(path, [Unix.O_RDWR, Unix.O_CREAT], 777);
  let%lwt _ =
    Lwt.finalize(
      () => Lwt_unix.write_string(x, contents, 0, String.length(contents)),
      () => Lwt_unix.close(x),
    );
  Lwt.return();
};

let exists = path => {
  try%lwt (Lwt_unix.file_exists(path)) {
  | Unix.Unix_error(_, _, _) => Lwt.return(false)
  };
};

// Credit: https://github.com/fastpack/fastpack/blob/9f6aa7d5b83ffef03e73a15679200576ff9dbcb7/FastpackUtil/FS.re#L94
let rec rmdir = dir => {
  let%lwt files = Lwt_stream.to_list(Lwt_unix.files_of_directory(dir));
  let%lwt () =
    Lwt_list.iter_s(
      filename =>
        switch (filename) {
        | "."
        | ".." => Lwt.return_unit
        | _ =>
          let path = dir ++ "/" ++ filename;
          switch%lwt (Lwt_unix.stat(path)) {
          | {st_kind: Lwt_unix.S_DIR, _} => rmdir(path)
          | _ => Lwt_unix.unlink(path)
          };
        },
      files,
    );
  Lwt_unix.rmdir(dir);
};

let realpath = Filename.realpath;

let try_readlink = path =>
  try (Ok(Unix.readlink(path))) {
  | err => Error(err)
  };
