# disk-check

Pre-dispatch disk space gate: validates free disk before agent dispatch.

## Usage

```bash
disk-check [OPTIONS]

Options:
  --min-gb <MIN_GB>          Minimum free space required in GB [default: 30]
  --warn-gb <WARN_GB>        Warning threshold in GB (exit 2) [default: 10]
  --path <PATH>              Path to check [default: /System/Volumes/Data]
  --verbose                  Verbose output
  --help                      Print help
```

## Exit Codes

- **0**: OK (available space >= min-gb)
- **1**: CRITICAL (available space < warn-gb)
- **2**: WARNING (available space between warn-gb and min-gb)

## Integration with agent-orchestrator

The `agent-orchestrator` dispatch path wires `disk-check` as a pre-dispatch gate:

```rust
// agent-orchestrator/src/main.rs::cmd_lanes_dispatch()
disk_check::invoke_disk_check_gate(10, true)?;  // Block if <10GB, warn if approaching
```

**Flow:**
1. Agent dispatch triggered (`agent-orchestrator lanes dispatch <lane-id>`)
2. Before generating dispatch prompt, invoke `disk-check --min-gb 10 --verbose`
3. Exit code 0 → proceed with dispatch
4. Exit code 1 → abort dispatch (insufficient disk)
5. Exit code 2 → warn but proceed (disk approaching limits)

**Binary location:** Must be in PATH (install via `cargo install` in this directory)

## Testing

```bash
cargo test --lib
```

## Installation

```bash
# From repos root
cd FocalPoint/tooling/disk-check
cargo install --path .
```

Or add to your PATH after building:
```bash
cargo build --release
export PATH="$PATH:$(pwd)/target/release"
```
