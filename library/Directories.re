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
let currentVersion = Filename.concat(sfwRoot, "current");
let downloads = Filename.concat(sfwRoot, "downloads");
