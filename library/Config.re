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
  let getOpt = () => Sys.getenv_opt(name) |> Opt.map(parse);
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

let getDocs = () => [FNM_DIR.docInfo, FNM_NODE_DIST_MIRROR.docInfo];
