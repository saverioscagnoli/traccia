import { Config } from "./config";
import { DefaultFormatter } from "./format";
import { Console } from "./target";
import { CallSite, callsite } from "./utils";

/**
 * Represents a single log record with all its associated metadata.
 */
type LogRecord = {
  level: LogLevel;
  threadID: number;
  timestamp: Date;
  messages: (string | object | number | boolean | null | undefined)[];
  callsite: CallSite;
};

/**
 * Enumeration of available log levels in order of severity.
 * Lower numeric values represent less severe log levels.
 */
enum LogLevel {
  Trace = 0,
  Debug = 1,
  Info = 2,
  Warn = 3,
  Error = 4,
  Fatal = 5,
}

class Logger {
  static #instance: Logger;

  #config: Config;
  context: Map<string, any> = new Map();

  private constructor(config: Config) {
    this.#config = config;
  }

  /**
   * Creates or returns the singleton Logger instance.
   * @param config - The logger configuration. Required on first call.
   * @returns The Logger singleton instance.
   * @throws {Error} When config is not provided and no instance exists.
   */
  public static build(config?: Config): Logger {
    if (Logger.#instance) {
      return Logger.#instance;
    }

    if (!config) {
      throw new Error();
    }

    Logger.#instance = new Logger(config!);

    return Logger.#instance;
  }

  #enabled(level: LogLevel): boolean {
    return this.#config.level! <= level;
  }

  /**
   * Logs a message at the specified level with optional metadata.
   * @param level - The log level for this message.
   * @param metadata - Additional metadata to include with the log record.
   * @param message - The message(s) to log. Objects will be JSON stringified.
   */
  public log(level: LogLevel, ...messages: any[]): void {
    if (!this.#enabled(level)) {
      return;
    }

    let record: LogRecord = {
      level,
      threadID: process.pid,
      timestamp: new Date(),
      messages,
      callsite: callsite(3),
    };

    for (const target of this.#config.targets!) {
      target.write(
        level,
        this.#config.formatter!.format(record, Object.fromEntries(this.context))
      );
    }
  }
}

/**
 * Initializes the logger with the provided configuration.
 * If no configuration is provided, uses default settings (Info level, DefaultFormatter, Console target).
 * @param config - Optional logger configuration. If not provided, defaults will be used.
 */
function init(config?: Config): void {
  if (!config) {
    config = {
      level: LogLevel.Info,
      formatter: new DefaultFormatter(),
      targets: [new Console()],
    };
  }

  if (config.level === undefined || config.level === null) {
    config.level = LogLevel.Info;
  }

  if (!config.formatter) {
    config.formatter = new DefaultFormatter();
  }

  if (!config.targets || config.targets.length === 0) {
    config.targets = [new Console()];
  }

  Logger.build(config);
}

/**
 * Sets a context value that will be included in all subsequent log records.
 * @param key - The context key.
 * @param value - The context value.
 */
function set(key: string, value: any): void {
  Logger.build().context.set(key, value);
}

/**
 * Removes a specific context key from the logger context.
 * @param key - The context key to remove.
 */
function clear(key: string): void {
  Logger.build().context.delete(key);
}

/**
 * Clears all context values from the logger.
 */
function clearAll(): void {
  Logger.build().context.clear();
}

/**
 * Logs a message at the specified level without additional metadata.
 * @param level - The log level for this message.
 * @param message - The message(s) to log. Objects will be JSON stringified.
 */
function log(level: LogLevel, ...message: any[]): void {
  Logger.build().log(level, ...message);
}

/**
 * Logs a message at the Trace level.
 * @param message - The message(s) to log. Objects will be JSON stringified.
 */
function trace(...message: any[]): void {
  log(LogLevel.Trace, ...message);
}

/**
 * Logs a message at the Debug level.
 * @param message - The message(s) to log. Objects will be JSON stringified.
 */
function debug(...message: any[]): void {
  log(LogLevel.Debug, ...message);
}

/**
 * Logs a message at the Info level.
 * @param message - The message(s) to log. Objects will be JSON stringified.
 */
function info(...message: any[]): void {
  log(LogLevel.Info, ...message);
}

/**
 * Logs a message at the Warn level.
 * @param message - The message(s) to log. Objects will be JSON stringified.
 */
function warn(...message: any[]): void {
  log(LogLevel.Warn, ...message);
}

/**
 * Logs a message at the Error level.
 * @param message - The message(s) to log. Objects will be JSON stringified.
 */
function error(...message: any[]): void {
  log(LogLevel.Error, ...message);
}

/**
 * Logs a message at the Fatal level.
 * @param message - The message(s) to log. Objects will be JSON stringified.
 */
function fatal(...message: any[]): void {
  log(LogLevel.Fatal, ...message);
}

export default {
  init,
  set,
  clear,
  clearAll,
  log,
  trace,
  debug,
  info,
  warn,
  error,
  fatal,
  LogLevel,
  DefaultFormatter,
  Console,
  File,
};

export type { LogRecord };
export { LogLevel };

/**
 * Re-Exports
 */
export type { Config } from "./config";

export { DefaultFormatter } from "./format";
export type { Formatter } from "./format";

export type { Target, FileMode } from "./target";
export { File, Console } from "./target";
