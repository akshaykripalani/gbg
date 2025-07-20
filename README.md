# gbg

Go (to) BackGround, a hypersimple cli tool to run any command in the background from your shell.

## Install

```bash
cargo install gbg
```

## Usage

```text
gbg [-l LOG_FILE] <command> [args...]
```

- Without `-l` the command's output is discarded (`/dev/null`).
- With `-l` the command's stdout & stderr are appended to `LOG_FILE`.

### Examples

```bash
# Run a long-running script and discard output
gbg python long_task.py

# Run a build and log everything to build.log
gbg -l python long_task.py
```

Built for Linux & macOS. Requires the Rust installed