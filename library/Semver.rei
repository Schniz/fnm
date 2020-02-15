[@deriving (eq, ord)]
type t;

let make: (~major: int, ~minor: int, ~patch: int) => t;

let fromString: string => option(t);
let toString: t => string;

let major: t => int;
let minor: t => int;
let patch: t => int;

let isMajor: (int, t) => bool;