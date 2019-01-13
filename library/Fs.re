open Core;

let readdir = dir =>
  switch (Sys.readdir(dir)) {
  | x => Ok(x)
  | exception (Sys_error(error)) => Error(error)
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

let realpath = Filename.realpath;
