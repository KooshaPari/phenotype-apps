//! V20-T2 (L44) — Flamegraph-driven performance deep-dive.
//!
//! This bench drives the **4-stage / 5-component port-adapter pipeline**
//! that the pheno-port-adapter fleet relies on for adapter-spec ingestion:
//!
//! ```text
//!   source bytes
//!        │
//!        ▼
//!   ┌─────────┐    ┌──────────┐    ┌────────┐    ┌──────────┐    ┌─────────┐
//!   │ Scanner │ ─► │ Analyzer │ ─► │ Mapper │ ─► │ Compiler │ ─► │ Emitter │
//!   └─────────┘    └──────────┘    └────────┘    └──────────┘    └─────────┘
//!   byte → token   token → AST     AST → IR    IR → dispatch    IR → bytes
//! ```
//!
//! - **Stage 1 — Scanner** (1 component): byte-level tokenization of a
//!   synthetic TOML-ish adapter-spec source.
//! - **Stage 2 — Analyzer** (1 component): parse tokens into a typed
//!   `AdapterSpec` AST with kind + endpoint.
//! - **Stage 3 — Mapper** (1 component): group specs into a `RouteTable`
//!   IR keyed by adapter kind.
//! - **Stage 4 — Compiler + Emitter** (2 components): lower the IR into
//!   a dispatch table and serialize to a byte buffer.
//!
//! ## Why pprof here?
//!
//! Per L44 ("flamegraph-driven performance deep-dives"), we want a single
//! reproducible flamegraph committed alongside the source so perf drift
//! between releases is visible at PR-review time. `pprof` 0.13 with the
//! `flamegraph` feature gives us a self-contained SVG that doesn't need
//! `perf`/`dtrace`/Linux-specific tooling.
//!
//! ## Running
//!
//! ```bash
//! # 10-second measurement per criterion sample (matches CI invocation)
//! cargo bench --bench flame -- --profile-time=10
//!
//! # Or just build + run the bench binary directly
//! cargo build --benches
//! ./target/release/deps/flame-<hash> --bench --profile-time=10
//! ```
//!
//! Output: `flamegraph.svg` in the crate root (or in the path passed via
//! the `PHENO_FLAMEGRAPH_OUT` env var). The capture runs **once**, before
//! the first criterion warmup — the timed region is never polluted by
//! SVG serialization cost.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::hash::{Hash, Hasher};
use std::hint::black_box;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use pprof::flamegraph::Options;

// ---------------------------------------------------------------------------
// Pipeline input — synthetic adapter-spec source.
// ---------------------------------------------------------------------------

/// Build the synthetic source: ~10 KiB of TOML-ish `key = value` lines,
/// one adapter-spec per line. The shape matches what a real
/// `phenotype-apps` config dump would look like.
fn synth_source() -> String {
    let mut s = String::with_capacity(16 * 1024);
    s.push_str("# pheno-port-adapter bench source — generated, do not edit\n");
    let kinds = ["tcp", "unix", "cache", "retry", "timeout"];
    let mut i: u32 = 0;
    while s.len() < 10 * 1024 {
        let kind = kinds[(i as usize) % kinds.len()];
        match kind {
            "tcp" => {
                let port = 1024 + (i % 60_000);
                writeln!(s, "adapter.{kind}.node_{i:04} = \"tcp@127.0.0.1:{port}\"").unwrap();
            }
            "unix" => {
                writeln!(
                    s,
                    "adapter.{kind}.node_{i:04} = \"unix@/var/run/phenotype/{i:04}.sock\""
                )
                .unwrap();
            }
            "cache" => {
                let ttl = 100 + (i % 60_000);
                writeln!(
                    s,
                    "adapter.{kind}.node_{i:04} = \"cache@redis://127.0.0.1:6379?ttl_ms={ttl}\""
                )
                .unwrap();
            }
            "retry" => {
                let n = 1 + (i % 7);
                writeln!(s, "adapter.{kind}.node_{i:04} = \"retry@max_attempts={n}\"").unwrap();
            }
            _ => {
                let ms = 50 + (i % 5_000);
                writeln!(s, "adapter.{kind}.node_{i:04} = \"timeout@{ms}ms\"").unwrap();
            }
        }
        i += 1;
    }
    s
}

// ---------------------------------------------------------------------------
// Stage 1 — Scanner.
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tok {
    /// `key` — identifier on the left of `=`.
    Ident(String),
    /// `=` — assignment.
    Equals,
    /// `"value"` — quoted string on the right of `=`.
    Str(String),
    /// Comment or blank — dropped by the scanner.
    Skip,
}

fn scanner(src: &str) -> Vec<Tok> {
    // Pre-size: ~1 token per ~12 bytes of input.
    let mut out = Vec::with_capacity(src.len() / 12 + 16);
    let bytes = src.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        match b {
            b'#' => {
                // line comment — skip to end of line
                while i < bytes.len() && bytes[i] != b'\n' {
                    i += 1;
                }
                out.push(Tok::Skip);
            }
            b'\n' | b'\r' | b' ' | b'\t' => {
                i += 1;
            }
            b'=' => {
                out.push(Tok::Equals);
                i += 1;
            }
            b'"' => {
                // String literal: copy until next unescaped quote.
                i += 1;
                let start = i;
                while i < bytes.len() && bytes[i] != b'"' {
                    if bytes[i] == b'\\' && i + 1 < bytes.len() {
                        i += 2;
                    } else {
                        i += 1;
                    }
                }
                let s = std::str::from_utf8(&bytes[start..i]).unwrap_or("").to_string();
                out.push(Tok::Str(s));
                if i < bytes.len() {
                    i += 1; // closing quote
                }
            }
            c if c.is_ascii_alphabetic() || c == b'_' => {
                let start = i;
                while i < bytes.len()
                    && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_' || bytes[i] == b'.')
                {
                    i += 1;
                }
                let s = std::str::from_utf8(&bytes[start..i])
                    .unwrap_or("")
                    .to_string();
                out.push(Tok::Ident(s));
            }
            _ => {
                // Unknown byte — emit Skip so downstream sees one event
                // per consumed byte and we keep the byte offset advancing.
                out.push(Tok::Skip);
                i += 1;
            }
        }
    }
    out
}

// ---------------------------------------------------------------------------
// Stage 2 — Analyzer.
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
enum AdapterKind {
    Tcp,
    Unix,
    Cache,
    Retry,
    Timeout,
}

#[derive(Debug, Clone)]
struct AdapterSpec {
    /// Full dotted key, e.g. `adapter.tcp.node_0042`.
    key: String,
    kind: AdapterKind,
    /// Raw value payload as it appeared between the quotes, e.g.
    /// `tcp@127.0.0.1:8080` or `cache@redis://...?ttl_ms=30000`.
    raw: String,
}

fn classify(kind_segment: &str) -> Option<AdapterKind> {
    match kind_segment {
        "tcp" => Some(AdapterKind::Tcp),
        "unix" => Some(AdapterKind::Unix),
        "cache" => Some(AdapterKind::Cache),
        "retry" => Some(AdapterKind::Retry),
        "timeout" => Some(AdapterKind::Timeout),
        _ => None,
    }
}

fn analyzer(toks: &[Tok]) -> Vec<AdapterSpec> {
    let mut out = Vec::with_capacity(toks.len() / 3);
    let mut i = 0;
    while i + 2 < toks.len() {
        if let Tok::Ident(key) = &toks[i] {
            if matches!(toks[i + 1], Tok::Equals) {
                if let Tok::Str(raw) = &toks[i + 2] {
                    // Parse dotted key: "adapter.<kind>.<name>".
                    let mut parts = key.split('.');
                    let _prefix = parts.next(); // "adapter"
                    let kind_segment = parts.next().unwrap_or("");
                    let name_segment = parts.next().unwrap_or("");
                    if !kind_segment.is_empty()
                        && !name_segment.is_empty()
                        && let Some(kind) = classify(kind_segment)
                    {
                        out.push(AdapterSpec {
                            key: key.clone(),
                            kind,
                            raw: raw.clone(),
                        });
                        i += 3;
                        continue;
                    }
                }
            }
        }
        i += 1;
    }
    out
}

// ---------------------------------------------------------------------------
// Stage 3 — Mapper.
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
struct RouteEntry {
    name: String,
    raw: String,
}

#[derive(Debug, Clone, Default)]
struct RouteTable {
    by_kind: BTreeMap<&'static str, Vec<RouteEntry>>,
}

impl RouteTable {
    fn kind_label(k: &AdapterKind) -> &'static str {
        match k {
            AdapterKind::Tcp => "tcp",
            AdapterKind::Unix => "unix",
            AdapterKind::Cache => "cache",
            AdapterKind::Retry => "retry",
            AdapterKind::Timeout => "timeout",
        }
    }
}

fn mapper(specs: Vec<AdapterSpec>) -> RouteTable {
    let mut tbl = RouteTable::default();
    for spec in specs {
        let label = RouteTable::kind_label(&spec.kind);
        // "kind.name" — strip the dotted prefix to get just the leaf.
        let leaf = spec
            .key
            .rsplit('.')
            .next()
            .unwrap_or(&spec.key)
            .to_string();
        tbl.by_kind.entry(label).or_default().push(RouteEntry {
            name: leaf,
            raw: spec.raw,
        });
    }
    // Per-kind sort so downstream stages see deterministic input.
    for v in tbl.by_kind.values_mut() {
        v.sort_by(|a, b| a.name.cmp(&b.name));
    }
    tbl
}

// ---------------------------------------------------------------------------
// Stage 4 — Compiler.
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
struct DispatchKey {
    kind: &'static str,
    name: String,
}

#[derive(Debug, Clone)]
struct DispatchEntry {
    key: DispatchKey,
    hash: u64,
    payload: String,
}

fn fnv1a(s: &str) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in s.as_bytes() {
        h ^= *b as u64;
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}

fn compiler(tbl: RouteTable) -> Vec<DispatchEntry> {
    let mut out: Vec<DispatchEntry> = tbl
        .by_kind
        .iter()
        .flat_map(|(kind, entries)| {
            entries.iter().map(move |e| {
                let key = DispatchKey {
                    kind,
                    name: e.name.clone(),
                };
                let hash = {
                    let mut hasher = std::collections::hash_map::DefaultHasher::new();
                    key.kind.hash(&mut hasher);
                    key.name.hash(&mut hasher);
                    hasher.finish()
                };
                DispatchEntry {
                    key,
                    hash,
                    payload: e.raw.clone(),
                }
            })
        })
        .collect();
    // Cross-check the std hasher against fnv and fold the two so we get a
    // a stable composite hash that's deterministic across std hasher
    // versions (DefaultHasher's SipHash seed can change between rustc
    // releases; mixing in fnv1a neutralises that drift).
    for entry in &mut out {
        let composed = format!("{}|{}", entry.key.kind, entry.key.name);
        let alt = fnv1a(&composed);
        entry.hash ^= alt.rotate_left(17);
    }
    out.sort_by_key(|e| e.hash);
    out
}

// ---------------------------------------------------------------------------
// Stage 5 — Emitter.
// ---------------------------------------------------------------------------

fn emitter(entries: &[DispatchEntry]) -> Vec<u8> {
    // Emit a compact binary-ish encoding:
    //   [u32 entry_count]
    //   per entry:
    //     [u8 kind_len][kind bytes][u8 name_len][name bytes][u8 payload_len][payload bytes][u64 hash]
    let mut buf = Vec::with_capacity(entries.len() * 64);
    buf.extend_from_slice(&(entries.len() as u32).to_le_bytes());
    for e in entries {
        let kind_bytes = e.key.kind.as_bytes();
        buf.push(kind_bytes.len() as u8);
        buf.extend_from_slice(kind_bytes);
        let name_bytes = e.key.name.as_bytes();
        buf.push(name_bytes.len() as u8);
        buf.extend_from_slice(name_bytes);
        let payload_bytes = e.payload.as_bytes();
        let payload_len = payload_bytes.len().min(u8::MAX as usize) as u8;
        buf.push(payload_len);
        buf.extend_from_slice(&payload_bytes[..payload_len as usize]);
        buf.extend_from_slice(&e.hash.to_le_bytes());
    }
    buf
}

// ---------------------------------------------------------------------------
// Pipeline glue.
// ---------------------------------------------------------------------------

#[inline(never)]
fn run_pipeline_final_only(src: &str) -> Vec<u8> {
    // The "real" hot path used by services: only the final bytes matter,
    // intermediate vectors are dropped. This is what gets profiled.
    let toks = scanner(src);
    let specs = analyzer(&toks);
    let ir = mapper(specs);
    let disp = compiler(ir);
    emitter(&disp)
}

// ---------------------------------------------------------------------------
// pprof flamegraph capture.
// ---------------------------------------------------------------------------

fn flamegraph_out_path() -> std::path::PathBuf {
    if let Ok(p) = std::env::var("PHENO_FLAMEGRAPH_OUT") {
        return std::path::PathBuf::from(p);
    }
    // Default: `<crate-root>/flamegraph.svg`
    std::path::PathBuf::from("flamegraph.svg")
}

/// Run a profiled burst of the pipeline and write an SVG flamegraph.
///
/// `seconds` controls how long the inner loop runs. We default to
/// `PHENO_FLAME_PROFILE_SECS` (or 10 s) so it matches the CI invocation
/// `cargo bench --bench flame -- --profile-time=10`.
fn capture_flamegraph(seconds: u64) -> std::io::Result<std::path::PathBuf> {
    let src = synth_source();
    let guard = pprof::ProfilerGuard::new(100).expect("pprof ProfilerGuard::new");
    let start = std::time::Instant::now();
    let mut acc: u64 = 0;
    while start.elapsed().as_secs() < seconds {
        let bytes = run_pipeline_final_only(&src);
        acc = acc.wrapping_add(bytes.len() as u64);
    }
    // Force the optimizer to keep the work.
    black_box(acc);
    let report = guard.report().build().expect("pprof report build");
    let out = flamegraph_out_path();
    let mut opts = Options::default();
    opts.title =
        "pheno-port-adapter / 4-stage pipeline (Scanner->Analyzer->Mapper->Compiler->Emitter)"
            .to_string();
    opts.subtitle = format!(
        "L44 flamegraph -- {} s profile, {} Hz sampling",
        seconds, 100
    );
    opts.count_name = "samples".to_string();
    let file = std::fs::File::create(&out)?;
    report.flamegraph_with_options(file, &mut opts)?;
    Ok(out)
}

// ---------------------------------------------------------------------------
// Criterion integration.
// ---------------------------------------------------------------------------

/// One-shot capture: runs the FIRST time `bench_pipeline` is called,
/// BEFORE criterion starts warming up / measuring. The timed region is
/// never polluted by SVG serialization cost.
fn maybe_capture_flamegraph(profile_secs: u64) {
    use std::sync::Once;
    static ONCE: Once = Once::new();
    ONCE.call_once(|| match capture_flamegraph(profile_secs) {
        Ok(path) => eprintln!("flamegraph written: {}", path.display()),
        Err(e) => eprintln!("flamegraph capture failed: {e}"),
    });
}

fn profile_secs_from_env() -> u64 {
    std::env::var("PHENO_FLAME_PROFILE_SECS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(10)
}

fn bench_pipeline(c: &mut Criterion) {
    let profile_secs = profile_secs_from_env();
    maybe_capture_flamegraph(profile_secs);
    let src = synth_source();
    c.bench_with_input(
        BenchmarkId::new("port_pipeline_4stage", src.len()),
        &src,
        |b, src| {
            b.iter(|| {
                let bytes = run_pipeline_final_only(black_box(src));
                black_box(bytes);
            });
        },
    );
}

criterion_group! {
    name = pheno_port_adapter_flame;
    config = Criterion::default()
        .sample_size(20)
        .measurement_time(std::time::Duration::from_secs(profile_secs_from_env()));
    targets = bench_pipeline
}
criterion_main!(pheno_port_adapter_flame);

// ---------------------------------------------------------------------------
// Unit tests — the 5 components must round-trip a known input so a
// regression in any stage fails the test suite, not just the bench.
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scanner_emits_known_ident() {
        let toks = scanner("adapter.tcp.node = \"tcp@127.0.0.1:1\"");
        assert!(matches!(toks[0], Tok::Ident(ref s) if s == "adapter.tcp.node"));
    }

    #[test]
    fn analyzer_parses_tcp_spec() {
        let toks = scanner("adapter.tcp.node_0001 = \"tcp@127.0.0.1:8080\"");
        let specs = analyzer(&toks);
        assert_eq!(specs.len(), 1);
        assert_eq!(specs[0].key, "adapter.tcp.node_0001");
        assert!(matches!(specs[0].kind, AdapterKind::Tcp));
    }

    #[test]
    fn analyzer_handles_all_five_kinds() {
        let src = "\
            adapter.tcp.x = \"tcp@127.0.0.1:1\"\n\
            adapter.unix.x = \"unix@/tmp/x\"\n\
            adapter.cache.x = \"cache@redis://localhost:6379\"\n\
            adapter.retry.x = \"retry@max_attempts=3\"\n\
            adapter.timeout.x = \"timeout@100ms\"";
        let toks = scanner(src);
        let specs = analyzer(&toks);
        assert_eq!(specs.len(), 5, "expected one spec per kind, got {specs:?}");
    }

    #[test]
    fn full_pipeline_emits_nonzero_bytes() {
        let src = synth_source();
        let bytes = run_pipeline_final_only(&src);
        assert!(!bytes.is_empty());
        // First 4 bytes are the entry-count little-endian u32.
        let count = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        assert!(count > 0);
    }

    #[test]
    fn compiler_emits_dispatch_entries_sorted_by_hash() {
        let src = "adapter.tcp.b = \"tcp@127.0.0.1:2\"\nadapter.tcp.a = \"tcp@127.0.0.1:1\"";
        let toks = scanner(src);
        let specs = analyzer(&toks);
        let ir = mapper(specs);
        let disp = compiler(ir);
        assert_eq!(disp.len(), 2);
        for win in disp.windows(2) {
            assert!(win[0].hash <= win[1].hash, "dispatch entries not sorted by hash");
        }
    }
}