let unix_exec =
    (~args=[||], ~env=?, ~stderr: Lwt_process.redirection=`Keep, command) => {
  let realArgs = Array.append([|command|], args);
  Lwt_process.pread_lines(~stderr, ~env?, ("", realArgs))
  |> Lwt_stream.to_list;
};

let mkdirp = destination =>
  unix_exec("mkdir", ~stderr=`Dev_null, ~args=[|"-p", destination|]);

module Shell = {
  type t =
    | Bash
    | Fish;
};

module NodeArch = {
  type t =
    | X32
    | X64
    | Other;

  let rec last = xs =>
    switch (xs) {
    | [x] => Some(x)
    | [_, ...xs] => last(xs)
    | [] => None
    };

  let findArches = unameResult => {
    let words = unameResult |> List.hd |> String.split_on_char(' ');
    List.exists(word => word == "x86_64", words) ? X64 : X32;
  };

  /* Get node-compliant architecture (x64, x86) */
  let get = () =>
    switch (Sys.os_type) {
    | "Unix" =>
      let%lwt result = unix_exec("uname", ~args=[|"-a"|]);
      try (result |> findArches |> Lwt.return) {
      | _ => Lwt.fail_with("Error getting unix information")
      };
    | _ => Lwt.return(Other)
    };

  let toString =
    fun
    | X64 => "x64"
    | X32 => "x32"
    | Other => "other";
};

module NodeOS = {
  type t =
    | Darwin
    | Linux
    | Other(string);

  let get = () =>
    switch (Sys.os_type) {
    | "Unix" =>
      let%lwt result = unix_exec("uname", ~args=[|"-s"|]);
      switch (result |> List.hd) {
      | "Darwin" => Lwt.return(Darwin)
      | _ => Lwt.return(Linux)
      | exception _ => Lwt.fail_with("Error getting unix information")
      };
    | other => Other(other) |> Lwt.return
    };

  let toString =
    fun
    | Darwin => "darwin"
    | Linux => "linux"
    | Other(_) => "other";
};

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
