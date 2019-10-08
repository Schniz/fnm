type item = {
  version: string,
  lts: string,
};

let item_to_lts = (item: VersionListing.item) => {
  item.lts |> Base.Option.map(~f=lts => {lts, version: item.version});
};

type get_latest_lts_errors =
  | Cant_parse_remote_version_listing(string)
  | Cant_find_latest_lts;

exception Problem_with_finding_latest_lts(get_latest_lts_errors);

let getLatest = () => {
  let%lwt versions = VersionListing.fromHttp();
  versions
  |> Base.Result.map_error(~f=err => Cant_parse_remote_version_listing(err))
  |> Base.Result.bind(~f=parsed => {
       parsed
       |> Base.List.filter_map(~f=item_to_lts)
       |> Base.List.max_elt(~compare=(a, b) =>
            Versions.compare(a.version, b.version)
          )
       |> Base.Result.of_option(~error=Cant_find_latest_lts)
     })
  |> Lwt.return;
};
