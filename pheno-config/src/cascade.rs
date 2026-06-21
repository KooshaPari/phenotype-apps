//! Layered configuration cascade for `pheno-config`.
//!
//! Cascades providers in strict priority order (highest wins):
//!
//! 1. **`Env::prefixed("PHENO_")`** — environment variables of the form
//!    `PHENO_<KEY>=value`. 12-factor style; CI / container runtime overrides.
//! 2. **`Toml::file("config.toml")`** — checked-in TOML defaults (loaded only
//!    if the file exists; missing file is non-fatal).
//! 3. **`Toml::string(DEFAULT_TOML)`** — embedded compile-time defaults
//!    (lowest priority; always present).
//!
//! NOTE: A previous version of this cascade also layered a `Jetbrains`
//! provider (`.idea/runConfigurations/*.xml` developer-machine overrides).
//! That provider was removed from `figment` upstream (post-0.10.10), so the
//! top-of-stack IntelliJ override is no longer available. If a developer
//! needs to shadow a checked-in value, use the `PHENO_*` env var layer.
//!
//! The cascade order in [`build_cascade`] is **bottom-of-stack first**:
//! [`Figment::merge`] appends a provider on top of the previous, so the last
//! `merge` call has the highest priority.  Therefore the call order is:
//!
//! `defaults → toml-file → env`
//!
//! which produces the desired priority `env > toml > default`.

use figment::{
    providers::{Env, Format, Toml},
    Figment,
};

/// Embedded default TOML payload — always available, lowest priority.
///
/// Values here are the `pheno-config` defaults that the rest of the Phenotype
/// fleet can rely on without reading any file or env var.
pub const DEFAULT_TOML: &str = r#"
# pheno-config defaults (lowest priority in cascade)
[server]
host = "127.0.0.1"
port = 8080

[logging]
level = "info"
format = "json"

[database]
path = "./pheno.db"
pool_size = 8
"#;

/// Build the standard `pheno-config` cascade.
///
/// Provider order (bottom-of-stack → top-of-stack, top wins):
///
/// 1. [`Toml::string(DEFAULT_TOML)`] — embedded defaults.
/// 2. [`Toml::file("config.toml")`] — checked-in TOML (if present).
/// 3. [`Env::prefixed("PHENO_")`] — environment variables.
pub fn build_cascade() -> Figment {
    Figment::new()
        // 1. Embedded defaults (lowest priority).
        .merge(Toml::string(DEFAULT_TOML))
        // 2. Checked-in TOML — fails soft if `config.toml` is missing.
        .merge(Toml::file("config.toml"))
        // 3. Environment variables (`PHENO_*` prefix).
        .merge(Env::prefixed("PHENO_"))
}

/// Build the cascade from an explicit TOML string instead of `config.toml`.
///
/// Useful for tests and for callers that want to inject TOML from a custom
/// source (e.g. a bundled config blob inside a binary).
pub fn build_cascade_from_str(toml: &str) -> Figment {
    Figment::new()
        .merge(Toml::string(DEFAULT_TOML))
        .merge(Toml::string(toml))
        .merge(Env::prefixed("PHENO_"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_toml_parses_as_valid_toml() {
        // The embedded defaults must always be valid TOML — a syntax error
        // here is a compile-time-equivalent bug.
        let cascade = build_cascade();
        // `figment::find_value` returns `figment::value::Value`; we
        // extract the inner `u64` via `to_u64()` (the typed
        // `find_value::<u16>` shortcut was not yet stable in 0.10).
        let value = cascade
            .find_value("server.port")
            .expect("default server.port must be present");
        assert_eq!(value.to_u128(), Some(8080));
    }
}