type variable_doc('t) = {
  name: string,
  doc: string,
  default: string,
};

module EnvVar =
       (
         M: {
           type t;
           let name: string;
           let doc: string;
           let default: t;
           let parse: string => t;
           let unparse: t => string;
         },
       ) => {
  include M;
  let optValue = Sys.getenv_opt(name) |> Opt.map(parse);
  let getOpt = () => optValue;
  let get = () => Opt.(getOpt() or default);
  let docInfo = {name, doc, default: unparse(default)};
};

let ensureTrailingBackslash = str =>
  switch (str.[String.length(str) - 1]) {
  | '/' => str
  | _ => str ++ "/"
  };

module FNM_NODE_DIST_MIRROR =
  EnvVar({
    type t = string;
    let name = "FNM_NODE_DIST_MIRROR";
    let doc = "https://nodejs.org/dist/ mirror";
    let default = "https://nodejs.org/dist/";
    let parse = ensureTrailingBackslash;
    let unparse = ensureTrailingBackslash;
  });

module FNM_DIR =
  EnvVar({
    type t = string;
    let parse = ensureTrailingBackslash;
    let unparse = ensureTrailingBackslash;
    let name = "FNM_DIR";
    let doc = "The root directory of fnm installations.";
    let default = {
      let home =
        Sys.getenv_opt("HOME")
        |> Opt.orThrow("There isn't $HOME environment variable set.");
      Filename.concat(home, ".fnm");
    };
  });

module FNM_MULTISHELL_PATH =
  EnvVar({
    type t = string;
    let parse = x => x;
    let unparse = x => x;
    let name = "FNM_MULTISHELL_PATH";
    let doc = "Where the current node version link is stored";
    let default = "";
  });

module FNM_LOGLEVEL =
  EnvVar({
    type t = LogLevel.t;
    let parse = LogLevel.fromString;
    let unparse = LogLevel.toString;
    let name = "FNM_LOGLEVEL";
    let doc = "The log level of fnm commands";
    let default = LogLevel.All;
  });

let parseBooleanOrDie = (~name, str) =>
  switch (bool_of_string_opt(str)) {
  | Some(boolean) => boolean
  | None =>
    let errorString =
      <Pastel color=Pastel.Red>
        <Pastel color=Pastel.Cyan> str </Pastel>
        " isn't a valid option for "
        <Pastel color=Pastel.Magenta> name </Pastel>
        " which assumes a boolean value. Consider passing "
        <Pastel color=Pastel.Cyan> "true" </Pastel>
        " or "
        <Pastel color=Pastel.Cyan> "false" </Pastel>
        "."
      </Pastel>;
    Printf.eprintf("%s\n", errorString);
    exit(1);
  };

module FNM_INTERACTIVE_CLI =
  EnvVar({
    type t = bool;
    let default = Unix.(isatty(stdin) && isatty(stdout) && isatty(stderr));
    let unparse = string_of_bool;
    let name = "FNM_INTERACTIVE_CLI";
    let doc = "Should the CLI be interactive? true/false";
    let parse = parseBooleanOrDie(~name);
  });

let getDocs = () => [
  FNM_DIR.docInfo,
  FNM_NODE_DIST_MIRROR.docInfo,
  FNM_MULTISHELL_PATH.docInfo,
  FNM_INTERACTIVE_CLI.docInfo,
  FNM_LOGLEVEL.docInfo,
];
