let return = x => Ok(x);

let both = (a, b) =>
  switch (a, b) {
  | (Error(_) as e, _)
  | (_, Error(_) as e) => e
  | (Ok(ax), Ok(bx)) => Ok((ax, bx))
  };

let mapError = (fn, res) =>
  switch (res) {
  | Error(x) => Error(fn(x))
  | Ok(_) as x => x
  };

let map = (fn, res) =>
  switch (res) {
  | Ok(x) => Ok(fn(x))
  | Error(_) as e => e
  };

let bind = (fn, res) =>
  switch (res) {
  | Ok(x) => fn(x)
  | Error(_) as e => e
  };

let fold = (error, ok, res) =>
  switch (res) {
  | Ok(x) => ok(x)
  | Error(x) => error(x)
  };

module Let_syntax = {
  let map = (x, ~f) => map(f, x);
  let bind = (x, ~f) => bind(f, x);
};

let toLwt = res =>
  switch (res) {
  | Error(x) => Lwt.fail_with(x)
  | Ok(x) => Lwt.return(x)
  };
