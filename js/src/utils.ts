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

type CallSite = {
  /**
   * The file name of the call site, or null if not available.
   */
  file: string | null;
  /**
   * The line number of the call site, or null if not available.
   */
  line: number | null;
  /**
   * The column number of the call site, or null if not available.
   */
  column: number | null;
  /**
   * The function name of the call site, or null if not available.
   */
  function: string | null;
};

export type { CallSite };

/**
 * Retrieves the call site information from the stack trace.
 * @param depth The depth in the stack trace to retrieve the call site information.
 * @returns An object containing the file name, line number, column number, and function name of the call site.
 */
function callsite(depth: number = 1): CallSite {
  let og = Error.prepareStackTrace;
  Error.prepareStackTrace = (_, stack) => stack;

  let err = new Error();

  Error.captureStackTrace(err, callsite);

  if (!err.stack || err.stack.length <= depth) {
    Error.prepareStackTrace = og;

    return {
      file: null,
      line: null,
      column: null,
      function: null,
    };
  }

  let frame = err.stack[depth] as unknown as NodeJS.CallSite;

  Error.prepareStackTrace = og;

  return {
    file: frame.getFileName(),
    line: frame.getLineNumber(),
    column: frame.getColumnNumber(),
    function: frame.getFunctionName(),
  };
}

export { stripAnsi, callsite };
