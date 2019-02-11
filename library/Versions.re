module VersionSet = Set.Make(String);

module Local = {
  type t = {
    name: string,
    fullPath: string,
  };
};

exception Version_not_found(string);
exception Already_installed(string);

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
    Fs.readdir(Directories.nodeVersions)
    |> Result.fold(_ => [||], x => x)
    |> Array.fold_left(
         (acc, curr) => VersionSet.add(curr, acc),
         VersionSet.empty,
       );

  let getRelativeLinksFromHTML = html =>
    Soup.parse(html)
    |> Soup.select("pre a")
    |> Soup.to_list
    |> List.map(Soup.attribute("href"))
    |> Core.List.filter_map(~f=x => x);

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

  let ansiCharRegex = Str.regexp({|\\027\[[0-9]+m|});
  Str.global_replace(ansiCharRegex, "", String.escaped(version));
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
  let url = "https://nodejs.org/dist/" ++ versionName ++ "/";
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
    Some(Local.{fullPath, name: Core.Filename.basename(fullPath)});
  | exception (Unix.Unix_error(_, _, _)) => None
  };

let getInstalledVersions = () =>
  Fs.readdir(Directories.nodeVersions)
  |> Result.map(x => {
       Array.sort(Remote.compare, x);
       x;
     })
  |> Result.map(
       Array.map(name =>
         Local.{
           name,
           fullPath: Filename.concat(Directories.nodeVersions, name),
         }
       ),
     );

let getRemoteVersions = () => {
  let%lwt bodyString =
    Http.makeRequest("https://nodejs.org/dist/") |> Lwt.map(Http.body);

  let versions = bodyString |> Remote.getRelativeLinksFromHTML;
  let installedVersions = Remote.getInstalledVersionSet();

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
         baseURL: "https://nodejs.org/dist/" ++ name ++ "/",
       }
     )
  |> Lwt.return;
};

let isAlreadyInstalled = versionName => {
  switch(getInstalledVersions()) {
  | Ok(x) => {
    Array.to_list(x)
    |> List.exists(x => {
      open Local;
      x.name == versionName;
    })
    |> x => switch x {
    | true => Lwt.fail(Already_installed(versionName));
    | false => Lwt.return();
    };
  };
  | Error(_) => Lwt.return();
  };
};
