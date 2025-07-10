import { LogLevel } from ".";
import { Formatter } from "./format";
import { Target } from "./target";

/**
 * The configuration object for the logger.
 * It allows you to set the log level, formatter, and targets for logging.
 */
type Config = {
  /**
   * The log level for the logger.
   * If not set, defaults to LogLevel.Info.
   *
   * This determines the minimum level of messages that will be logged.
   * Messages with a level lower than this will be ignored.
   *
   * Fatal > Error > Warn > Info > Debug > Trace
   */
  level?: LogLevel;

  /**
   * The formatter to use for formatting log messages.
   * If not provided, the DefaultFormatter will be used.
   * This allows you to customize the format of log messages.
   */
  formatter?: Formatter;

  /**
   * The target array for logging.
   * When logging, the logger will call the `write` method on each target with the log level and formatted message.
   * If not provided, the logger will use a default target that logs to the console.
   */
  targets?: Target[];
};

export { Config };
