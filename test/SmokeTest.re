open TestFramework;

describe("Smoke test", ({test, _}) => {
  test("Tests run!", ({expect}) =>
    expect.int(1).toBe(1)
  );

  test("Get version", ({expect}) => {
    let version = run([|"--version"|]);
    expect.string(version |> String.trim).toMatch("^\\d+.\\d+.\\d+$");
  });

  test("env", ({expect}) => {
    let env = run([|"env", "--shell=bash"|]) |> redactSfwRoot;
    expect.string(env).toMatchSnapshot();
  });
});
