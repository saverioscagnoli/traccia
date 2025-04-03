All the dates in this changelog are formatted as day/month/year.

# 0.2.0 - 3/3/2025

- Added `Debug, Clone, Copy, PartialEq, Eq` to `Color`.

- Added `mode` parameter to the file target,
  to specify whether to append to or to truncate the file on start.

- Added a `thread_id` field to the record structure, to identify the thread
  where the log was generated.

  - The `threads` example was updated to demonstrate this functionality.

# 0.2.1 - 3/3/2025

- Added some preview image to README.

# 0.2.2 - 3/3/2025

- Removed the docs folder from the package, that caused the size to go up to 450 KiB
- Renamed CHANGELOG to CHANGELOG.md
- Replaced `///` comments with `//!` comments in `lib.rs`

# 1.2.2 - 4/3/2025

- Removed the `flush` function and changed all examples accordingly.
- Added a wrapper around libc's `atexit` to create a shutdown hook that flushes the log buffer automatically.

# 1.3.2 - 5/3/2025

- Added the `style` trait, that enables the user for more string customization.
- Added the `background` function to the `Colorize` trait, so that the user can change the background color of the contents.

# 1.4.2 - 9/3/2025

- Added the `Fatal` LogLevel variant, useful for logging errors that cause the program to stop.
- Added the possibility to filter logs for specific targets (like only logging fatal errors to files.)

# 2.0.0 - 4/4/2025

- Added the `level` parameter to the `write` trait function for `Target`, which breaks current custom implementations.

- Added the possibility to make console write to a custom output `stdout` or `stderr`, both globally and per-level.