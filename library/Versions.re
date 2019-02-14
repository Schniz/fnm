module VersionSet = Set.Make(String);

let lwtIgnore = lwt => Lwt.catch(() => lwt, _ => Lwt.return());

module Local = {
  type t = {
    name: string,
    fullPath: string,
    aliases: list(string),
  };

  let toDirectory = name => Filename.concat(Directories.nodeVersions, name);
};

exception Version_not_found(string);
exception Already_installed(string);

module Aliases = {
  module VersionAliasMap = Map.Make(String);

  type t = {
    name: string,
    versionName: string,
    fullPath: string,
  };

  let toDirectory = name => Filename.concat(Directories.aliases, name);

  let getAll = () => {
    let%lwt aliases =
      try%lwt (Fs.readdir(Directories.aliases)) {
      | _ => Lwt.return([])
      };
    aliases
    |> List.map(alias => {
         let fullPath = Filename.concat(Directories.aliases, alias);
         {
           name: alias,
           fullPath,
           versionName:
             Filename.concat(Directories.aliases, alias)
             |> Fs.realpath
             |> Filename.basename,
         };
       })
    |> Lwt.return;
  };

  let byVersion = () => {
    let%lwt aliases = getAll();
    aliases
    |> List.fold_left(
         (map, curr) => {
           let value =
             switch (VersionAliasMap.find_opt(curr.versionName, map)) {
             | None => [curr.name]
             | Some(arr) => [curr.name, ...arr]
             };
           VersionAliasMap.add(curr.versionName, value, map);
         },
         VersionAliasMap.empty,
       )
    |> Lwt.return;
  };

  let set = (~alias, ~versionPath) => {
    let aliasPath = alias |> toDirectory;
    let%lwt _ = System.mkdirp(Directories.aliases);
    let%lwt _ = Lwt_unix.unlink(aliasPath) |> lwtIgnore;
    let%lwt _ = Lwt_unix.symlink(versionPath, aliasPath);
    Lwt.return();
  };
};

module Remote = {
  type t = {
    name: string,
    baseURL: string,
    installed: bool,
  };

  let skip = (~amount, str) =>
    Str.last_chars(str, String.length(str) - amount);

  let parseSemver = version => version |> skip(~amount=1) |> Semver.of_string;

  let compare = (v1, v2) =>
    switch (parseSemver(v1), parseSemver(v2)) {
    | (Some(v1), Some(v2)) => Semver.compare(v1, v2)
    | (None, _)
    | (_, None) => - Core.String.compare(v1, v2)
    };

  let getInstalledVersionSet = () =>
    Lwt.(
      catch(() => Fs.readdir(Directories.nodeVersions), _ => return([]))
      >|= List.fold_left(
            (acc, curr) => VersionSet.add(curr, acc),
            VersionSet.empty,
          )
    );

  let getRelativeLinksFromHTML = html =>
    Soup.parse(html)
    |> Soup.select("pre a")
    |> Soup.to_list
    |> List.map(Soup.attribute("href"))
    |> Core.List.filter_map(~f=x => x)
    |> List.map(x => {
         let parts = String.split_on_char('/', x) |> List.rev;
         switch (parts) {
         | ["", x, ...xs] => x ++ "/"
         | [x, ...xs] => x
         | [] => ""
         };
       });

  let downloadFileSuffix = ".tar.xz";

  let getVersionFromFilename = filename => {
    let strings = filename |> String.split_on_char('-');
    List.nth(strings, 1);
  };
};

let format = version => {
  let version =
    switch (Str.first_chars(version, 1) |> Int32.of_string) {
    | _ => "v" ++ version
    | exception _ => version
    };

  version;
};

let endsWith = (~suffix, str) => {
  let suffixLength = String.length(suffix);

  String.length(str) > suffixLength
  && Str.last_chars(str, suffixLength) == suffix;
};

exception No_Download_For_System(System.NodeOS.t, System.NodeArch.t);

let getFileToDownload = (~version as versionName, ~os, ~arch) => {
  let versionName =
    switch (Str.first_chars(versionName, 1) |> Int32.of_string) {
    | _ => "v" ++ versionName
    | exception _ => versionName
    };

  let url =
    Printf.sprintf("%s%s/", Config.FNM_NODE_DIST_MIRROR.get(), versionName);

  let%lwt html =
    try%lwt (Http.makeRequest(url) |> Lwt.map(Http.body)) {
    | Http.Not_found(_) => Lwt.fail(Version_not_found(versionName))
    };
  let filenames =
    html
    |> Remote.getRelativeLinksFromHTML
    |> List.filter(
         endsWith(
           ~suffix=
             System.NodeOS.toString(os)
             ++ "-"
             ++ System.NodeArch.toString(arch)
             ++ Remote.downloadFileSuffix,
         ),
       );

  switch (filenames |> List.hd) {
  | x => Lwt.return(url ++ x)
  | exception _ => Lwt.fail(No_Download_For_System(os, arch))
  };
};

let getCurrentVersion = () =>
  switch (Fs.realpath(Directories.currentVersion)) {
  | installationPath =>
    let fullPath = Filename.dirname(installationPath);
    Some(
      Local.{fullPath, name: Core.Filename.basename(fullPath), aliases: []},
    );
  | exception (Unix.Unix_error(_, _, _)) => None
  };

let getInstalledVersions = () => {
  let%lwt versions =
    Fs.readdir(Directories.nodeVersions)
    |> Lwt.map(List.sort(Remote.compare))
  and aliases = Aliases.byVersion();

  versions
  |> List.map(name =>
       Local.{
         name,
         fullPath: Filename.concat(Directories.nodeVersions, name),
         aliases: Opt.(Aliases.VersionAliasMap.find_opt(name, aliases) or []),
       }
     )
  |> Lwt.return;
};

let getRemoteVersions = () => {
  let%lwt bodyString =
    Config.FNM_NODE_DIST_MIRROR.get()
    |> Http.makeRequest
    |> Lwt.map(Http.body);

  let versions = bodyString |> Remote.getRelativeLinksFromHTML;
  let%lwt installedVersions = Remote.getInstalledVersionSet();

  versions
  |> Core.List.filter(~f=x =>
       Str.last_chars(x, 1) == "/" && Str.first_chars(x, 1) != "."
     )
  |> Core.List.map(~f=x => Str.first_chars(x, String.length(x) - 1))
  |> List.sort(Remote.compare)
  |> List.map(name =>
       Remote.{
         name,
         installed: VersionSet.find_opt(name, installedVersions) != None,
         baseURL:
           Printf.sprintf("%s%s/", Config.FNM_NODE_DIST_MIRROR.get(), name),
       }
     )
  |> Lwt.return;
};

type t =
  | Alias(string)
  | Local(string);

let parse = version => {
  let formattedVersion = format(version);
  let aliasPath = Aliases.toDirectory(version);
  let versionPath = Local.toDirectory(formattedVersion);

  let%lwt aliasExists = Lwt_unix.file_exists(aliasPath)
  and versionExists = Lwt_unix.file_exists(versionPath);

  switch (versionExists, aliasExists) {
  | (true, _) => Some(Local(formattedVersion)) |> Lwt.return
  | (_, true) => Some(Alias(version)) |> Lwt.return
  | (false, false) => Lwt.return_none
  };
};

let throwIfInstalled = versionName => {
  let%lwt installedVersions =
    try%lwt (getInstalledVersions()) {
    | _ => Lwt.return([])
    };
  let isAlreadyInstalled =
    installedVersions |> List.exists(x => Local.(x.name == versionName));
  if (isAlreadyInstalled) {
    Lwt.fail(Already_installed(versionName));
  } else {
    Lwt.return();
  };
};
