import chalk from "chalk";
import { LogLevel, LogRecord } from ".";
import path from "path";

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

    let t = record.timestamp
      .toISOString()
      .replace("T", " ")
      .replace("Z", "")
      .replace(/-/g, "/");

    let timestamp = chalk.gray(t);
    let level = levelColor(LogLevel[record.level].toLowerCase());
    let at = `${record.callsite.line || 0}:${record.callsite.column || 0}`;
    let loc = chalk.gray(
      `${
        record.callsite.file?.split(path.sep).slice(-1).join("") || "<unknown>"
      }@${at}`
    );

    let message = record.messages
      .map((m) => (typeof m === "object" ? JSON.stringify(m) : m?.toString()))
      .join(" ");

    let c = Object.entries(context).length > 0 ? JSON.stringify(context) : "";

    return `[${timestamp}] [${level}] [${loc}]: ${message} ${c}`;
  }
}

export { DefaultFormatter };
export type { Formatter };
