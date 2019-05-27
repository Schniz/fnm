let configuredLogLevel = Config.FNM_LOGLEVEL.get();

let info = message => {
  switch (configuredLogLevel) {
  | LogLevel.Debug
  | LogLevel.Info => Console.log(message)
  | LogLevel.Error
  | LogLevel.Quiet => ()
  };
};

let debug = message => {
  switch (configuredLogLevel) {
  | LogLevel.Debug => Console.log(message)
  | LogLevel.Info
  | LogLevel.Error
  | LogLevel.Quiet => ()
  };
};

let error = message => {
  switch (configuredLogLevel) {
  | LogLevel.Debug
  | LogLevel.Info
  | LogLevel.Error => Console.error(message)
  | LogLevel.Quiet => ()
  };
};
