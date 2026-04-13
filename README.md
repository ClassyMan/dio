# dio

Real-time disk I/O monitor for Linux with terminal charts.

Reads `/proc/diskstats` and per-process I/O from `/proc/<pid>/io` to display live throughput, IOPS, and latency sparklines using [ratatui](https://github.com/ratatui/ratatui).

![dio screenshot](screenshot.png)

## Features

- Per-device sparkline charts (throughput, IOPS, latency)
- Per-process I/O table with sortable columns
- Tab to cycle views, `d`/`D` to switch devices
- Configurable refresh rate, scrollback, and device filtering
- `?` for keybinding help

## Install

```bash
cargo install --path .
```

Or grab the binary from `target/release/dio` after:

```bash
cargo build --release
```

## Usage

```
dio                  # default 500ms refresh
dio -r 1000          # 1s refresh
dio -a               # include loopback/ram devices
dio -s 600           # 10 minutes of scrollback
```

## Disclaimer

This software was generated with the assistance of AI (Claude, Anthropic). It is provided **as-is**, with **no warranty of any kind**, express or implied. The author(s) accept **no responsibility or liability** for any damage, data loss, or other issues arising from its use. Use entirely at your own risk. You are solely responsible for reviewing the code and determining its suitability for your environment before running it.

## License

MIT
