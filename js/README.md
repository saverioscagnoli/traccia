# Traccia ✍️

A flexible and configurable logging library for JavaScript/TypeScript applications.

## Quick Start

```javascript
import traccia from "traccia";

// Initialize with default configuration
traccia.init();

// Start logging
traccia.info("Hello, world!");
traccia.warn("This is a warning");
traccia.error("Something went wrong");
```

## Features

- **Multiple log levels**: Trace, Debug, Info, Warn, Error, Fatal
- **Configurable targets**: Console, File, and custom targets
- **Custom formatters**: Create your own log formatting logic
- **Context support**: Add contextual data to log records
- **TypeScript support**: Full type definitions included

## Configuration

Initialize traccia with custom configuration:

```javascript
import traccia from "traccia";

traccia.init({
  level: traccia.LogLevel.Debug,
  targets: [new traccia.Console()],
  formatter: new traccia.DefaultFormatter(),
});
```

## Examples

Check out the `examples/` directory for comprehensive usage examples:

- **`demo.js`** - Basic logging with different levels
- **`file.js`** - Logging to files with custom targets
- **`formatter.js`** - Custom formatters with JSDoc implementation

Run examples:

```bash
npm run example:demo
npm run example:file
npm run example:formatter
```

## API

### Log Levels

- `traccia.trace(message)`
- `traccia.debug(message)`
- `traccia.info(message)`
- `traccia.warn(message)`
- `traccia.error(message)`
- `traccia.fatal(message)`

### Configuration Options

- `level`: Minimum log level to output
- `targets`: Array of output targets (Console, File, etc.)
- `formatter`: Custom formatter for log messages

## License

MIT
