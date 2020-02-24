open TestFramework;

let allVersions6 = [
  "v6.0.0",
  "v6.1.0",
  "v6.2.0",
  "v6.2.1",
  "v6.2.2",
  "v6.3.0",
  "v6.3.1",
  "v6.4.0",
  "v6.5.0",
  "v6.6.0",
  "v6.7.0",
  "v6.8.0",
  "v6.8.1",
  "v6.9.0",
  "v6.9.1",
  "v6.9.2",
  "v6.9.3",
  "v6.9.4",
  "v6.9.5",
  "v6.10.0",
  "v6.10.1",
  "v6.10.2",
  "v6.10.3",
  "v6.11.0",
  "v6.11.1",
  "v6.11.2",
  "v6.11.3",
  "v6.11.4",
  "v6.11.5",
  "v6.12.0",
  "v6.12.1",
  "v6.12.2",
  "v6.12.3",
  "v6.13.0",
  "v6.13.1",
  "v6.14.0",
  "v6.14.1",
  "v6.14.2",
  "v6.14.3",
  "v6.14.4",
  "v6.15.0",
  "v6.15.1",
  "v6.16.0",
  "v6.17.0",
  "v6.17.1",
];

let allVersions6_11 = [
  "v6.11.0",
  "v6.11.1",
  "v6.11.2",
  "v6.11.3",
  "v6.11.4",
  "v6.11.5",
];

describe("List Remote", ({test}) => {
  let versionRegExp = Str.regexp(".*[0-9]+\\.[0-9]+\\.[0-9]+\\|.*latest-*");

  let filterVersionNumbers = response =>
    response
    |> String.split_on_char('\n')
    |> List.filter(s => Str.string_match(versionRegExp, s, 0))
    |> List.map(s => Str.replace_first(Str.regexp("\\*"), "", s))
    |> List.map(String.trim);

  let runAndFilterVersionNumbers = args => run(args) |> filterVersionNumbers;

  test("Should display remote versions matching a major version", ({expect}) => {
    let versionNumbers = runAndFilterVersionNumbers([|"ls-remote", "6"|]);

    expect.lines(versionNumbers).toEqualLines(allVersions6);
  });
  test(
    "Should display remote versions matching a major and minor version",
    ({expect}) => {
    let versionNumbers = runAndFilterVersionNumbers([|"ls-remote", "6.11"|]);

    expect.lines(versionNumbers).toEqualLines(allVersions6_11);
  });
  test(
    "Should display remote version matching a specific version", ({expect}) => {
    let versionNumbers =
      runAndFilterVersionNumbers([|"ls-remote", "6.11.3"|]);

    expect.int(versionNumbers |> List.length).toBe(1);
    expect.lines(versionNumbers).toEqual("v6.11.3");
  });
  test("Should display latest remote version", ({expect}) => {
    let versionNumbers =
      runAndFilterVersionNumbers([|"ls-remote", "latest"|]);

    expect.int(versionNumbers |> List.length).toBe(1);
    expect.lines(versionNumbers).toEqual("latest");
  });

  test("Should display latest specific remote version", ({expect}) => {
    let versionNumbers =
      runAndFilterVersionNumbers([|"ls-remote", "latest-v6"|]);

    expect.int(versionNumbers |> List.length).toBe(1);
    expect.lines(versionNumbers).toEqual("latest-v6.x");
  });
  test(
    "Should display an error message if version does not exist", ({expect}) => {
    let result = run([|"ls-remote", "190385"|]);

    let versionNumbers = result |> filterVersionNumbers;

    expect.int(versionNumbers |> List.length).toBe(0);
    expect.string(result).toMatch(".*No versions found.*");
  });
});
