open Fnm;

let colorizeVersions = (~current, ~versions) => {
  let strings =
    versions
    |> List.map(version => {
         open Versions.Local;
         let str = "- " ++ version.name;

         let color =
           current
           |> Opt.bind(current =>
                current.name == version.name ? Some(Pastel.Green) : None
              );

         <Pastel ?color> str </Pastel>;
       });

  <Pastel>
    <Pastel color=Pastel.Cyan> "## List of installed versions:\n" </Pastel>
    <Pastel> ...strings </Pastel>
  </Pastel>;
};

let getVersionsString = () =>
  Result.(
    {
      let%bind versions =
        Versions.getInstalledVersions() |> Result.map(Array.to_list);

      let current = Versions.getCurrentVersion();

      colorizeVersions(~current, ~versions) |> Result.return;
    }
  );

let run = () => getVersionsString() |> Result.map(Console.log) |> Lwt.return;
