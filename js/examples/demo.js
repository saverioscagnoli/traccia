import traccia, { Console, LogLevel } from "../dist/index.js";

traccia.init({
  level: LogLevel.Trace,
  targets: [new Console()],
});

traccia.set("userId", "12345");

traccia.trace("This is a trace message");
traccia.errorWithMetadata(
  { errorCode: 404 },
  "The requested resource was not found."
);

traccia.clear("userId");

traccia.debug("This is a debug message");
traccia.info("This is an info message");
traccia.warn("This is a warning message");
traccia.error("This is an error message");
traccia.fatal("This is a fatal message");
