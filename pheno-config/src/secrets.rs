// Reference: ADR-078 (docs/adr/2026-06-21/ADR-078-encryption-at-rest-mandate.md)
// Plan:       plans/2026-06-21-v19-71-pillar-cycle-9-p0.md §2 Track T2
//
//! Secret-holding wrapper types for the `pheno-config` substrate.
//!
//! All secret material (API keys, bearer tokens, database passwords) MUST be
//! held in a wrapper that derives `Zeroize` AND `ZeroizeOnDrop` so heap
//! snapshots cannot recover the secret bytes after the wrapper is dropped.
//!
//! Anti-patterns explicitly rejected (caught by `cargo deny` lint table
//! in `.cargo/audit-rules.toml`):
//!
//! - `String::from_utf8(secret_bytes)` — clones the secret into a heap
//!   `String` that never zeroizes.
//! - `String::from_utf8_lossy(secret_bytes)` — same as above.
//! - `format!("Bearer {}", api_key)` — clones the secret into the
//!   format buffer.
//! - `api_key.as_bytes().to_vec()` — clones the secret into a `Vec<u8>`
//!   that never zeroizes.
//! - `tracing::Span::field(api_key)` — tracing fields are retained for
//!   the span lifetime and may be exported to OTLP sinks.
//!
//! Consumers MUST NOT clone the inner bytes via `.to_vec()` / `.to_string()`;
//! use `expose_secret()` to borrow read-only, then pass the borrow into the
//! downstream TLS / DB layer.

use std::fmt;
use zeroize::{Zeroize, ZeroizeOnDrop};

/// Error returned by the secret constructors.
#[derive(Debug, PartialEq, Eq)]
pub enum SecretError {
    /// Caller passed an empty byte slice.
    Empty,
}

impl fmt::Display for SecretError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SecretError::Empty => write!(f, "secret bytes are empty"),
        }
    }
}

impl std::error::Error for SecretError {}

/// External HTTP API key (Anthropic, OpenAI, GitHub PAT, etc.).
///
/// Wraps `Box<[u8]>` so the inner buffer is heap-allocated and can be
/// zeroed on `Drop`. The `Zeroize` derive wipes the buffer via
/// `write_volatile` + `compiler_fence(seq_cst)` — the same primitive
/// used by `age`, `rusqlite` (bundled-sqlcipher), and `RustCrypto`
/// secret types.
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct ApiKey(Box<[u8]>);

/// OAuth bearer token / JWT.
///
/// Same `Zeroize` + `ZeroizeOnDrop` semantics as [`ApiKey`]; named
/// separately so call sites self-document which kind of credential is
/// being passed.
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct BearerToken(Box<[u8]>);

/// Database connection password.
///
/// Same `Zeroize` + `ZeroizeOnDrop` semantics as [`ApiKey`]; named
/// separately because database passwords are typically rotated on a
/// shorter cadence than API keys, and the rotation flow may want to
/// dispatch on the type.
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct DatabasePassword(Box<[u8]>);

/// Manual `Debug` impl that redacts the secret bytes. The default
/// `#[derive(Debug)]` would print the buffer contents (defeating
/// zeroization on `eprintln!` panic messages).
impl fmt::Debug for ApiKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ApiKey").field("len", &self.0.len()).finish()
    }
}

/// Manual `Debug` impl that redacts the secret bytes.
impl fmt::Debug for BearerToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BearerToken").field("len", &self.0.len()).finish()
    }
}

/// Manual `Debug` impl that redacts the secret bytes.
impl fmt::Debug for DatabasePassword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DatabasePassword").field("len", &self.0.len()).finish()
    }
}

macro_rules! impl_secret {
    ($name:ident) => {
        impl $name {
            /// Construct a secret wrapper from a byte slice.
            ///
            /// Returns [`SecretError::Empty`] when `bytes` is empty.
            /// The bytes are copied into a `Box<[u8]>` so the source
            /// buffer can be zeroized independently by the caller.
            pub fn new(bytes: &[u8]) -> Result<Self, SecretError> {
                if bytes.is_empty() {
                    return Err(SecretError::Empty);
                }
                Ok(Self(bytes.into()))
            }

            /// Borrow the underlying secret bytes read-only.
            ///
            /// The borrow lifetime is bound to `&self` to prevent
            /// accidental `.to_vec()` / `.to_string()` cloning that
            /// would defeat zeroization.
            pub fn expose_secret(&self) -> &[u8] {
                &self.0
            }

            /// Length of the secret in bytes. Useful for diagnostic
            /// logging without exposing the secret contents.
            pub fn len(&self) -> usize {
                self.0.len()
            }

            /// Whether the secret buffer is empty. Always `false`
            /// after a successful `new()` (the constructor rejects
            /// empty input); provided for symmetry with `len()`.
            pub fn is_empty(&self) -> bool {
                self.0.is_empty()
            }
        }
    };
}

impl_secret!(ApiKey);
impl_secret!(BearerToken);
impl_secret!(DatabasePassword);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input_rejected() {
        // Match on the result to assert the variant without requiring
        // `PartialEq` on the secret types (which would force a
        // comparison that defeats zeroization).
        assert!(matches!(ApiKey::new(&[]), Err(SecretError::Empty)));
        assert!(matches!(BearerToken::new(&[]), Err(SecretError::Empty)));
        assert!(matches!(DatabasePassword::new(&[]), Err(SecretError::Empty)));
    }

    #[test]
    fn round_trip_bytes() {
        let k = ApiKey::new(b"sk-test-1234").expect("non-empty");
        assert_eq!(k.expose_secret(), b"sk-test-1234");
        assert_eq!(k.len(), 12); // "sk-test-1234" is 12 bytes
        let t = BearerToken::new(b"eyJhbGciOiJIUzI1NiJ9.test").expect("non-empty");
        assert_eq!(t.expose_secret(), b"eyJhbGciOiJIUzI1NiJ9.test");
        let p = DatabasePassword::new(b"hunter2").expect("non-empty");
        assert_eq!(p.expose_secret(), b"hunter2");
    }

    #[test]
    fn debug_redacts_secret() {
        let k = ApiKey::new(b"sk-test-1234").expect("non-empty");
        let dbg = format!("{:?}", k);
        // The Debug output identifies the wrapper type.
        assert!(dbg.contains("ApiKey"), "Debug must show type: {}", dbg);
        // Critical: the secret bytes must NOT appear in Debug output.
        assert!(
            !dbg.contains("sk-test-1234"),
            "Debug leaked secret bytes: {}",
            dbg
        );
    }

    #[test]
    fn types_are_zeroize_on_drop() {
        // The compile-time `#[derive(Zeroize, ZeroizeOnDrop)]` is the
        // contract: any non-derivation is a build error. This test
        // simply pins the API surface.
        fn assert_zeroize_on_drop<T: ZeroizeOnDrop>() {}
        assert_zeroize_on_drop::<ApiKey>();
        assert_zeroize_on_drop::<BearerToken>();
        assert_zeroize_on_drop::<DatabasePassword>();
    }
}
