let sfwRoot = Config.FNM_DIR.get();
let nodeVersions = Filename.concat(sfwRoot, "node-versions");
let globalCurrentVersion = Filename.concat(sfwRoot, "current");
let currentVersion =
  Opt.(Sys.getenv_opt("FNM_MULTISHELL_PATH") or globalCurrentVersion);
let downloads = Filename.concat(sfwRoot, "downloads");
let aliases = Filename.concat(sfwRoot, "aliases");
let defaultVersion = Filename.concat(aliases, "default");
