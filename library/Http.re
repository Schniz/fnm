type response = {
  body: string,
  status: int,
};

let body = response => response.body;
let status = response => response.status;

let rec getBody = listOfStrings => {
  switch (listOfStrings) {
  | [] => ""
  | ["", ...rest] => String.concat("\n", rest)
  | [_, ...xs] => getBody(xs)
  };
};

let rec getStatus = string => {
  List.nth(String.split_on_char(' ', string), 1) |> int_of_string;
};

exception Unknown_status_code(int, response);
exception Not_found(response);
exception Internal_server_error(response);

let verifyStatus = response => {
  switch (response.status) {
  | 200 => Lwt.return(response)
  | x when x / 100 == 4 => Lwt.fail(Not_found(response))
  | x when x / 100 == 5 => Lwt.fail(Internal_server_error(response))
  | x => Lwt.fail(Unknown_status_code(response.status, response))
  };
};

let rec skipRedirects = (~skipping=false, lines) => {
  switch (skipping, lines) {
  | (_, []) => failwith("Response is empty")
  | (true, ["", ...xs]) => skipRedirects(~skipping=false, xs)
  | (true, [x, ...xs]) => skipRedirects(~skipping=true, xs)
  | (false, [x, ...xs])
      when Str.first_chars(x, 4) == "HTTP" && getStatus(x) / 100 == 3 => [
      x,
      ...xs,
    ]
  | (false, xs) => xs
  };
};

let parseResponse = lines => {
  let linesAfterRedirect = skipRedirects(lines);
  let body = getBody(linesAfterRedirect);
  let status = getStatus(linesAfterRedirect |> List.hd);
  {body, status};
};

let makeRequest = url => {
  let%lwt response =
    System.unix_exec("curl", ~args=[|url, "-D", "-", "--silent"|]);
  response |> parseResponse |> verifyStatus;
};

let download = (url, ~into) => {
  let%lwt response =
    System.unix_exec(
      "curl",
      ~args=[|url, "-L", "-D", "-", "--silent", "-o", into|],
    );
  response |> parseResponse |> verifyStatus;
};
