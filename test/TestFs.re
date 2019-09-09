open TestFramework;
open Fnm;

let get_tempdir = prefix => {
  Printf.sprintf(
    "fnm-test-%s-%s",
    prefix,
    Unix.time() |> int_of_float |> string_of_int,
  )
  |> Filename.concat(Filename.get_temp_dir_name());
};

describe("TestSemver", ({test, describe, _}) => {
  describe("listDirRecursively", ({test, _}) => {
    test("list files and directories", ({expect, _}) => {
      let xs = Fs.listDirRecursively("feature_tests") |> Lwt_main.run;
      expect.list(xs).toContainEqual(
        Fs.File("feature_tests/multishell/run.sh"),
      );
      expect.list(xs).toContainEqual(Fs.Dir("feature_tests/multishell"));
      expect.list(xs).toContainEqual(Fs.Dir("feature_tests"));
    });

    test("works with symlinks", ({expect, _}) => {
      let tmpdir = get_tempdir("symlinks");
      let fileDir = Filename.concat(tmpdir, "a/b/c/d/e/f/g");
      let filePath = Filename.concat(fileDir, "file");
      Sys.command("mkdir -p " ++ fileDir) |> ignore;
      Sys.command("touch " ++ filePath) |> ignore;
      Sys.command("ln -s " ++ filePath ++ " " ++ filePath ++ "_link")
      |> ignore;
      Sys.command(
        "ln -s " ++ filePath ++ "_not_found " ++ filePath ++ "_link_not_found",
      )
      |> ignore;

      let xs = Fs.listDirRecursively(tmpdir) |> Lwt_main.run;
      expect.list(xs).toContainEqual(Fs.File(filePath ++ "_link"));
      expect.list(xs).toContainEqual(
        Fs.File(filePath ++ "_link_not_found"),
      );
      expect.list(xs).toContainEqual(Fs.File(filePath));
      expect.list(xs).toContainEqual(Fs.Dir(fileDir));
    });
  });

  test("rmdir", ({expect, _}) => {
    let tmpdir = get_tempdir("rmdir");
    let fileDir = Filename.concat(tmpdir, "a/b/c/d/e/f/g");
    let filePath = Filename.concat(fileDir, "file");

    Sys.command("mkdir -p " ++ fileDir) |> ignore;
    Sys.command("touch " ++ filePath) |> ignore;
    Unix.symlink(filePath, filePath ++ "_link");
    Fs.rmdir(tmpdir) |> Lwt_main.run;

    let remainingFiles = Fs.listDirRecursively(tmpdir) |> Lwt_main.run;

    expect.list(remainingFiles).toBeEmpty();
  });
});
