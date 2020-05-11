open Fnm;

let run = () => {
  let%lwt currentVersion = Versions.getCurrentVersion();
  switch (currentVersion) {
  | Some({name}) => Console.log(name)
  | _ => Console.log("none")
  };

  Lwt.return_ok();
};
