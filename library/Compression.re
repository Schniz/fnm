let extractFile = (~into as destination, filepath) => {
  let%lwt _ = System.mkdirp(destination);
  let%lwt _ =
    System.unix_exec(
      "tar",
      ~args=[|"-xvf", filepath, "--directory", destination|],
      ~stderr=`Dev_null,
    );
  let%lwt files = Fs.readdir(destination);
  let filename = List.hd(files);
  Lwt_unix.rename(
    Filename.concat(destination, filename),
    Filename.concat(destination, "installation"),
  );
};
