<div align="center">

# pipespy

**Real-time pipeline debugger for your terminal.**

`pv` shows bytes — `pipespy` shows your data.

[![Crates.io](https://img.shields.io/crates/v/pipespy.svg)](https://crates.io/crates/pipespy)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Built with Rust](https://img.shields.io/badge/built%20with-Rust-dea584.svg)](https://www.rust-lang.org/)
[![GitHub Release](https://img.shields.io/github/v/release/jasonm4130/pipespy)](https://github.com/jasonm4130/pipespy/releases)

<br>

<img src="assets/demo.gif" alt="pipespy demo" width="800">

</div>

<br>

Drop `pipespy` into any shell pipeline to instantly see what's flowing through — throughput, record samples, format detection, and more. Your data passes through **completely untouched**.

```bash
cat events.jsonl | pipespy | jq '.users[]' | grep "active" > out.txt
```

## Install

**Homebrew** (macOS/Linux):

```bash
brew install jasonm4130/tap/pipespy
```

**Cargo** (requires [Rust](https://rustup.rs)):

```bash
cargo install pipespy
```

<details>
<summary>More install methods</summary>

**Shell one-liner** (download pre-built binary):

```bash
curl -fsSL https://raw.githubusercontent.com/jasonm4130/pipespy/main/install.sh | sh
```

**Build from source:**

```bash
git clone https://github.com/jasonm4130/pipespy.git
cd pipespy
cargo build --release
# Binary at target/release/pipespy
```

Pre-built binaries are available for macOS (arm64/amd64) and Linux (arm64/amd64) on the [releases page](https://github.com/jasonm4130/pipespy/releases).

</details>

## Quick Start

```bash
# See what's flowing through your pipeline
cat server.log | pipespy | grep ERROR > errors.txt

# Fullscreen mode with histogram and extended stats
cat events.jsonl | pipespy --fullscreen | jq '.' > out.json

# Quiet mode for scripts — just the summary
cat huge.jsonl | pipespy -q | jq '.' > out.json
# pipespy: 1,204,831 lines | 482MB | 14.2s | 33.9MB/s
```

## Features

### Two Display Modes

Press `f` to toggle between compact and fullscreen at any time.

**Compact** — fixed height, fits in a split pane. Shows throughput, sparkline, and live record samples.

**Fullscreen** — fills the terminal with extended stats (min/max/avg line size), a throughput history sparkline, line length histogram, and a scrollable record viewer.

<div align="center">
<img src="assets/fullscreen-mode.png" alt="pipespy fullscreen mode" width="800">
<br>
<sub>Fullscreen mode — throughput history, line length histogram, and extended stats</sub>
</div>

<br>

### Format Detection

pipespy automatically detects your data format and adapts the display:

| Format | Detection | Display |
|--------|-----------|---------|
| **JSON** | Valid JSON objects per line | Syntax-highlighted keys, values, numbers |
| **CSV** | Consistent comma-separated columns | Color-coded columns |
| **Plain text** | Everything else | Raw display |

Override with `--json`, `--csv`, or `--no-detect`.

### Transparent Proxy

Every byte that enters stdin exits stdout — **in order, unmodified**. pipespy renders entirely to stderr, so it never interferes with your data pipeline. This is the core correctness guarantee, verified by integration tests.

### Quiet Mode

Skip the TUI entirely. Get a one-line summary when the pipeline completes — perfect for scripts and CI:

```
$ cat access.log | pipespy -q | awk '{print $1}' | sort -u > ips.txt
pipespy: 8,412,093 lines | 1.2GB | 4.7s | 255.3MB/s
```

## Keyboard Shortcuts

| Key | Action |
|:---:|--------|
| `f` | Toggle fullscreen / compact mode |
| `q` | Detach TUI and print summary |

## CLI Reference

```
pipespy [OPTIONS]

Options:
  -f, --fullscreen        Start in fullscreen mode
  -n, --sample-rate <N>   Show 1 in N records (default: auto)
  -b, --buffer <SIZE>     Ring buffer size in bytes (default: 8MB)
      --no-detect         Skip format detection, treat as plain text
      --json              Force JSON mode
      --csv               Force CSV mode
  -q, --quiet             No TUI, just print summary on completion
  -h, --help              Print help
  -V, --version           Print version
```

## Architecture

```
stdin  ──▶  Reader Thread  ──▶  Ring Buffer  ──▶  Writer Thread  ──▶  stdout
                                     │
                               Stats Collector
                                     │
                               TUI Renderer  ──▶  stderr
```

Three threads keep data flowing at full speed:

- **Reader** — pumps stdin into a shared ring buffer, records per-line statistics
- **Writer** — drains the buffer to stdout as fast as downstream can consume
- **TUI** — samples stats on a timer and renders to stderr via [ratatui](https://github.com/ratatui/ratatui)

The TUI thread never touches the data path. Rendering to stderr means the alternate screen, raw mode, and all visual output are completely isolated from your pipeline data.

## Comparison with `pv`

| Feature | `pv` | `pipespy` |
|---------|:----:|:---------:|
| Bytes transferred | :white_check_mark: | :white_check_mark: |
| Line count | :x: | :white_check_mark: |
| Live record samples | :x: | :white_check_mark: |
| Format detection | :x: | :white_check_mark: |
| Syntax highlighting | :x: | :white_check_mark: |
| Throughput sparkline | :x: | :white_check_mark: |
| Line length histogram | :x: | :white_check_mark: |
| Fullscreen TUI | :x: | :white_check_mark: |
| Data integrity | :white_check_mark: | :white_check_mark: |

## Built With

- [Rust](https://www.rust-lang.org/) — zero-cost abstractions, fearless concurrency
- [ratatui](https://github.com/ratatui/ratatui) — terminal UI framework
- [crossterm](https://github.com/crossterm-rs/crossterm) — cross-platform terminal manipulation
- [clap](https://github.com/clap-rs/clap) — CLI argument parsing

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

```bash
# Clone and build
git clone https://github.com/jasonm4130/pipespy.git
cd pipespy
cargo build

# Run tests
cargo test

# Run locally
seq 1 100000 | cargo run
```

## License

This project is licensed under the [MIT License](LICENSE).
