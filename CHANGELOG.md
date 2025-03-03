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