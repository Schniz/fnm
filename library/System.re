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
    | Zsh
    | Fish;

  let infer = () => {
    let processInfo = pid => {
      switch%lwt (unix_exec("ps", ~args=[|"-o", "ppid,comm", pid|])) {
      | [] => Lwt.return_none
      | [_headers, line, ..._otherLines] =>
        let psResult = String.split_on_char(' ', line |> String.trim);
        let parentPid = List.nth(psResult, 0);
        let executable = List.nth(psResult, 1) |> Filename.basename;
        Lwt.return_some((parentPid, executable));
      | [_, ..._] => Lwt.return_none
      };
    };

    let rec getShell = (~level=0, pid) => {
      switch%lwt (processInfo(pid)) {
      | Some((_, "sh"))
      | Some((_, "-sh"))
      | Some((_, "-bash"))
      | Some((_, "bash")) => Lwt.return_some(Bash)
      | Some((_, "-zsh"))
      | Some((_, "zsh")) => Lwt.return_some(Zsh)
      | Some((_, "fish"))
      | Some((_, "-fish")) => Lwt.return_some(Fish)
      | Some((ppid, _)) when level < 10 => getShell(~level=level + 1, ppid)
      | Some(_)
      | None => Lwt.return_none
      };
    };

    getShell(Unix.getpid() |> string_of_int);
  };
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
