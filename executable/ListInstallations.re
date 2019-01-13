module Path = {
  let rec join = xs =>
    switch (xs) {
    | [x] => x
    | [x, ...xs] => Filename.concat(x, join(xs))
    | [] => ""
    };
};

module Fs = {
  open Core;
  let readdir = dir =>
    switch (Sys.readdir(dir)) {
    | x => Ok(x)
    | exception (Sys_error(error)) => Error(error)
    };

  let realpath = Filename.realpath;
};

module Result = {
  let return = x => Ok(x);

  let both = (a, b) =>
    switch (a, b) {
    | (Error(_) as e, _)
    | (_, Error(_) as e) => e
    | (Ok(ax), Ok(bx)) => Ok((ax, bx))
    };

  let map = (fn, res) =>
    switch (res) {
    | Ok(x) => Ok(fn(x))
    | Error(_) as e => e
    };

  let bind = (fn, res) =>
    switch (res) {
    | Ok(x) => fn(x)
    | Error(_) as e => e
    };

  let fold = (error, ok, res) =>
    switch (res) {
    | Ok(x) => ok(x)
    | Error(x) => error(x)
    };

  module Let_syntax = {
    let map = (x, ~f) => map(f, x);
    let bind = (x, ~f) => bind(f, x);
  };
};

module Opt = {
  let orThrow = (message, opt) =>
    switch (opt) {
    | None => failwith(message)
    | Some(x) => x
    };

  let fold = (none, some, opt) =>
    switch (opt) {
    | None => none()
    | Some(x) => some(x)
    };

  let toResult = (error, opt) =>
    switch (opt) {
    | None => Error(error)
    | Some(x) => Ok(x)
    };
};

module Directories = {
  open Core;
  let home =
    Sys.getenv("HOME")
    |> Opt.orThrow("There isn't $HOME environment variable set.");
  let sfwRoot = Path.join([home, ".nsw"]);
  let nodeVersions = Path.join([sfwRoot, "node-versions"]);
  let currentVersion = Path.join([sfwRoot, "current"]);
};

let currentVersion = () =>
  switch (Fs.realpath(Directories.currentVersion)) {
  | x => Some(x)
  | exception (Unix.Unix_error(_, _, _)) => None
  };

let printableVersions = (~current, ~versions) => {
  open Pastel;

  let strings =
    versions
    |> List.map(version => {
         let fullPath = Path.join([Directories.nodeVersions, version]);
         let str = "- " ++ version;
         fullPath == current ? <Pastel color=Green> str </Pastel> : str;
       });

  <Pastel> ...strings </Pastel>;
};

let run = () => {
  open Result;

  let%bind current =
    currentVersion()
    |> Opt.toResult("No version selected")
    |> Result.fold(x => x, x => x)
    |> Result.return;
  let%bind x = Fs.readdir(Directories.nodeVersions);
  let%bind versions =
    Fs.readdir(Directories.nodeVersions) |> Result.map(Array.to_list);

  Console.log(
    <Pastel color=Pastel.Cyan> "## List of installed versions:" </Pastel>,
  );
  printableVersions(~current, ~versions) |> Console.log;
  Result.return();
};