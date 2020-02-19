open TestFramework;

describe("List Remote", ({test}) => {
  let allVersions6 = [
    "6.0.0",
    "6.1.0",
    "6.2.0",
    "6.2.1",
    "6.2.2",
    "6.3.0",
    "6.3.1",
    "6.4.0",
    "6.5.0",
    "6.6.0",
    "6.7.0",
    "6.8.0",
    "6.8.1",
    "6.9.0",
    "6.9.1",
    "6.9.2",
    "6.9.3",
    "6.9.4",
    "6.9.5",
    "6.10.0",
    "6.10.1",
    "6.10.2",
    "6.10.3",
    "6.11.0",
    "6.11.1",
    "6.11.2",
    "6.11.3",
    "6.11.4",
    "6.11.5",
    "6.12.0",
    "6.12.1",
    "6.12.2",
    "6.12.3",
    "6.13.0",
    "6.13.1",
    "6.14.0",
    "6.14.1",
    "6.14.2",
    "6.14.3",
    "6.14.4",
    "6.15.0",
    "6.15.1",
    "6.16.0",
    "6.17.0",
    "6.17.1",
  ];

  let allVersions6_11 = [
    "6.11.0",
    "6.11.1",
    "6.11.2",
    "6.11.3",
    "6.11.4",
    "6.11.5",
  ];

  let versionRegExp = Str.regexp(".*[0-9]+\.[0-9]+\.[0-9]+\|.*latest-*");

  let formatVersion = version => "* v" ++ version;

  test("Should display remote versions matching a major version", ({expect}) => {
    let result = run([|"ls-remote", "6"|]);

    let versions =
      result
      |> String.split_on_char('\n')
      |> List.filter(s => Str.string_match(versionRegExp, s, 0));

    expect.lines(versions).toEqualLines(
      allVersions6 |> List.map(formatVersion),
    );
  });
  test(
    "Should display remote versions matching a major and minor version",
    ({expect}) => {
    let result = run([|"ls-remote", "6.11"|]);

    let versions =
      result
      |> String.split_on_char('\n')
      |> List.filter(s => Str.string_match(versionRegExp, s, 0));

    expect.lines(versions).toEqualLines(
      allVersions6_11 |> List.map(formatVersion),
    );
  });
  test(
    "Should display remote version matching a specific version", ({expect}) => {
    let result = run([|"ls-remote", "6.11.3"|]);

    let versions =
      result
      |> String.split_on_char('\n')
      |> List.filter(s => Str.string_match(versionRegExp, s, 0));

    expect.int(versions |> List.length).toBe(1);
    expect.lines(versions).toEqual(formatVersion("6.11.3"));
  });
  test("Should display latest remote version", ({expect}) => {
    let result = run([|"ls-remote", "latest"|]);

    let versions =
      result
      |> String.split_on_char('\n')
      |> List.filter(s => Str.string_match(versionRegExp, s, 0));

    expect.int(versions |> List.length).toBe(1);
    expect.lines(versions).toEqual("* latest");
  });

  test("Should display latest specific remote version", ({expect}) => {
    let result = run([|"ls-remote", "latest-v6"|]);

    let versions =
      result
      |> String.split_on_char('\n')
      |> List.filter(s => Str.string_match(versionRegExp, s, 0));

    expect.int(versions |> List.length).toBe(1);
    expect.lines(versions).toEqual("* latest-v6.x");
  });
  test(
    "Should display an error message if version does not exist", ({expect}) => {
    let result = run([|"ls-remote", "190384"|]);

    let versions =
      result
      |> String.split_on_char('\n')
      |> List.filter(s => Str.string_match(versionRegExp, s, 0));

    expect.int(versions |> List.length).toBe(0);
    expect.string(result).toMatch(".*No versions found.*");
  });
});
