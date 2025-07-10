// @ts-check
/// <reference types="../dist/index.d.ts" />

import traccia from "../dist/index.js";

/**
 * Custom formatter implementation that implements the Formatter interface.
 *
 * @typedef {import("../dist/index.js").Formatter} Formatter
 * @typedef {import("../dist/index.js").LogRecord} LogRecord
 *
 * @class
 * @implements {Formatter}
 */
class CustomFormatter {
  /**
   * @param {LogRecord} record - The log record containing level, timestamp, messages, and callsite info
   * @param {Record<string, any>} context - Additional context data for formatting
   * @returns {string} The formatted log message
   */
  format(record, context) {
    let message = record.messages
      .map((m) => (typeof m === "object" ? JSON.stringify(m) : m?.toString()))
      .join(" ");

    return `Incredible formatter for this message: ${message} at ${record.callsite.file}:${record.callsite.line}:${record.callsite.column}`;
  }
}

traccia.init({
  formatter: new CustomFormatter(),
});

traccia.warn("This is a warning message with a custom formatter");
