open TestFramework;

describe("List Remote", ({test}) => {
  let versionRegExp = Str.regexp(".*[0-9]+\.[0-9]+\.[0-9]+\|.*latest-*");

  let matchMajor = (majorVersion, s) =>
    Str.string_match(
      Str.regexp(".*" ++ string_of_int(majorVersion) ++ "\.[0-9]+\.[0-9]+"),
      s,
      0,
    );
  let matchMajorAndMinor = (majorVersion, minorVersion, s) =>
    Str.string_match(
      Str.regexp(
        ".*"
        ++ string_of_int(majorVersion)
        ++ "\."
        ++ string_of_int(minorVersion)
        ++ "\.[0-9]+",
      ),
      s,
      0,
    );

  test("Should display remote versions matching a major version", ({expect}) => {
    let result = run([|"ls-remote", "6"|]);

    let versions =
      result
      |> String.split_on_char('\n')
      |> List.filter(s => Str.string_match(versionRegExp, s, 0));

    let allMatch = versions |> List.for_all(matchMajor(6));

    expect.bool(allMatch).toBeTrue();
  });
  test(
    "Should display remote versions matching a major and minor version",
    ({expect}) => {
    let result = run([|"ls-remote", "6.11"|]);

    let versions =
      result
      |> String.split_on_char('\n')
      |> List.filter(s => Str.string_match(versionRegExp, s, 0));

    let allMatch = versions |> List.for_all(matchMajorAndMinor(6, 11));

    expect.bool(allMatch).toBeTrue();
  });
  test("Should display a specific remote version", ({expect}) => {
    let result = run([|"ls-remote", "6.11.3"|]);

    let versions =
      result
      |> String.split_on_char('\n')
      |> List.filter(s => Str.string_match(versionRegExp, s, 0));

    expect.int(versions |> List.length).toBe(1);
    expect.lines(versions).toEqual("* v6.11.3");
  });
});
