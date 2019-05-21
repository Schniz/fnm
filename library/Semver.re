[@deriving (eq, ord, make)]
type t = {
  major: int,
  minor: int,
  patch: int,
};

let fromString = str =>
  switch (
    String.split_on_char('.', str)
    |> List.map(int_of_string_opt)
    |> Base.Option.all
  ) {
  | Some([major, minor, patch]) => Some({major, minor, patch})
  | _ => None
  };

let toString = ({major, minor, patch}) =>
  Printf.sprintf("%d.%d.%d", major, minor, patch);

let major = ({major, _}) => major;
let minor = ({minor, _}) => minor;
let patch = ({patch, _}) => patch;