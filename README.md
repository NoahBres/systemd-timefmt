# systemd-timefmt

A CLI utility that parses [systemd.time](https://www.freedesktop.org/software/systemd/man/latest/systemd.time.html) format specifications and outputs them in customizable date/time formats.

## Features

- Parse [systemd.time](https://www.freedesktop.org/software/systemd/man/latest/systemd.time.html) format strings (relative times, absolute timestamps, epoch times)
- Output in local time (default) or UTC
- Customizable output format using strftime patterns
- Helpful error messages with suggestions for common mistakes

## Installation

```bash
cargo install --path .
```

## Usage

### Basic Examples

```bash
# Parse relative time
systemd-timefmt "now -1hr"
# Output: 2025-05-24 14:30:00

# Parse relative time with different units
systemd-timefmt "today +2days"
# Output: 2025-05-26 00:00:00

# Parse natural language time expressions
systemd-timefmt "11min ago"
# Output: 2025-05-24 15:19:00

systemd-timefmt "+3hr"
# Output: 2025-05-24 18:30:00

# Parse complex relative times
systemd-timefmt "tomorrow -3h30m"
# Output: 2025-05-25 20:30:00

# Parse epoch timestamps
systemd-timefmt "@1529578800s"
# Output: 2018-06-21 11:00:00

# Parse absolute timestamps
systemd-timefmt "2023-12-25 15:30:00"
# Output: 2023-12-25 15:30:00
```

### Output Options

#### UTC Time

Use the `--utc` flag to output time in UTC instead of local time:

```bash
systemd-timefmt "now -1hr" --utc
# Output: 2025-05-24 22:30:00
```

#### Custom Format

Use the `--format` option to customize the output format using [strftime](https://docs.rs/chrono/latest/chrono/format/strftime/index.html) patterns:

```bash
systemd-timefmt "2023-12-25 15:30:00" --format "%B %d, %Y at %I:%M %p"
# Output: December 25, 2023 at 03:30 PM

systemd-timefmt "now" --format "%A, %Y-%m-%d %H:%M:%S %Z"
# Output: Saturday, 2025-05-24 15:30:00 PDT
```

### Command Line Options

```
systemd-timefmt [OPTIONS] <TIMESPEC>

Arguments:
  <TIMESPEC>  Timestamp specification following systemd.time format

Options:
      --utc              Output UTC time instead of local time
      --format <FORMAT>  Format string for output (default: "%Y-%m-%d %H:%M:%S")
                         Supports strftime formatting: https://docs.rs/chrono/latest/chrono/format/strftime/index.html
  -h, --help             Print help (see more with '--help')
  -V, --version          Print version
```

## Supported Time Formats

This tool supports the systemd.time specification, including:

### Relative Times
- `now`, `today`, `yesterday`, `tomorrow`
- Time arithmetic: `now -1hr`, `today +2days`, `tomorrow -3h30m`

### Time Units
- `usec`, `us`, `µs` (microseconds)
- `msec`, `ms` (milliseconds)  
- `seconds`, `second`, `sec`, `s`
- `minutes`, `minute`, `min`, `m`
- `hours`, `hour`, `hr`, `h`
- `days`, `day`, `d`
- `weeks`, `week`, `w`
- `months`, `month`, `M` (defined as 30.44 days)
- `years`, `year`, `y` (defined as 365.25 days)

### Absolute Times
- `2018-08-20 09:11:12`
- `18-08-20 09:11:12`
- `09:11:12`
- `11:12`

### Epoch Times
- `@1529578800s` (Unix timestamp)
- `epoch +1529578800s`

### Natural Language
- `3s ago`
- `4h50m left`
- `11min ago`

## Error Handling

The tool provides helpful error messages and suggestions. For example, if you write `now-1hr` (without a space), it will suggest the correct format:

```
You wrote `now-1hr` which uses a special token (`now`) with an attached offset.
According to the systemd.time specification, a space is required after `now`.

    Try writing: `now -1hr`

For more details, see https://www.freedesktop.org/software/systemd/man/latest/systemd.time.html
```

## References

For complete details on the systemd.time format specification, see:
https://www.freedesktop.org/software/systemd/man/latest/systemd.time.html
