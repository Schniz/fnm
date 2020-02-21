open TestFramework;

let installVersion = version => run([|"install", version|]);
let uninstallVersion = version => run([|"uninstall", version|]);

let isVersionInstalled = version =>
  run([|"ls"|])
  |> String.split_on_char('\n')
  |> List.exists(v => v == "* v" ++ version);

describe("Uninstall", ({test}) => {
  test("Should be possible to uninstall a specific version", ({expect, _}) => {
    let version = "6.0.0";
    let _ = installVersion(version);
    let response = uninstallVersion(version);
    expect.string(response).toMatch(
      ".*v" ++ version ++ ".* has correctly been removed.*",
    );
    expect.bool(isVersionInstalled(version)).toBeFalse();
  });
  test(
    "Should print out a message if multiple versions match the prefix",
    ({expect, _}) => {
    let v1 = "6.10.0";
    let v2 = "6.11.0";
    let _ = installVersion(v1);
    let _ = installVersion(v2);
    let response =
      uninstallVersion("6")
      |> String.split_on_char('\n')
      |> String.concat(" ");
    expect.string(response).toMatch(
      ".*multiple versions.*" ++ v1 ++ ".*" ++ v2 ++ ".*",
    );
    expect.bool(isVersionInstalled(v1)).toBeTrue();
    expect.bool(isVersionInstalled(v2)).toBeTrue();
    clearTmpDir();
  });
  test(
    "Should be able to uninstall with a prefix if only one version match",
    ({expect, _}) => {
    let v1 = "6.10.0";
    let v2 = "6.11.0";
    let _ = installVersion(v1);
    let _ = installVersion(v2);
    let response = uninstallVersion("6.10");
    expect.string(response).toMatch(
      ".*v" ++ v1 ++ ".* has correctly been removed.*",
    );
    expect.bool(isVersionInstalled(v1)).toBeFalse();
    expect.bool(isVersionInstalled(v2)).toBeTrue();
    clearTmpDir();
  });
});
