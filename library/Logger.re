let configuredLogLevel = Config.FNM_LOGLEVEL.get();

let log = message => {
  switch (configuredLogLevel) {
  | LogLevel.All => Console.log(message)
  | LogLevel.Error
  | LogLevel.Quiet => ()
  };
};

let error = message => {
  switch (configuredLogLevel) {
  | LogLevel.All
  | LogLevel.Error => Console.error(message)
  | LogLevel.Quiet => ()
  };
};
