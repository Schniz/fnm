open Lwt;
open Cohttp;
open Cohttp_lwt;
open Cohttp_lwt_unix;

type response = {
  body: string,
  status: int,
};

let body = response => response.body;
let status = response => response.status;

exception Not_found(response);

let throwOnKnownErrors =
  fun
  | {status: 404} as r => Lwt.fail(Not_found(r))
  | r => Lwt.return(r);

let rec makeRequest = url =>
  Uri.of_string(url)
  |> Cohttp_lwt_unix.Client.get
  >>= (
    ((resp, body)) => {
      let status = resp |> Response.status;
      let code_of_status = status |> Code.code_of_status;
      let location = resp |> Response.headers |> Header.get_location;

      switch (Code.is_redirection(code_of_status), location) {
      | (true, Some(uri)) => makeRequest(Uri.to_string(uri))
      | _ =>
        let%lwt body = body |> Cohttp_lwt.Body.to_string;
        throwOnKnownErrors({status: code_of_status, body});
      };
    }
  );

let download = (url, ~into) => {
  let%lwt _ = makeRequest(url) >>= (({body}) => Fs.writeFile(into, body));
  Lwt.return();
};
