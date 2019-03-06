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

let realpath = Filename.realpath;
