/**
 * Copyright 2004-present Facebook. All Rights Reserved.
 *
 * @emails oncall+ads_front_end_infra
 */;

[@warning "-27-39-32-34"];

let sep = "/";
let homeChar = "~";

type absolute;
type relative;
type upDirs = int; /* int 0 implies ./ and 1 implies ../ etc */

/**
 * We might eventually want to allow extending this with many
 * reference points.
 */
type relFrom =
  | Home
  | Any;
type base('kind) =
  /* Optional drive name */
  | Abs(option(string)): base(absolute)
  | Rel(relFrom, upDirs): base(relative);
/**
 * Internal representation of paths. The list of strings represents all
 * subdirectories after the base (in reverse order - head of the list is the
 * rightmost segment of the path).
 */
type t('kind) = (base('kind), list(string));
type firstClass =
  | Absolute(t(absolute))
  | Relative(t(relative));
type opaqueBase =
  | Base(base('exists)): opaqueBase;

let drive = name => (Abs(Some(name)), []);
let root = (Abs(None), []);
let home = (Rel(Home, 0), []);
let dot = (Rel(Any, 0), []);

let hasParentDir = ((Abs(_), lst): t(absolute)) => lst !== [];

let rec revSegmentsAreInside = (~ofSegments, l) =>
  switch (ofSegments, l) {
  | ([], [_, ..._]) => true
  | ([], []) => true
  | ([_, ..._], []) => false
  | ([hd, ...tl], [hd2, ...tl2]) =>
    String.equal(hd, hd2) && revSegmentsAreInside(~ofSegments=tl, tl2)
  };

let segmentsAreInside = (~ofSegments, l) =>
  revSegmentsAreInside(~ofSegments=List.rev(ofSegments), List.rev(l));

let isDescendent: type kind. (~ofPath: t(kind), t(kind)) => bool =
  (~ofPath, p) =>
    switch (ofPath, p) {
    | ((Abs(dr1), l1), (Abs(dr2), l2)) =>
      switch (dr1, dr2) {
      | (None, None) => segmentsAreInside(~ofSegments=l1, l2)
      | (Some(d1), Some(d2)) =>
        String.equal(d1, d2) && segmentsAreInside(~ofSegments=l1, l2)
      | (Some(_), None)
      | (None, Some(_)) => false
      }
    | ((Rel(Any, d1), l1), (Rel(Any, d2), l2)) =>
      d1 === d2 && segmentsAreInside(~ofSegments=l1, l2)
    | ((Rel(Home, d1), l1), (Rel(Home, d2), l2)) =>
      d1 === d2 && segmentsAreInside(~ofSegments=l1, l2)
    | ((Rel(Any, _), _), (Rel(Home, _), _)) => false
    | ((Rel(Home, _), _), (Rel(Any, _), _)) => false
    };

let toString: type kind. t(kind) => string =
  path =>
    switch (path) {
    | (Abs(l), lst) =>
      let lbl =
        switch (l) {
        | None => ""
        | Some(txt) => txt
        };
      lbl ++ "/" ++ (lst |> List.rev |> String.concat(sep));
    | (Rel(w, i), lst) =>
      let init =
        switch (w) {
        | Any => "." ++ sep
        | Home => "~" ++ sep
        };
      let rest =
        lst
        |> List.rev
        |> List.append(Array.to_list(Array.init(i, _ => "..")))
        |> String.concat(sep);
      init ++ rest;
    };

/**
 * Expose this under the name `toDebugString` and accept any kind of path.
 * The name is to warn people about using this for relative paths. This may
 * print paths like `"."` and `"~"`, which is not very meaningful.
 */
let toDebugString = toString;

type token =
  | SLASH
  | DOT
  | TILDE
  | DOTDOT
  | DRIVE(string)
  | TXT(string);

let makeToken = s =>
  switch (s) {
  | "~" => TILDE
  | "." => DOT
  | ".." => DOTDOT
  | s when String.length(s) >= 2 && s.[String.length(s) - 1] === ':' =>
    DRIVE(s)
  | s => TXT(s)
  };
/*
 * Splits on slashes, but being intelligent about escaped slashes.
 */
let lex = s => {
  let s = String.trim(s);
  let len = String.length(s);
  let revTokens = {contents: []};
  /* j is what you are all caught up to */
  let j = {contents: (-1)};
  let prevEsc = {contents: false};
  for (i in 0 to len - 1) {
    let ch = String.unsafe_get(s, i);
    if (ch === '/' && !prevEsc.contents) {
      if (j.contents !== i - 1) {
        let tok =
          makeToken(String.sub(s, j.contents + 1, i - j.contents - 1));
        revTokens.contents = [tok, ...revTokens.contents];
      };
      revTokens.contents = [SLASH, ...revTokens.contents];
      j.contents = i;
    };
    prevEsc.contents = ch === '\\' && !prevEsc.contents;
  };
  let rev =
    j.contents === len - 1 ?
      revTokens.contents :
      [
        makeToken(String.sub(s, j.contents + 1, len - 1 - j.contents)),
        ...revTokens.contents,
      ];
  List.rev(rev);
};

let _parseFirstToken = token =>
  switch (token) {
  | SLASH => (Base(Abs(None)), [])
  | DOT => (Base(Rel(Any, 0)), [])
  | TILDE => (Base(Rel(Home, 0)), [])
  | DOTDOT => (Base(Rel(Any, 1)), [])
  | DRIVE(l) => (Base(Abs(Some(l))), [])
  | TXT(s) => (Base(Rel(Any, 0)), [s])
  };

let parseNextToken: type kind. (t(kind), token) => t(kind) =
  (path, nextToken) =>
    switch (path, nextToken) {
    | (path, SLASH) => path
    | (path, DOT) => path
    | ((base, subs), TILDE) => (base, [homeChar, ...subs])
    | ((base, subs), DRIVE(l)) => (base, [l, ...subs])
    | ((base, subs), TXT(s)) => (base, [s, ...subs])
    | ((base, [hd, ...tl]), DOTDOT) => (base, tl)
    | ((Rel(Any, r), []), DOTDOT) => (Rel(Any, r + 1), [])
    | ((Rel(Home, r), []), DOTDOT) => (Rel(Home, r + 1), [])
    | ((Abs(_), []), DOTDOT) => path
    };

let parseFirstTokenAbsolute = token =>
  switch (token) {
  | SLASH => Some((Abs(None), []))
  | DRIVE(l) => Some((Abs(Some(l)), []))
  | TXT(_)
  | DOT
  | TILDE
  | DOTDOT => None
  };

let parseFirstTokenRelative = token =>
  switch (token) {
  | DOT => Some((Rel(Any, 0), []))
  | TILDE => Some((Rel(Home, 0), []))
  | DOTDOT => Some((Rel(Any, 1), []))
  | TXT(s) => Some((Rel(Any, 0), [s]))
  | SLASH => None
  | DRIVE(l) => None
  };

let absolute = s =>
  switch (lex(s)) {
  /* Cannot pass empty string for absolute path */
  | [] => None
  | [hd, ...tl] =>
    switch (parseFirstTokenAbsolute(hd)) {
    | None => None
    | Some(initAbsPath) =>
      Some(List.fold_left(parseNextToken, initAbsPath, tl))
    }
  };

let absoluteExn = s =>
  switch (lex(s)) {
  /* Cannot pass empty string for absolute path */
  | [] => raise(Invalid_argument("Empty path is not a valid absolute path."))
  | [hd, ...tl] =>
    switch (parseFirstTokenAbsolute(hd)) {
    | None =>
      raise(
        Invalid_argument("First token in path " ++ s ++ " is not absolute."),
      )
    | Some(initAbsPath) => List.fold_left(parseNextToken, initAbsPath, tl)
    }
  };

let relative = s => {
  let (tok, tl) =
    switch (lex(s)) {
    | [] => (DOT, [])
    | [hd, ...tl] => (hd, tl)
    };
  switch (parseFirstTokenRelative(tok)) {
  | None => None
  | Some(initRelPath) =>
    Some(List.fold_left(parseNextToken, initRelPath, tl))
  };
};

let relativeExn = s =>
  switch (lex(s)) {
  /* Cannot pass empty string for absolute path */
  | [] => dot
  | [hd, ...tl] =>
    switch (parseFirstTokenRelative(hd)) {
    | None =>
      raise(
        Invalid_argument("First token in path " ++ s ++ " not relative."),
      )
    | Some(initRelPath) => List.fold_left(parseNextToken, initRelPath, tl)
    }
  };

/**
 * Relates two positive integers to zero and eachother.
 */
type ord =
  | /** 0 === i === j */ Zeros
  | /** 0 === i < j */ ZeroPositive
  | /** i > 0 === j */ PositiveZero
  | /** 0 < i && 0 < j */ Positives;

/**
 * Using `ord` allows us to retain exhaustiveness pattern matching checks that
 * would normally be lost when adding `when i < j` guards to matches. It's
 * very likely inlined so there's no performance hit. Annotate as int so that
 * it isn't inferred to be polymorphic.
 */
let ord = (i: int, j: int) =>
  i === 0 && j === 0 ?
    Zeros : i === 0 ? ZeroPositive : j === 0 ? PositiveZero : Positives;

let rec repeat = (soFar, i, s) =>
  i === 0 ? soFar : repeat(soFar ++ s, i - 1, s);

/*
 *  relativize(a/rest1..., a/rest2...) == relativize(rest1..., rest2...)
 *  relativize(../rest1..., ../rest2...) == relativize(rest1..., res2...)
 *  relativize(a/rest1..., b/rest2...) == [...len(1)]/b/rest2
 *  relativize(../a/rest1..., b/rest2...) == raise
 *  relativize(a/rest1..., ../b/rest2...) == [...len(1)]../b/rest2
 *
 *  "upDirs" is the number of ../ the path is assumed to have. The segments
 *  `s1`/`s2`, are in the path order from left to right, unlike `Path.t` which
 *  usually stores them in reverse order. Relativizing paths is one place where
 *  it's more convenient to have them in the left to right segment order.
 */
let rec relativizeDepth = ((upDirs1, s1), (upDirs2, s2)) =>
  switch (ord(upDirs1, upDirs2), s1, s2) {
  | (Zeros, [hd1, ...tl1], [hd2, ...tl2]) =>
    if (String.compare(hd1, hd2) === 0) {
      relativizeDepth((0, tl1), (0, tl2));
    } else {
      (List.length(s1), s2);
    }
  | (Zeros, [], []) => (0, [])
  | (Zeros, [], [hd2, ...tl2] as s2) => (upDirs2, s2)
  | (Zeros, [hd1, ...tl1] as s1, []) => (List.length(s1), [])
  | (Positives, _, _) =>
    relativizeDepth((upDirs1 - 1, s1), (upDirs2 - 1, s2))
  | (ZeroPositive, _, _) => (List.length(s1) + upDirs2, s2)
  | (PositiveZero, _, _) =>
    raise(
      Invalid_argument(
        "Cannot relativize paths source='"
        ++ repeat("", upDirs1, "../")
        ++ String.concat(sep, s1)
        ++ "' dest='"
        ++ repeat("", upDirs2, "../")
        ++ String.concat(sep, s2),
      ),
    )
  };

let raiseDriveMismatch = (p1, p2) =>
  raise(
    Invalid_argument(
      "Cannot relativize paths with different drives or relative roots "
      ++ toString(p1)
      ++ " and "
      ++ toString(p2),
    ),
  );

let relativizeExn: type k. (~source: t(k), ~dest: t(k)) => t(relative) =
  (~source, ~dest) => {
    let (depth, segs) =
      switch (source, dest) {
      | ((Abs(d1), s1), (Abs(d2), s2)) =>
        switch (d1, d2) {
        | (None, None) =>
          relativizeDepth((0, List.rev(s1)), (0, List.rev(s2)))
        | (Some(_), None) => raiseDriveMismatch(source, dest)
        | (None, Some(_)) => raiseDriveMismatch(source, dest)
        | (Some(d1), Some(d2)) =>
          String.compare(d1, d2) !== 0 ?
            raiseDriveMismatch(source, dest) :
            relativizeDepth((0, List.rev(s1)), (0, List.rev(s2)))
        }
      | ((Rel(w1, r1), s1), (Rel(w2, r2), s2)) =>
        w1 === w2 ?
          relativizeDepth((r1, List.rev(s1)), (r2, List.rev(s2))) :
          raiseDriveMismatch(source, dest)
      };
    (Rel(Any, depth), List.rev(segs));
  };

let relativize:
  type k. (~source: t(k), ~dest: t(k)) => result(t(relative), exn) =
  (~source, ~dest) =>
    try (Ok(relativizeExn(~source, ~dest))) {
    | Invalid_argument(_) as e => Error(e)
    };

let rec segEq = (l1, l2) =>
  switch (l1, l2) {
  | ([], []) => true
  | ([], [_, ..._]) => false
  | ([_, ..._], []) => false
  | ([hd1, ...tl1], [hd2, ...tl2]) =>
    String.compare(hd1, hd2) === 0 && segEq(tl1, tl2)
  };

let eq: type k1 k2. (t(k1), t(k2)) => bool =
  (p1, p2) =>
    switch (p1, p2) {
    | ((Abs(_), s1), (Rel(_), s2)) => false
    | ((Rel(_), s1), (Abs(_), s2)) => false
    | ((Abs(d1), s1), (Abs(d2), s2)) =>
      switch (d1, d2) {
      | (Some(_), None)
      | (None, Some(_)) => false
      | (None, None) => segEq(s1, s2)
      | (Some(d1), Some(d2)) =>
        String.compare(d1, d2) === 0 && segEq(s1, s2)
      }
    | ((Rel(w1, r1), s1), (Rel(w2, r2), s2)) =>
      w1 === w2 && r1 === r2 && segEq(s1, s2)
    };

let absoluteEq = eq;

let relativeEq = eq;

let testForPath = s =>
  switch (absolute(s)) {
  | Some(abs) => Some(Absolute(abs))
  | None =>
    switch (relative(s)) {
    | Some(r) => Some(Relative(r))
    | None => None
    }
  };

let firstClass: type k. t(k) => firstClass =
  p =>
    switch (p) {
    | (Abs(d), s) => Absolute((Abs(d), s))
    | (Rel(w, r), s) => Relative((Rel(w, r), s))
    };

let testForPathExn = s =>
  switch (testForPath(s)) {
  | Some(res) => res
  | None => raise(Invalid_argument("Path neither absolute nor relative."))
  };

let continue = (s, path) => List.fold_left(parseNextToken, path, lex(s));

let rec join: type k1 k2. (t(k1), t(k2)) => t(k1) =
  (p1, p2) =>
    switch (p1, p2) {
    | ((Rel(w, r1), []), (Rel(Any, r2), s2)) => (Rel(w, r1 + r2), s2)
    | ((Rel(w, r1), [s1hd, ...s1tl] as s1), (Rel(Any, r2), s2)) =>
      r2 > 0 ?
        join((Rel(w, r1), s1tl), (Rel(Any, r2 - 1), s2)) :
        (Rel(w, r1), List.append(s2, s1))
    | ((b1, s1), (Rel(Home, r2), s2)) =>
      join((b1, [homeChar, ...List.append(s2, s1)]), (Rel(Any, r2), s2))
    | ((b1, s1), (Abs(Some(ll)), s2)) => (
        b1,
        [ll, ...List.append(s2, s1)],
      )
    | ((b1, s1), (Abs(None), s2)) => (b1, List.append(s2, s1))
    | ((Abs(_) as d, []), (Rel(Any, r2), s2)) => (d, s2)
    | ((Abs(_) as d, [s1hd, ...s1tl] as s1), (Rel(Any, r2), s2)) =>
      r2 > 0 ?
        join((d, s1tl), (Rel(Any, r2 - 1), s2)) :
        (d, List.append(s2, s1))
    };

let dirName: type k1. t(k1) => t(k1) =
  p1 =>
    switch (p1) {
    | (Rel(w, r1), []) => (Rel(w, r1 + 1), [])
    | (Rel(w, r1), [s1hd, ...s1tl]) => (Rel(w, r1), s1tl)
    | (Abs(_) as d, []) => (d, [])
    | (Abs(_) as d, [s1hd, ...s1tl]) => (d, s1tl)
    };

let baseName: type k1. t(k1) => option(string) =
  p1 =>
    switch (p1) {
    | (Rel(w, r1), []) => None
    | (Rel(w, r1), [s1hd, ...s1tl]) => Some(s1hd)
    | (Abs(_), []) => None
    | (Abs(_), [s1hd, ...s1tl]) => Some(s1hd)
    };

let sub: type k1. (string, t(k1)) => t(k1) =
  (name, path) => continue(name, path);

/**
 * Append functions always follow their "natural" left/right ordering,
 * regardless of t-first/last.
 *
 * The following pairs are equivalent but note that `append` is always safe.
 *
 *     Path.append(Path.root, "foo");
 *     Option.getUnsafe(Path.absolute("/foo"));
 *
 *     Path.append(Path.root, "foo/bar");
 *     Option.getUnsafe(Path.absolute("/foo/bar"));
 *
 *     Path.append(Path.drive("C"), "foo/bar");
 *     Option.getUnsafe(Path.absolute("C:/foo/bar"));
 *
 *     Path.append(Path.dot, "foo");
 *     Option.getUnsafe(Path.relative("./foo"));
 */
let append: type k1. (t(k1), string) => t(k1) =
  (path, name) => continue(name, path);

module At = {
  let (/) = append;
  /**
   * Applies `dirName` to the first argument, then passes the result to
   * `append` with the second.
   *
   *     let result = root / "foo" / "bar" /../ "baz";
   *
   * Would result in
   *
   *     "/foo/baz"
   */
  let (/../) = (dir, s) => append(dirName(dir), s);
  let (/../../) = (dir, s) => append(dirName(dirName(dir)), s);
  let (/../../../) = (dir, s) =>
    append(dirName(dirName(dirName(dir))), s);
  let (/../../../../) = (dir, s) =>
    append(dirName(dirName(dirName(dirName(dir)))), s);
  let (/../../../../../) = (dir, s) =>
    append(dirName(dirName(dirName(dirName(dirName(dir))))), s);
  let (/../../../../../../) = (dir, s) =>
    append(
      dirName(dirName(dirName(dirName(dirName(dirName(dir)))))),
      s,
    );
};
