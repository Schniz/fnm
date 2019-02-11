let run = (~name, ~version) => {
  Printf.sprintf("Aliasing %s to %s", name, version) |> Console.log;

  Lwt.return();
};