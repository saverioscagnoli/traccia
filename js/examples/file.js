import traccia, { Console, File } from "../dist/index.js";

traccia.init({
  level: traccia.LogLevel.Trace,
  targets: [new Console(), new File("./.logs/example.log", "overwrite")],
});

traccia.info("Hello world with file logging!");
