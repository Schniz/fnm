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

exception Unknown_status_code(int, response);
exception Not_found(response);
exception Internal_server_error(response);

let makeRequest = url =>
  Uri.of_string(url)
  |> Cohttp_lwt_unix.Client.get
  >>= (
    ((resp, body)) => {
      let status = resp |> Response.status |> Code.code_of_status;
      let%lwt body = body |> Cohttp_lwt.Body.to_string;

      Lwt.return({status, body});
    }
  );

let download = (url, ~into) => {
  let%lwt _ = makeRequest(url) >>= (({body}) => Fs.writeFile(into, body));
  Lwt.return();
};