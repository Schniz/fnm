let orThrow = (message, opt) =>
  switch (opt) {
  | None => failwith(message)
  | Some(x) => x
  };

let map = (fn, opt) =>
  switch (opt) {
  | None => None
  | Some(x) => Some(fn(x))
  };

let bind = (fn, opt) =>
  switch (opt) {
  | None => None
  | Some(x) => fn(x)
  };

let fold = (none, some, opt) =>
  switch (opt) {
  | None => none()
  | Some(x) => some(x)
  };

let toResult = (error, opt) =>
  switch (opt) {
  | None => Error(error)
  | Some(x) => Ok(x)
  };

let toLwt = (error, opt) =>
  switch (opt) {
  | Some(x) => Lwt.return(x)
  | None => Lwt.fail(error)
  };

let some = x => Some(x);

let (or) = (opt, b) => fold(() => b, x => x, opt);
