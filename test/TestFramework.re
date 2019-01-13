let projectDir = Sys.getcwd();
let tmpDir = Filename.concat(projectDir, ".nswTmp");

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
    args |> Array.append([|"./_build/default/executable/NswApp.exe"|]);
  let env = Unix.environment() |> Array.append([|"NSW_DIR=" ++ tmpDir|]);
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