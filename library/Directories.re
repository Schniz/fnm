let sfwRoot =
  Opt.(
    Sys.getenv_opt("FNM_DIR")
    or {
      let home =
        Sys.getenv_opt("HOME")
        |> Opt.orThrow("There isn't $HOME environment variable set.");
      Filename.concat(home, ".fnm");
    }
  );
let nodeVersions = Filename.concat(sfwRoot, "node-versions");
let globalCurrentVersion = Filename.concat(sfwRoot, "current");
let currentVersion =
  Opt.(Sys.getenv_opt("FNM_MULTISHELL_PATH") or globalCurrentVersion);
let downloads = Filename.concat(sfwRoot, "downloads");
let aliases = Filename.concat(sfwRoot, "aliases");
