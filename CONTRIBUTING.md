# Contributing to pipespy

Thanks for your interest in contributing! Here's how to get started.

## Development Setup

```bash
git clone https://github.com/jasonm4130/pipespy.git
cd pipespy
cargo build
```

**Requirements:**
- Rust 1.70+ (install via [rustup](https://rustup.rs))

## Running Tests

```bash
# All tests
cargo test

# Unit tests only
cargo test --lib

# Integration tests only
cargo test --test passthrough
cargo test --test format_detection
```

## Testing the TUI

The TUI can't be tested automatically, so manual testing is important:

```bash
# Compact mode with plain text
seq 1 100000 | cargo run

# Fullscreen mode
seq 1 100000 | cargo run -- --fullscreen

# JSON format detection
for i in $(seq 1 5000); do echo "{\"id\": $i, \"name\": \"user$i\"}"; done | cargo run

# CSV format detection
echo -e "id,name,age\n1,alice,30\n2,bob,25" | cargo run

# Quiet mode
seq 1 100000 | cargo run -- --quiet

# Piped output (data should pass through)
seq 1 100 | cargo run -- --quiet | wc -l  # should print 100
```

## Project Structure

```
src/
├── main.rs          Entry point, thread orchestration
├── lib.rs           Module exports
├── cli.rs           CLI argument definitions (clap)
├── buffer.rs        Thread-safe ring buffer
├── stats.rs         Throughput tracking, sparkline history
├── format.rs        JSON/CSV/text format detection
├── pipeline.rs      Reader and writer thread functions
├── highlight.rs     Syntax highlighting for records
└── tui/
    ├── mod.rs       TUI app state, event loop
    ├── compact.rs   Compact mode layout
    └── fullscreen.rs Fullscreen mode layout

tests/
├── passthrough.rs      Data integrity (stdin == stdout)
└── format_detection.rs Format sniffing correctness
```

## Pull Requests

1. Fork the repo and create your branch from `main`
2. Add tests for any new functionality
3. Ensure `cargo test` passes
4. Ensure `cargo clippy` has no warnings
5. Update documentation if you changed behavior
6. Open a PR with a clear description of the change

## Reporting Issues

[Open an issue](https://github.com/jasonm4130/pipespy/issues/new) with:
- What you expected to happen
- What actually happened
- Steps to reproduce
- Your OS and terminal emulator

## Code Style

- Follow standard Rust conventions (`cargo fmt`)
- Keep files focused — one responsibility per module
- No unnecessary abstractions (YAGNI)
- Tests for all non-TUI logic

## License

By contributing, you agree that your contributions will be licensed under the [MIT License](LICENSE).
