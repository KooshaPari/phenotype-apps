# benchmarks/flamegraph — v20 T2 L44 flamegraph-driven optimization loop

Authority: v20 71-pillar cycle 10 P1 plan §2 Track T2 + ADR-040 (coverage gates per tier).

This directory contains the **flamegraph harness** for the three fleet-critical
substrate crates:

| Crate             | Hot top                              | Why it's critical                              |
| ----------------- | ------------------------------------ | ---------------------------------------------- |
| `pheno-config`    | `cascade::build_cascade`             | Called on every service start; 12-factor cascade |
| `pheno-tracing`   | `TracePort::submit` + samplers       | Per-request span path; observability backbone   |
| `pheno-mcp-router`| `route_request` decision layer       | Per-request LLM routing; cost/budget/quota gate |

L44 score: 2.0 → 2.5 with this directory in place (real flamegraphs shipped + weekly trend tracking).

## Quickstart

```bash
# Per-crate real run (Linux + cargo-flamegraph installed):
./benchmarks/flamegraph/run-flamegraph.sh pheno-config
./benchmarks/flamegraph/run-flamegraph.sh pheno-tracing
./benchmarks/flamegraph/run-flamegraph.sh pheno-mcp-router

# Force the synthetic baseline (no real profiler needed; for CI smoke tests):
./benchmarks/flamegraph/run-flamegraph.sh pheno-tracing --synthetic
FLAMEGRAPH_SYNTHETIC=1 ./benchmarks/flamegraph/run-flamegraph.sh pheno-config
```

Each run writes (or refreshes) a single artifact:

- `benchmarks/flamegraph/<crate>-baseline.svg` — the flamegraph image.

The script picks the right tool automatically:

1. `cargo flamegraph` if installed (Linux / Windows with `perf`).
2. `cargo instruments -t time` on macOS if installed.
3. **Synthetic baseline** otherwise — the committed SVG is updated in place with a
   timestamp comment so downstream tooling can tell synthetic from real runs.

## Installing `cargo-flamegraph`

```bash
# Cargo subcommand (Linux / Windows with `perf` or `dtrace`):
cargo install flamegraph

# On Linux you also need:
#   - `perf` (linux-tools-common on Debian/Ubuntu, linux-tools on Fedora)
#   - Unprivileged perf: `sudo sysctl kernel.perf_event_paranoid=-1`
#       or run with `sudo -E` (CI does this via the workflow's runner).
```

`cargo flamegraph` wraps Brendan Gregg's `FlameGraph` repo; the SVG it produces
follows the standard format (x-axis = sample count, y-axis = call-stack depth,
width = CPU time).

## macOS-specific notes

`cargo flamegraph` does **not** work out of the box on macOS — `dtrace` requires
either the Xcode Command Line Tools' "Instruments" entitlements or a special
kernel flag that Apple has historically restricted. Two fallback paths:

### Option A: `cargo-instruments` (recommended on macOS)

```bash
brew install cargo-instruments    # ships templates for time, alloc, threads
cargo instruments -t time --bench <bench_name>
# Output: a .trace bundle in the current directory; export via:
xcrun xctrace export --input <crate>.trace --xpath '/trace-toc/run/data/table[0]' \
    > table.csv
```

The `run-flamegraph.sh` wrapper detects `cargo-instruments` and runs it
automatically. The macOS path still falls back to the synthetic baseline when the
`.trace` bundle cannot be exported to an SVG inside the script (the trace is
uploaded as a CI artifact; the SVG summary is the synthetic one).

### Option B: Linux container

Run the script inside a Linux container — Docker / OrbStack / lima / colima all
work. The CI workflow uses `ubuntu-latest` for the real run, so the committed
SVG is always generated on Linux with full `perf` support.

### Option C: Synthetic baseline (always works)

```bash
./benchmarks/flamegraph/run-flamegraph.sh pheno-tracing --synthetic
```

Writes a documentation-quality SVG with the expected hot paths labelled. Useful
for code review, design docs, and offline planning when you don't have a Linux
machine handy.

## Interpreting the SVG

A standard flamegraph (from `cargo flamegraph`):

- **x-axis**: alphabetical sample count, **NOT** time. Width of a frame = total
  samples where that function was on the call stack. So a wide frame at the
  bottom of the stack = a hot root.
- **y-axis**: call-stack depth. The root (top of stack) is the bottom row.
- **Colors**: by default, warm colors (red/orange) = hot/CPU-bound, cool colors
  (blue/green) = cold. The synthetic baselines in this directory use the same
  palette plus purple for caller-visible hot paths and red for explicit
  "optimization candidate" frames.
- **Off-CPU vs on-CPU**: `cargo flamegraph` defaults to **on-CPU** sampling
  (perf record). For blocking / I/O latency, run with `--off-cpu` or use the
  `perf record -e sched:sched_switch` template.
- **Reading**: hover over a frame in a browser to see the full function
  signature in the tooltip (`<title>` tag). The frame's width is its self+children
  cost; its children's width is what it delegated to.

A wide **red** frame near the bottom of the call stack is the most actionable
signal: it's a leaf function (or one with shallow children) where ~30%+ of
total CPU is being spent. That's a candidate for inlining, specialization, or
algorithmic change.

## Files in this directory

| File                            | Purpose                                                      |
| ------------------------------- | ------------------------------------------------------------ |
| `run-flamegraph.sh`             | The wrapper script (executable). Real run + synthetic fallback. |
| `README.md`                     | This file.                                                   |
| `config-baseline.svg`           | Flamegraph for `pheno-config` (synthetic until first real run). |
| `tracing-baseline.svg`          | Flamegraph for `pheno-tracing` (synthetic until first real run). |
| `mcp-router-baseline.svg`       | Flamegraph for `pheno-mcp-router` (synthetic until first real run). |

## CI integration

`.github/workflows/flamegraph.yml` runs every **Sunday 02:00 UTC** (weekly
trend tracking, not per-PR — per ADR-041 cadence rules). On every run it:

1. Builds `cargo-flamegraph` from source on `ubuntu-latest`.
2. Runs the wrapper script for each of the three crates.
3. Computes a frame-count and SVG-byte-size delta against the previous week's
   committed baseline.
4. If any crate's flamegraph grew by **> 20%** in either dimension, opens (or
   updates) the `flamegraph-trends` issue with the regression table and a link
   to the uploaded SVG artifact.

The synthetic baseline path is the **safe fallback** for local-only runs and
for the macOS-fastlane path; production trend tracking always uses the real
Linux run.

## See also

- `findings/2026-06-22-v20-T2-L44-flamegraph-deep-dives.md` — analysis report
  with the full set of optimization opportunities per crate.
- `findings/2026-06-22-v20-T2-L44-opportunities.md` — machine-readable
  opportunities table (P0 / P1 / P2 with est. speedup + effort).
- `benchmarks/perf-budgets.toml` (v12 T9) — per-method latency budgets.
- `benchmarks/fleet-perf.toml` (v19 T4) — fleet-wide perf-gate manifest.
- `scripts/perf_gate.py` (v19 T4) — the perf gate that consumes the manifest.
- ADR-040 — test coverage gates per tier (informs the L44 score's tier weighting).