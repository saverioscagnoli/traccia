import traccia, { Console, LogLevel } from "../dist/index.js";

traccia.init({
  level: LogLevel.Trace,
  targets: [new Console()],
});

traccia.set("userId", "12345");

traccia.trace("This is a trace message");

traccia.clear("userId");

traccia.debug("This is a debug message");
traccia.info("This is an info message");
traccia.warn("This is a warning message");
traccia.error("This is an error message");
traccia.fatal("This is a fatal message");

function login() {
  traccia.info("User logged in", { userId: "user123" });
}

login();
