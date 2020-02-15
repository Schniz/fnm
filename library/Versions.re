module VersionSet = Set.Make(String);

let lwtIgnore = lwt => Lwt.catch(() => lwt, _ => Lwt.return());

let flip = (fn, a, b) => fn(b, a);

let skip = (~amount, str) =>
  Str.last_chars(str, String.length(str) - amount);

let parseSemver = version => version |> skip(~amount=1) |> Semver.fromString;

let compare = (v1, v2) =>
  switch (parseSemver(v1), parseSemver(v2)) {
  | (Some(v1), Some(v2)) => Semver.compare(v1, v2)
  | (None, _)
  | (_, None) => - Base.String.compare(v1, v2)
  };

let isVersionFitsPrefix = (prefix, version) => {
  let length = String.length(prefix);
  String.length(version) >= length
  + 1
  && Str.first_chars(version, length + 1) == prefix
  ++ ".";
};

module Local = {
  type t = {
    name: string,
    fullPath: string,
    aliases: list(string),
  };

  let systemVersion: t = {
    name: "system",
    fullPath: "/dev/null/installation",
    aliases: [],
  };

  let toDirectory = name =>
    Filename.concat(
      Filename.concat(Directories.nodeVersions, name),
      "installation",
    );

  let remove = version => Fs.rmdir(version.fullPath);

  let getLatestInstalledNameByPrefix = prefix =>
    Lwt.Infix.(
      {
        let%lwt versions =
          Lwt.catch(
            () =>
              Fs.readdir(Directories.nodeVersions)
              >|= List.filter(isVersionFitsPrefix(prefix))
              >|= List.sort(flip(compare)),
            _ => Lwt.return_nil,
          );
        switch (versions) {
        | [version, ..._xs] => Lwt.return_some(version)
        | [] => Lwt.return_none
        };
      }
    );
};

exception Version_not_found(string);

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
      try%lwt(Fs.readdir(Directories.aliases)) {
      | _ => Lwt.return([])
      };
    aliases
    |> Lwt_list.map_p(alias => {
         let fullPath = Filename.concat(Directories.aliases, alias);
         let%lwt realpath =
           Filename.concat(Directories.aliases, alias)
           |> Fs.realpath
           |> Lwt.map(
                fun
                | Fs.Exists(x) => x
                | Fs.Missing(x) => x,
              );

         Lwt.return({
           name: alias,
           fullPath,
           versionName: realpath |> Filename.dirname |> Filename.basename,
         });
       });
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

  let getInstalledVersionSet = () =>
    Lwt.(
      catch(() => Fs.readdir(Directories.nodeVersions), _ => return([]))
      >|= List.fold_left(
            (acc, curr) => VersionSet.add(curr, acc),
            VersionSet.empty,
          )
    );

  let isMajor = (majorVersion, version) => {
    parseSemver(version.name)
    |> Base.Option.value_map(~f=Semver.isMajor(majorVersion), ~default=false);
  };

  let getRelativeLinksFromHTML = html =>
    Soup.parse(html)
    |> Soup.select("pre a")
    |> Soup.to_list
    |> List.map(Soup.attribute("href"))
    |> Base.List.filter_map(~f=x => x)
    |> List.map(x => {
         let parts = String.split_on_char('/', x) |> List.rev;
         switch (parts) {
         | ["", x, ..._xs] => x ++ "/"
         | [x, ..._xs] => x
         | [] => ""
         };
       });

  let downloadFileSuffix = ".tar.xz";

  let getVersionFromFilename = filename => {
    let strings = filename |> String.split_on_char('-');
    List.nth(strings, 1);
  };
};

let ltsVersion = version =>
  if (Base.String.is_prefix(version, ~prefix="lts/")) {
    switch (Str.last_chars(version, String.length(version) - 4)) {
    | exception _ => None
    | x => Some(x)
    };
  } else {
    None;
  };

let format = version => {
  let version =
    switch (Str.first_chars(version, 1) |> Int32.of_string) {
    | _ => "v" ++ version
    | exception _ => version
    };

  let version =
    switch (ltsVersion(version)) {
    | Some(lts) => "latest-" ++ lts
    | None => version
    };

  version;
};

let endsWith = (~suffix, str) => {
  let suffixLength = String.length(suffix);

  String.length(str) > suffixLength
  && Str.last_chars(str, suffixLength) == suffix;
};

exception No_Download_For_System(System.NodeOS.t, System.NodeArch.t);

let getCurrentVersion = () =>
  switch%lwt (Fs.realpath(Directories.currentVersion)) {
  | Missing(x) when x == Directories.currentVersion => Lwt.return_none
  | Missing(_) => Lwt.return_some(Local.systemVersion)
  | Exists(installationPath) =>
    let fullPath = Filename.dirname(installationPath);
    Lwt.return_some(
      Local.{
        fullPath,
        name:
          fullPath
          |> Path.absolute
          |> Base.Option.bind(~f=Path.baseName)
          |> Base.Option.value(~default=""),
        aliases: [],
      },
    );
  };

let getInstalledVersions = () => {
  let%lwt versions =
    Fs.readdir(Directories.nodeVersions) |> Lwt.map(List.sort(compare))
  and aliases = Aliases.byVersion();

  versions
  |> List.map(name =>
       Local.{
         name,
         fullPath: Filename.concat(Directories.nodeVersions, name),
         aliases: Opt.(Aliases.VersionAliasMap.find_opt(name, aliases) or []),
       }
     )
  |> List.append([Local.systemVersion])
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
  |> List.filter(x =>
       Str.last_chars(x, 1) == "/" && Str.first_chars(x, 1) != "."
     )
  |> List.map(x => Str.first_chars(x, String.length(x) - 1))
  |> List.sort(compare)
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

let getRemoteLatestVersionByPrefix = prefix =>
  Remote.(
    {
      let%lwt remoteVersions = getRemoteVersions();

      let compatibleVersions =
        remoteVersions
        |> List.map(x => x.name)
        |> List.filter(isVersionFitsPrefix(prefix))
        |> List.sort(flip(compare));

      switch (compatibleVersions) {
      | [version, ..._vs] => Lwt.return_some(version)
      | [] => Lwt.return_none
      };
    }
  );

let getExactFileToDownload = (~version as versionName, ~os, ~arch) => {
  let versionName =
    switch (Str.first_chars(versionName, 1) |> Int32.of_string) {
    | _ => "v" ++ versionName
    | exception _ => versionName
    };

  let url =
    Printf.sprintf("%s%s/", Config.FNM_NODE_DIST_MIRROR.get(), versionName);

  let%lwt html =
    try%lwt(Http.makeRequest(url) |> Lwt.map(Http.body)) {
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
  | filename =>
    let nodeVersion = List.nth(String.split_on_char('-', filename), 1);
    Lwt.return((nodeVersion, url ++ filename));
  | exception _ => Lwt.fail(No_Download_For_System(os, arch))
  };
};

let getFileToDownload = (~version, ~os, ~arch) =>
  try%lwt(getExactFileToDownload(~version, ~os, ~arch)) {
  | Version_not_found(_) as e =>
    switch%lwt (getRemoteLatestVersionByPrefix(version)) {
    | None => Lwt.fail(e)
    | Some(exactVersion) =>
      getExactFileToDownload(~version=exactVersion, ~os, ~arch)
    }
  };

type t =
  | System
  | Alias(string)
  | Local(string);

let parse = version => {
  let formattedVersion = format(version);

  if (formattedVersion == Local.systemVersion.name) {
    Lwt.return_ok(System);
  } else {
    let%lwt aliasExists = Aliases.toDirectory(version) |> Fs.exists
    and aliasExistsOnFormatted =
      Aliases.toDirectory(formattedVersion) |> Fs.exists
    and versionExists = Local.toDirectory(formattedVersion) |> Fs.exists
    and versionByPrefixPath =
      Local.getLatestInstalledNameByPrefix(formattedVersion);

    switch (
      versionExists,
      aliasExists,
      aliasExistsOnFormatted,
      versionByPrefixPath,
    ) {
    | (true, _, _, _) => Local(formattedVersion) |> Lwt.return_ok
    | (_, true, _, _) => Alias(version) |> Lwt.return_ok
    | (_, _, true, _) => Alias(formattedVersion) |> Lwt.return_ok
    | (_, false, false, Some(version)) => Local(version) |> Lwt.return_ok
    | (false, false, false, None) => Lwt.return_error(formattedVersion)
    };
  };
};

let isInstalled = versionName => {
  let%lwt installedVersions =
    try%lwt(getInstalledVersions()) {
    | _ => Lwt.return([])
    };
  installedVersions
  |> List.exists(x => Local.(x.name == versionName))
  |> Lwt.return;
};
