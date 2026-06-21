//! # pheno-config
//!
//! Canonical typed-config loader for the `pheno-*` fleet.
//!
//! Two modules:
//!
//! 1. [`cascade`] — layered provider cascade (JetBrains run-configs
//!    > `PHENO_*` env vars > checked-in `config.toml` > embedded
//!    defaults). 12-factor; CI-friendly; developer-machine overrides
//!    win without touching env vars or TOML.
//! 2. [`secrets`] — secret-holding newtypes (`ApiKey`, `BearerToken`,
//!    `DbPassword`) with `ZeroizeOnDrop`. Per ADR-078 (L52, v19 T2),
//!    every secret-bearing value in `pheno-config` flows through
//!    these newtypes so the inner bytes are wiped on `Drop`.
//!
//! ## ADR-078 compliance
//!
//! `pheno-config` is in the secret-handling module surface. Per
//! ADR-078 §2.1, the crate root carries `#![forbid(unsafe_code)]` so
//! `unsafe` cannot be introduced without an explicit `allow(unsafe)`
//! and a corresponding ADR amendment.
//!
//! See:
//! - `docs/adr/2026-06-21/ADR-078-encryption-at-rest-mandate.md`
//! - `.cargo/audit-rules.toml` (cargo-deny config)

#![forbid(unsafe_code)]
#![warn(missing_docs)]

/// Layered config-provider cascade. See module docs for the priority
/// order and the rationale behind it.
pub mod cascade;

/// Secret-holding newtypes with `ZeroizeOnDrop`. See module docs for
/// the redaction policy and the `From`-based construction pattern.
pub mod secrets;
