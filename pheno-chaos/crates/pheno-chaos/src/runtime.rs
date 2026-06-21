//! Internal runtime used by the [`chaos_test`](crate::chaos_test)
//! proc-macro to drive a fault schedule.
//!
//! The proc-macro generates a `#[test]` function that calls
//! [`run_with_chaos`] with the configured fault list, SLO, and run
//! count. This module is the bridge between the macro and the fault
//! implementations.

use std::panic::{catch_unwind, resume_unwind, AssertUnwindSafe};
use std::time::{Duration, Instant};

use crate::fault::Fault;

/// Configuration produced by parsing the `#[chaos_test(...)]`
/// attribute. The proc-macro constructs one of these per test.
#[derive(Debug, Clone)]
pub struct ChaosConfig {
    /// Faults to randomly inject per run.
    pub faults: Vec<FaultKind>,
    /// Per-run SLO in milliseconds. The body must complete within
    /// this budget for the run to be considered a pass.
    pub slo_ms: u64,
    /// Number of runs (each picks a fresh random fault).
    pub runs: u32,
    /// Optional RNG seed for reproducible chaos. Default 0 = use
    /// clock entropy.
    pub seed: u64,
}

/// Identifier for the three fault kinds. The runtime stores this and
/// constructs the actual `Fault` instance per run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FaultKind {
    /// [`crate::NetworkLatency`] — application-layer sleep-based delay.
    Latency,
    /// [`crate::ConnectionDrop`] — explicit connection-reset simulation.
    Drop,
    /// [`crate::CpuSpike`] — dedicated spinning thread.
    Cpu,
}

impl std::fmt::Display for FaultKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl FaultKind {
    /// Construct the default `Fault` for this kind.
    pub fn build(self) -> Box<dyn Fault> {
        match self {
            FaultKind::Latency => Box::new(crate::NetworkLatency::default()),
            FaultKind::Drop => Box::new(crate::ConnectionDrop::default()),
            FaultKind::Cpu => Box::new(crate::CpuSpike::default()),
        }
    }

    /// String form used in attribute parsing.
    pub fn parse(s: &str) -> Option<Self> {
        match s.trim() {
            "latency" | "network" => Some(FaultKind::Latency),
            "drop" | "connection" => Some(FaultKind::Drop),
            "cpu" | "cpu_spike" => Some(FaultKind::Cpu),
            _ => None,
        }
    }

    /// Stable, human-readable name (mirrors [`crate::fault::Fault::name`]).
    pub fn name(self) -> &'static str {
        match self {
            FaultKind::Latency => "network_latency",
            FaultKind::Drop => "connection_drop",
            FaultKind::Cpu => "cpu_spike",
        }
    }

    /// All three kinds, in canonical order.
    pub fn all() -> &'static [FaultKind] {
        &[FaultKind::Latency, FaultKind::Drop, FaultKind::Cpu]
    }
}

/// Run a test body under the configured chaos schedule.
///
/// Pseudo-code:
///
/// ```text
/// for run in 0..runs:
///     fault = pick_random(faults, seed=seed+run)
///     guard = fault.inject()
///     start = Instant::now()
///     result = catch_unwind(body)
///     elapsed = start.elapsed()
///     assert elapsed < slo, "SLO breach on run {run} ({fault})"
///     if result.is_err(): resume_unwind
///     guard.revert()
/// ```
///
/// The body is `AssertUnwindSafe`-wrapped so panic-catching does not
/// flag `UnwindSafe` violations for harmless interior mutability
/// (e.g. counters). The body must be callable repeatedly (i.e.
/// implement `Fn`) so the chaos loop can run it `runs` times.
pub fn run_with_chaos<F>(cfg: &ChaosConfig, body: F)
where
    F: Fn(),
{
    let slo = Duration::from_millis(cfg.slo_ms);
    let mut rng = SimpleRng::new(if cfg.seed == 0 {
        // Derive a non-zero seed from the clock.
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0xDEADBEEF)
    } else {
        cfg.seed
    });

    let faults = if cfg.faults.is_empty() {
        FaultKind::all().to_vec()
    } else {
        cfg.faults.clone()
    };

    for run in 0..cfg.runs {
        let kind = faults[(rng.next_u32() as usize) % faults.len()];
        let fault = kind.build();

        let guard = fault
            .inject()
            .unwrap_or_else(|e| panic!("fault `{}` failed to inject: {}", kind, e));

        let start = Instant::now();
        let result = catch_unwind(AssertUnwindSafe(|| {
            body();
        }));
        let elapsed = start.elapsed();

        if let Err(payload) = result {
            // Body panicked — drop guard (revert) then propagate.
            drop(guard);
            resume_unwind(payload);
        }

        assert!(
            elapsed <= slo,
            "chaos_test SLO breach on run {}/{} (fault={}, elapsed={:?}, slo={:?})",
            run + 1,
            cfg.runs,
            kind,
            elapsed,
            slo
        );

        guard.revert();
    }
}

/// Minimal splitmix64-style PRNG. We avoid pulling `rand` to keep the
/// runtime surface to std + libc (per V20-T3 brief).
struct SimpleRng(u64);

impl SimpleRng {
    fn new(seed: u64) -> Self {
        Self(seed.max(1))
    }
    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }
}

/// Parse a `key = value` attribute list (e.g. `"faults = \"latency,
/// drop, cpu\", slo_ms = 500, runs = 3"`) into a [`ChaosConfig`].
///
/// This is a deliberately small parser — it handles the well-known
/// keys `faults`, `slo_ms`, `runs`, `seed`. Unknown keys are ignored
/// with a warning printed to stderr so a typo doesn't silently
/// disable the test.
pub fn parse_attr_args(s: &str) -> ChaosConfig {
    let mut cfg = ChaosConfig {
        faults: FaultKind::all().to_vec(),
        slo_ms: 500,
        runs: 3,
        seed: 0,
    };

    // Tokenise on commas at the top level. Simple state machine: we
    // don't support nested arrays or escaped quotes beyond the common
    // case.
    let mut depth = 0i32;
    let mut current = String::new();
    let mut parts: Vec<String> = Vec::new();
    for ch in s.chars() {
        match ch {
            '(' | '[' | '{' => {
                depth += 1;
                current.push(ch);
            }
            ')' | ']' | '}' => {
                depth -= 1;
                current.push(ch);
            }
            ',' if depth == 0 => {
                parts.push(current.trim().to_string());
                current.clear();
            }
            _ => current.push(ch),
        }
    }
    if !current.trim().is_empty() {
        parts.push(current.trim().to_string());
    }

    for part in parts {
        let Some((k, v)) = part.split_once('=') else {
            eprintln!("[pheno-chaos] ignoring malformed attr: `{}`", part);
            continue;
        };
        let key = k.trim();
        let value = v.trim();

        match key {
            "faults" => {
                // Strip surrounding quotes if present.
                let value = value.trim_matches('"');
                cfg.faults = value
                    .split(|c: char| c == ',' || c.is_whitespace())
                    .filter_map(|tok| FaultKind::parse(tok))
                    .collect();
                if cfg.faults.is_empty() {
                    eprintln!(
                        "[pheno-chaos] `faults` parsed empty; falling back to all faults"
                    );
                    cfg.faults = FaultKind::all().to_vec();
                }
            }
            "slo_ms" => {
                cfg.slo_ms = value.parse().unwrap_or_else(|_| {
                    eprintln!(
                        "[pheno-chaos] slo_ms=`{}` not an integer; defaulting to 500",
                        value
                    );
                    500
                });
            }
            "runs" => {
                cfg.runs = value.parse().unwrap_or_else(|_| {
                    eprintln!(
                        "[pheno-chaos] runs=`{}` not an integer; defaulting to 3",
                        value
                    );
                    3
                });
                cfg.runs = cfg.runs.max(1);
            }
            "seed" => {
                cfg.seed = value.parse().unwrap_or(0);
            }
            other => {
                eprintln!("[pheno-chaos] unknown attr key `{}`; ignoring", other);
            }
        }
    }

    cfg
}