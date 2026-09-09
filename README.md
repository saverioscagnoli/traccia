<h1 align="center">traccia</h1>

<p align="center">
  A small logging framework for Rust, built on top of the
  <a href="https://docs.rs/log"><code>log</code></a> facade.
</p>

<p align="center">
  <a href="https://crates.io/crates/traccia"><img alt="crates.io" src="https://img.shields.io/crates/v/traccia.svg?style=flat-square"></a>
  <a href="https://crates.io/crates/traccia"><img alt="downloads" src="https://img.shields.io/crates/d/traccia.svg?style=flat-square"></a>
  <a href="https://docs.rs/traccia"><img alt="docs.rs" src="https://img.shields.io/docsrs/traccia?style=flat-square"></a>
  <a href="#license"><img alt="license" src="https://img.shields.io/crates/l/traccia.svg?style=flat-square"></a>
  <img alt="no_std" src="https://img.shields.io/badge/no__std-compatible-blue?style=flat-square">
</p>

You get the usual `trace!`, `debug!`, `info!`, `warn!` and `error!` macros, plus configurable
output targets, formatting and per module levels. Because it implements `log::Log`, anything
in your dependency tree that logs through `log` goes through traccia too.

## Install

```toml
[dependencies]
traccia = "4.1.2"
```

## Usage

```rust
use traccia::{Config, info, warn};

fn main() {
    traccia::init(Config::new()).unwrap();

    info!("listening on port {}", 8080);
    warn!("disk usage at {}%", 91);
}
```

`Config::new()` writes to stderr at the info level. `Config::empty()` starts with no targets
at all, so you can pick your own:

```rust
use traccia::{Config, Console, File, LevelFilter, Output};

traccia::init(
    Config::empty()
        .with_min_level(LevelFilter::Debug)
        .with_target(Console::new(Output::Stdout, true))
        .with_target(File::new("app.log", true)?)
        .with_module_filter("myapp::db", LevelFilter::Trace),
)
.unwrap();
```

Module filters override the minimum level for a module and everything under it. The longest
matching prefix wins, so `myapp::db` beats `myapp`.

To change how lines look, implement `Formatter` and pass it to `with_formatter`:

```rust
use traccia::{Color, Colored, Formatter, Record};

struct Compact;

impl Formatter for Compact {
    fn format(&self, record: &Record) -> String {
        format!("{} {}", record.level().to_string().foreground_color(Color::Green), record.args())
    }
}
```

## Color

`Console::default()` and `Console::auto(output)` turn color on only when the stream is a
terminal and `NO_COLOR` is unset. `Console::new(output, colored)` and `with_color(bool)` skip
the detection if you want to decide yourself. When color is off, escape sequences are stripped
from the formatted line rather than passed through, which is also what the file target does.

## fatal!

`fatal!` logs at the error level, flushes every target and then exits the process:

```rust
let Some(config) = load_config(path) else {
    fatal!(code: 78, "could not read {}", path);
};
```

The exit code defaults to 1 if you leave off `code:`. It needs the `std` feature, since there
is no process to exit without it.

## no_std

The `std` feature is on by default. Turn it off and the crate is `no_std`, needing only a
global allocator:

```toml
[dependencies]
traccia = { version = "4.1.0", default-features = false }
```

Levels, module filters, `Formatter`, `Colored`, `Style` and `strip_ansi` work the same. What
goes away is everything that needs the operating system: the `Console` and `File` targets, and
the `fatal!` macro. `Config::new()` then starts with no targets at all, so implement `Target`
for whatever you write to:

```rust
use core::error::Error;

use traccia::{Config, Level, Target};

struct Serial;

impl Target for Serial {
    fn write(&self, _: Level, message: &str) -> Result<(), Box<dyn Error>> {
        // however this board writes bytes
        Ok(())
    }
}

traccia::init(Config::new().with_target(Serial)).unwrap();
```

## Examples

Run any of them with `cargo run --example <name>`: `basic`, `levels`, `output`, `file`,
`module-filter`, `custom-format`, `custom-target`, `styling`, `threads`, `fatal`.

## License

MIT
