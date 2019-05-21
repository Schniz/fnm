open TestFramework;

describe("Semver", ({test, _}) => {
  open Fnm;

  test("parses a string", ({expect, _}) => {
    let x = Semver.fromString("1.2.3");
    expect.equal(x, Some(Semver.make(~major=1, ~minor=2, ~patch=3)));
  });

  test("returns none on invalid semver", ({expect, _}) => {
    let x = Semver.fromString("x1.2.3");
    expect.equal(x, None);
  });

  test("compare versions", ({expect}) => {
    let semver1 = Semver.make(~major=1, ~minor=2, ~patch=0);
    let semver2 = Semver.make(~major=1, ~minor=2, ~patch=3);
    let semver3 = Semver.make(~major=1, ~minor=3, ~patch=3);
    let semver4 = Semver.make(~major=4, ~minor=3, ~patch=3);

    let sorted =
      List.sort(Semver.compare, [semver1, semver2, semver4, semver3]);
    expect.list(sorted).toEqual([semver1, semver2, semver3, semver4]);
  });
});
