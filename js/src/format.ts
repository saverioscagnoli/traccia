import chalk from "chalk";
import { LogLevel, LogRecord } from ".";

/**
 * Strips ANSI escape codes from a string.
 * This is useful for cleaning up log messages before writing them to a file or displaying them in
 * a non-ANSI-capable environment, like a file.
 *
 * @param str String to strip ANSI escape codes from.
 * @returns A string with ANSI escape codes removed.
 */
function stripAnsi(str: string): string {
  let st = "(?:\\u0007|\\u001B\\u005C|\\u009C)";
  let pattern = [
    `[\\u001B\\u009B][[\\]()#;?]*(?:(?:(?:(?:;[-a-zA-Z\\d\\/#&.:=?%@~_]+)*|[a-zA-Z\\d]+(?:;[-a-zA-Z\\d\\/#&.:=?%@~_]*)*)?${st})`,
    "(?:(?:\\d{1,4}(?:;\\d{0,4})*)?[\\dA-PR-TZcf-nq-uy=><~]))",
  ].join("|");

  let regex = new RegExp(pattern, "g");

  return str.replace(regex, "").replace(/\u001B\[\d+m/g, "");
}

/**
 * Formatter interface for log records.
 * This interface defines a method to format log records into a string representation.
 * Implementations can customize the format of the log messages.
 *
 * Adapt this interface to customize the way log records are formatted.
 *
 * If you don't provide a custom formatter, the DefaultFormatter will be used by default.
 *
 * @interface Formatter
 * @property {function} format - Method to format a log record.
 * @param {LogRecord} record - The log record to format.
 * @returns {string} The formatted log message.
 * @example
 * const formatter: Formatter = {
 *   format(record: LogRecord): string {
 *     return `[${record.timestamp.toISOString()}] [${LogLevel[record.level]}]: ${record.message}`;
 *   }
 * };
 */
interface Formatter {
  format(record: LogRecord, context: Record<string, any>): string;
}

/**
 * DefaultFormatter is the default implementation of the Formatter interface.
 * It formats log records into a string representation with ANSI colors for different log levels.
 * The format includes a timestamp, log level, and message.
 *
 * @class DefaultFormatter
 * @implements {Formatter}
 */
class DefaultFormatter implements Formatter {
  format(record: LogRecord, context: Record<string, any>): string {
    let levelColor: (level: string) => string;

    switch (record.level) {
      case LogLevel.Trace:
        levelColor = chalk.cyan;
        break;
      case LogLevel.Debug:
        levelColor = chalk.blue;
        break;
      case LogLevel.Info:
        levelColor = chalk.green;
        break;
      case LogLevel.Warn:
        levelColor = chalk.yellow;
        break;
      case LogLevel.Error:
        levelColor = chalk.red;
        break;
      case LogLevel.Fatal:
        levelColor = chalk.redBright;
        break;
    }

    let metadataOutput = "";

    if (record.metadata) {
      for (const [key, value] of Object.entries(record.metadata)) {
        if (metadataOutput.length > 0) {
          metadataOutput += ", ";
        }

        if (typeof value === "object") {
          metadataOutput += `${key}=${JSON.stringify(value, null, 2)}`;
        } else {
          metadataOutput += `${key}=${value}`;
        }
      }
    }

    let contextOutput = "";

    for (const [key, value] of Object.entries(context)) {
      if (contextOutput.length > 0) {
        contextOutput += ", ";
      }

      if (typeof value === "object") {
        contextOutput += `${key}=${JSON.stringify(value, null, 2)}`;
      } else {
        contextOutput += `${key}=${value}`;
      }
    }

    let t = record.timestamp.toISOString().replace("T", " ").replace("Z", "");

    return `[${chalk.gray(t)}] [${levelColor(
      LogLevel[record.level].toUpperCase()
    )}]: ${record.message} ${metadataOutput} ${contextOutput}`;
  }
}

export { DefaultFormatter, stripAnsi };
export type { Formatter };
