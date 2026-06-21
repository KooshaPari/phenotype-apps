# Performance flamegraph reporting (L44)

**Pillar:** L44 (Performance — flamegraph-driven deep-dives)
**Cycle:** v20 T2 (71-pillar cycle 10, P1 reduction)
**Authority:** ADR-040 (test coverage gates per tier) + v20 plan §3 Track T2

This document explains how to use the on-demand `flamegraph` workflow to investigate
performance regressions or hotspots in fleet crates.

## When to use a flamegraph

`perf-gate.yml` (v19 T4) catches **budget regressions** — a nightly + on-PR check that
each crate's hot method stays under its declared p95 budget. When the gate fails, or when
you're investigating a suspected hotspot, you need to see **where** the time goes inside
the binary. That's what a flamegraph is for.

Use a flamegraph when:

- A specific PR is suspected of regressing CPU usage or p95 latency.
- You want to find the top 5–10 functions to focus optimization effort.
- You're reviewing a sub-`config.toml` change and want to know the call paths it affects.

Do **not** use a flamegraph for:

- Routine PRs with no perf signal (perf-gate covers this).
- I/O-bound or syscall-bound workloads — use `perf` / `bpftrace` / `strace` instead.
- Allocations — use `heaptrack` / `dhat` (see "Future work" below).

## How to request a flamegraph on a PR

1. Add the `perf-investigate` label to the PR.
2. The `Fleet flamegraph (on-demand)` workflow (`.github/workflows/flamegraph.yml`) runs.
3. Once complete, a comment is posted on the PR with a summary table of the top 15
   visible symbols (from `addr2line`). The full SVG is attached as a workflow artifact.
4. Download the SVG (`flamegraph-<PR#>-<run-id>`), open it in any modern browser.
5. The first run may take ~5–8 min (cargo build of the target binary + `cargo install
   flamegraph`). Subsequent runs benefit from `Swatinem/rust-cache` and complete in
   ~1–2 min for unchanged crates.

## What the SVG shows

The standard `cargo flamegraph` output is an inverted-icicle SVG. Each box is one
function; the **width** of the box is proportional to the time spent in that function
(including its callees). Click a box to zoom in. Hover for `addr2line`-resolved
function name and file:line.

Reading tips:

- **Wide boxes at the top** = the actual hot paths. Optimize these first.
- **Tall narrow boxes** = leaf functions called from many places (e.g. `alloc::raw_vec`).
  Don't optimize these in isolation — find the caller that dominates.
- **`<inline>` / `<unknown>`** = debug info is missing; the binary was built without
  `debuginfo = true` or with stripped symbols. Fix `Cargo.toml` first.

## Interpreting results for our fleet

The default target is `perf-gate` (the v19 T4 binary). For per-crate deep-dives, set the
`INPUT_TARGET` and `INPUT_KIND` GitHub Actions variables (these are reserved for a
follow-up workflow_dispatch input block; for now, edit `flamegraph.yml` directly or
duplicate the workflow per crate).

Common hot paths we expect to see:

- `pheno-config`: TOML parse + serde deserialization. Look for `serde_json::de` and
  `toml::de`.
- `pheno-tracing`: OTLP export batching. Look for `tonic::transport` and `protobuf`.
- `pheno-mcp-router`: adapter dispatch + JSON-RPC framing. Look for `serde_json` and
  `tokio::sync::mpsc`.

## Local reproduction

```bash
# Install once.
cargo install flamegraph

# Run on a release build of the perf-gate binary.
scripts/flamegraph.sh --bin perf-gate --release
# Output: target/flamegraph.svg
```

On Linux you may need `--root` (or `kernel.perf_event_paranoid <= -1`). The wrapper
script forwards `--root` automatically when run as root.

On macOS, `cargo flamegraph` uses `dtrace`; you'll be prompted for a password and SIP
must be disabled. Prefer a Linux runner for reproducible CI runs.

## Cost (CI minutes)

Per run on `ubuntu-latest`:

| Phase | First run (cold cache) | Subsequent (warm cache) |
|-------|------------------------|--------------------------|
| `cargo install flamegraph` | ~90 s | ~10 s (toolchain cached) |
| Build target binary (release) | ~240 s | ~30 s |
| `cargo flamegraph` run (60 s sample) | ~75 s | ~75 s |
| **Total** | **~7 min** | **~2 min** |

GitHub-hosted `ubuntu-latest` runners are billed at 1× minute rate; a monthly cadence
of ~10 investigations × 5 min avg = **~50 min/month** of billable CI minutes.

## Future work (not in v20 T2)

- **`dhat`/`heaptrack` integration** for allocation flamegraphs (out of scope for L44
  this cycle; could land in v21 under L45).
- **Diff-flamegraph** (`flamegraph-diff`) for before/after PRs (L44 deepening to 3.0).
- **Per-crate workflow dispatch inputs** to avoid editing `flamegraph.yml` per
  investigation.
- **Inferno (`cargo-inferno`) fallback** for environments where `perf` is unavailable
  (e.g. container runners without `CAP_SYS_ADMIN`).

## References

- `cargo-flamegraph` — <https://github.com/flamegraph-rs/flamegraph>
- `perf-gate.yml` — `.github/workflows/perf-gate.yml` (v19 T4 L19 baseline)
- `benchmarks/fleet-perf.toml` — declarative perf budgets
- ADR-040 — test coverage gates per tier
- v20 plan — `plans/2026-06-22-v20-71-pillar-cycle-10-p1.md` §3 T2
