type t =
  | Quiet
  | Error
  | Info
  | Debug;

let toString = logLevel =>
  switch (logLevel) {
  | Quiet => "quiet"
  | Error => "error"
  | Info => "info"
  | Debug => "debug"
  };

let fromString = logLevelString =>
  switch (logLevelString) {
  | "quiet" => Quiet
  | "error" => Error
  | "info" => Info
  | "debug"
  | "all" => Debug
  | _ => failwith("Unsupported level: " ++ logLevelString)
  };
