type item = {
  version: string,
  date: string,
  files: list(string),
  lts: option(string),
};

let item_of_yojson = (json: Yojson.Safe.t) => {
  open Yojson.Safe.Util;
  let version = json |> member("version") |> to_string;
  let files = json |> member("files") |> to_list |> List.map(to_string);
  let date = json |> member("date") |> to_string;
  let lts =
    Base.Option.try_with(() =>
      json |> member("lts") |> to_string |> String.lowercase_ascii
    );
  Ok({version, date, files, lts});
};

[@deriving of_yojson({strict: false})]
type t = list(item);

let parseVersionListing = body => {
  Base.Result.try_with(() => Yojson.Safe.from_string(body))
  |> Base.Result.map_error(~f=Printexc.to_string)
  |> Base.Result.bind(~f=x => of_yojson(x));
};

let fromHttp = () => {
  let url =
    Config.FNM_NODE_DIST_MIRROR.get() |> Printf.sprintf("%s/index.json");
  let%lwt {Http.body, _} = Http.makeRequest(url);
  parseVersionListing(body) |> Lwt.return;
};
