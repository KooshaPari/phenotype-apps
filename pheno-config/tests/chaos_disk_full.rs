//! v20-T3 (L36) — chaos adoption test for `pheno-config`.
//!
//! Simulates a "disk full" / `ENOSPC`-style error in the
//! config-loader path. The test uses
//! `chaos_injection::inject` to inject a `DiskFull` fault and
//! asserts that the runtime surfaces it as an
//! `Err(FaultOutcome::DiskFull)`. It also verifies that the
//! config cascade (the production
//! `pheno_config::cascade::build_cascade_from_str` function) is
//! unaffected by the chaos runtime — the chaos framework is
//! observation-only, not a state-mutating dependency of the
//! loader.
//!
//! Run with:
//!
//! ```bash
//! cargo test --test chaos_disk_full
//! ```
//! FR: L36 — pheno-config must accept chaos-injection as a
//! dev-dependency and surface a `DiskFull` outcome through the
//! canonical `chaos_injection::inject` entry point.

use chaos_injection::{inject, Fault, FaultOutcome};
use pheno_config::cascade::build_cascade_from_str;

/// 1. Inject a `DiskFull` fault with `free_bytes = 0` and
/// assert the runtime surfaces it as
/// `Err(FaultOutcome::DiskFull)`. This is the canonical
/// "the chaos framework works for ENOSPC scenarios" smoke test.
#[test]
fn inject_disk_full_zero_bytes_surfaces_outcome() {
    let result = inject(Fault::DiskFull { free_bytes: 0 });
    assert!(
        result.is_err(),
        "DiskFull with free_bytes=0 must surface as Err"
    );
    assert_eq!(
        result.unwrap_err(),
        FaultOutcome::DiskFull,
        "the surfaced outcome must be exactly DiskFull"
    );
}

/// 2. DiskFull with `free_bytes >= 1024` is **absorbed** by
/// the runtime (treated as "filesystem not yet full"). The
/// config loader, when called outside a chaos scenario, should
/// still find the default values.
#[test]
fn disk_full_with_free_bytes_above_threshold_is_absorbed() {
    let chaos = inject(Fault::DiskFull {
        free_bytes: 4096,
    });
    assert!(
        chaos.is_ok(),
        "DiskFull with free_bytes=4096 (>=1024 threshold) must be absorbed"
    );

    // The config cascade must still produce the default
    // `server.port = 8080` from the embedded default TOML.
    let cfg = build_cascade_from_str("");
    let port: u16 = cfg
        .find_value("server.port")
        .expect("default server.port must be present");
    assert_eq!(port, 8080, "config cascade must still surface defaults");
}

/// 3. A DiskFull injection must not corrupt the cascade's
/// resolution of a TOML override. The cascade path operates
/// outside the chaos runtime.
#[test]
fn config_cascade_toml_override_survives_disk_full_chaos() {
    // Inject a DiskFull fault.
    let chaos = inject(Fault::DiskFull { free_bytes: 0 });
    assert!(chaos.is_err(), "DiskFull must surface");

    // Then exercise the production cascade with a TOML
    // override. The override must still win over the default.
    let toml_override = r#"
[server]
port = 9090
"#;
    let cfg = build_cascade_from_str(toml_override);
    let port: u16 = cfg
        .find_value("server.port")
        .expect("overridden server.port must be present");
    assert_eq!(
        port, 9090,
        "TOML override must win even after a DiskFull chaos injection"
    );
}

/// 4. The `pheno_config::cascade` module is the only public
/// API of the config crate that interacts with the filesystem
/// (via the optional `config.toml` read in `build_cascade`).
/// The test asserts that calling
/// `build_cascade_from_str` (the filesystem-free path) under a
/// DiskFull scenario is still a pure, deterministic function of
/// its input string.
#[test]
fn build_cascade_from_str_is_pure_under_disk_full_chaos() {
    // Inject DiskFull first.
    let _ = inject(Fault::DiskFull { free_bytes: 0 });

    // Call the production cascade twice with the same input;
    // both calls must return identical port values.
    let cfg1 = build_cascade_from_str("[server]\nport = 7070\n");
    let cfg2 = build_cascade_from_str("[server]\nport = 7070\n");
    let p1: u16 = cfg1.find_value("server.port").unwrap();
    let p2: u16 = cfg2.find_value("server.port").unwrap();
    assert_eq!(
        p1, 7070,
        "first call must return the overridden port value"
    );
    assert_eq!(p1, p2, "cascade must be pure / deterministic");
}
