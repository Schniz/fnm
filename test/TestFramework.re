let projectDir = Sys.getcwd();
let tmpDir = Filename.concat(projectDir, ".fnmTmp");

include Rely.Make({
  let config =
    Rely.TestFrameworkConfig.initialize({
      snapshotDir:
        Filename.concat(
          projectDir,
          Filename.concat("test", "__snapshots__"),
        ),
      projectDir,
    });
});

let run = args => {
  let arguments =
    args
    |> Array.append([|"./_esy/default/build/default/executable/FnmApp.exe"|]);
  let env =
    Unix.environment()
    |> Array.append([|
         Printf.sprintf("%s=%s", Fnm.Config.FNM_DIR.name, tmpDir),
       |]);
  let result =
    Lwt_process.pread_chars(~env, ("", arguments)) |> Lwt_stream.to_string;
  Lwt_main.run(result);
};

let clearTmpDir = () => {
  let _ = Lwt_process.pread(("", [|"rm", "-rf", tmpDir|])) |> Lwt_main.run;
  ();
};

let redactSfwRoot =
  Str.global_replace(Str.regexp_string(tmpDir), "<sfwRoot>");
