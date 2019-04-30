type t =
  | Quiet
  | Error
  | All;

let toString = logLevel =>
  switch (logLevel) {
  | Quiet => "quiet"
  | Error => "error"
  | All => "all"
  };

let fromString = logLevelString =>
  switch (logLevelString) {
  | "quiet" => Quiet
  | "error" => Error
  | "all" => All
  | _ => failwith("Unsupported level: " ++ logLevelString)
  };
