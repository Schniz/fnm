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

let getStatus = string => {
  List.nth(String.split_on_char(' ', string), 1);
};

exception Unknown_status_code(response);
exception Not_found(response);
exception Internal_server_error(response);

let verifyStatus = response => {
  switch (response.status) {
  | 200 => Lwt.return(response)
  | x when x / 100 == 4 => Lwt.fail(Not_found(response))
  | x when x / 100 == 5 => Lwt.fail(Internal_server_error(response))
  | _ => Lwt.fail(Unknown_status_code(response))
  };
};

let parseResponse = lines => {
  let body = getBody(lines);
  let status = getStatus(lines |> List.hd) |> int_of_string;
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
      ~args=[|url, "-D", "-", "--silent", "-o", into|],
    );
  response |> parseResponse |> verifyStatus;
};
