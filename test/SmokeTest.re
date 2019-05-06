open TestFramework;

describe("Smoke test", ({test, _}) => {
  test("Tests run!", ({expect}) =>
    expect.int(1).toBe(1)
  );

  test("Get version", ({expect}) => {
    let version = run([|"--version"|]);
    expect.string(version).toMatch("^[0-9]+.[0-9]+.[0-9]+$");
  });

  test("env", ({expect}) => {
    let env = run([|"env"|]) |> redactSfwRoot;
    expect.string(env).toMatchSnapshot();
  });
});
