import { LogLevel } from ".";
import fs from "fs";
import { stripAnsi } from "./format";

/**
 * Interface for log output targets.
 * Defines the contract for where log messages should be written.
 */
interface Target {
  /**
   * Writes a formatted log message to the target destination.
   * @param level - The log level of the message.
   * @param formatted - The pre-formatted log message string.
   */
  write(level: LogLevel, formatted: string): void;
}

/**
 * File writing mode for the File target.
 * - "append": Add new log entries to the end of the file
 * - "overwrite": Clear the file and start fresh
 */
type FileMode = "append" | "overwrite";

/**
 * File target that writes log messages to a file on disk.
 * Supports both append and overwrite modes, and automatically strips ANSI color codes.
 */
class File implements Target {
  #filePath: string;

  /**
   * Creates a new File target.
   * @param filePath - The path to the log file. File will be created if it doesn't exist.
   * @param fileMode - Whether to append to or overwrite the file. Defaults to "append".
   * @throws {Error} When an invalid file mode is provided.
   */
  public constructor(filePath: string, fileMode: FileMode = "append") {
    if (fileMode !== "append" && fileMode !== "overwrite") {
      throw new Error("Invalid file mode. Use 'append' or 'overwrite'.");
    }
    this.#filePath = filePath;
    if (fileMode === "overwrite") {
      // Clear the file if it exists
      fs.writeFileSync(this.#filePath, "", "utf8");
    }
    if (!fs.existsSync(this.#filePath)) {
      // Create the file if it does not exist
      fs.writeFileSync(this.#filePath, "", "utf8");
    }
  }

  /**
   * Writes a log message to the file.
   * ANSI color codes are automatically stripped before writing.
   * @param _ - The log level (not used by this implementation).
   * @param formatted - The formatted log message to write.
   */
  write(_: LogLevel, formatted: string): void {
    fs.appendFileSync(this.#filePath, stripAnsi(formatted + "\n"), "utf8");
  }
}

/**
 * Console target that writes log messages to the standard output.
 * Preserves ANSI color codes for colored terminal output.
 */
class Console implements Target {
  /**
   * Creates a new Console target.
   */
  public constructor() {}

  /**
   * Writes a log message to the console.
   * @param _ - The log level (not used by this implementation).
   * @param formatted - The formatted log message to write.
   */
  write(_: LogLevel, formatted: string): void {
    console.log(formatted);
  }
}

export { File, Console };
export type { Target, FileMode };
