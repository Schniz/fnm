type lts_item = {
  version: string,
  lts: string,
};

type item = {
  generic_version: string,
  optional_lts: option(string),
};

let item_to_lts = item => {
  item.optional_lts
  |> Base.Option.map(~f=lts => {lts, version: item.generic_version});
};

let item_of_yojson = (json: Yojson.Safe.t) => {
  open Yojson.Safe.Util;
  let generic_version = json |> member("version") |> to_string;
  let optional_lts =
    Base.Option.try_with(() => {
      json |> member("lts") |> to_string |> String.lowercase_ascii
    });
  Ok({generic_version, optional_lts});
};

[@deriving of_yojson({strict: false})]
type t = list(item);

type get_latest_lts_errors =
  | Cant_parse_remote_version_listing(string)
  | Cant_find_latest_lts;

exception Problem_with_finding_latest_lts(get_latest_lts_errors);

let latestLtsOfJsonString = body => {
  Base.Result.try_with(() => Yojson.Safe.from_string(body))
  |> Base.Result.map_error(~f=Printexc.to_string)
  |> Base.Result.bind(~f=x => of_yojson(x))
  |> Base.Result.map_error(~f=err => Cant_parse_remote_version_listing(err))
  |> Base.Result.bind(~f=parsed => {
       parsed
       |> Base.List.filter_map(~f=item_to_lts)
       |> Base.List.max_elt(~compare=(a, b) =>
            Versions.compare(a.version, b.version)
          )
       |> Base.Result.of_option(~error=Cant_find_latest_lts)
     });
};

let getLatestLts = () => {
  let url =
    Config.FNM_NODE_DIST_MIRROR.get() |> Printf.sprintf("%s/index.json");
  let%lwt {Http.body, _} = Http.makeRequest(url);
  latestLtsOfJsonString(body) |> Lwt.return;
};
