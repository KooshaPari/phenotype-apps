//! Property-based tests for `pheno-port-adapter` (v16 T6 L25).
//!
//! These properties pin the invariants of the [`PortAdapter`] trait
//! contract and its [`AdapterError`] error type. The concrete
//! [`TcpAdapter`] and (cfg(unix)) [`UnixAdapter`] are exercised
//! against a per-test tempfile fixture so that no two test runs ever
//! share a socket path.
//!
//! Run on macbook with `PROPTEST_CASES=64 cargo test --test
//! proptest_adapter` to keep wall under 10 minutes.

use std::io::{Read, Write};
use std::net::TcpListener;
use std::os::unix::net::UnixListener;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use proptest::prelude::*;
use tempfile::TempDir;

use pheno_port_adapter::{AdapterError, PortAdapter};

// ── Fixture: per-test ephemeral socket directory ─────────────────────

/// Spin up a TCP echo listener on `127.0.0.1:0` (OS-assigned port)
/// and return the address + the server's join handle. The fixture
/// does NOT use a shared static `TempDir`; each invocation gets a
/// fresh listener bound to an ephemeral port.
fn tcp_listener() -> (String, thread::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind ephemeral TCP port");
    let addr = listener.local_addr().expect("local_addr");
    let handle = thread::spawn(move || {
        if let Ok((mut stream, _)) = listener.accept() {
            // Echo a few bytes back so the client can probe the
            // connection's liveness without us writing more
            // elaborate protocol logic.
            let mut buf = [0u8; 64];
            if let Ok(n) = stream.read(&mut buf) {
                let _ = stream.write_all(&buf[..n]);
            }
        }
    });
    (addr.to_string(), handle)
}

/// Build a per-test fixture directory with a Unix-domain socket
/// bound to a unique path inside it. The [`TempDir`] returned by
/// `tempfile::tempdir()` is moved into the closure so the directory
/// is removed when the test finishes — no shared static, no manual
/// cleanup, no leaked sockets across test runs.
fn unix_listener_in_tempdir() -> (TempDir, std::path::PathBuf, thread::JoinHandle<()>) {
    let dir = tempfile::tempdir().expect("create tempdir for unix socket");
    let path = dir.path().join(format!(
        "pheno-port-adapter-{}.sock",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0)
    ));
    let listener = UnixListener::bind(&path).expect("bind unix listener");
    let handle = thread::spawn(move || {
        if let Ok((mut stream, _)) = listener.accept() {
            let mut buf = [0u8; 64];
            let _ = stream.read(&mut buf);
        }
    });
    (dir, path, handle)
}

// ── AdapterError invariants ──────────────────────────────────────────

proptest! {
    /// `AdapterError::Timeout` has no payload and its `Display` is
    /// exactly the literal string "timeout" — consumers pattern-match
    /// on the `Timeout` variant for retry-policy decisions.
    #[test]
    fn timeout_display_is_stable_literal(_dummy in 0u32..1) {
        let err = AdapterError::Timeout;
        prop_assert_eq!(err.to_string(), "timeout");
    }

    /// `ConnectFailed(_)` Display starts with "connect failed: ".
    #[test]
    fn connect_failed_display_prefix_is_stable(msg in "[ -~]{1,32}") {
        let err = AdapterError::ConnectFailed(msg.clone());
        let s = err.to_string();
        prop_assert!(s.starts_with("connect failed: "), "Display {:?}", s);
        prop_assert!(s.contains(&msg), "Display {:?} missing payload", s);
    }

    /// `DisconnectFailed(_)` Display starts with "disconnect failed: ".
    #[test]
    fn disconnect_failed_display_prefix_is_stable(msg in "[ -~]{1,32}") {
        let err = AdapterError::DisconnectFailed(msg.clone());
        let s = err.to_string();
        prop_assert!(s.starts_with("disconnect failed: "), "Display {:?}", s);
        prop_assert!(s.contains(&msg));
    }

    /// `HealthCheckFailed(_)` Display starts with "health check failed: ".
    #[test]
    fn health_check_failed_display_prefix_is_stable(msg in "[ -~]{1,32}") {
        let err = AdapterError::HealthCheckFailed(msg.clone());
        let s = err.to_string();
        prop_assert!(s.starts_with("health check failed: "), "Display {:?}", s);
        prop_assert!(s.contains(&msg));
    }
}

// ── TcpAdapter invariants ────────────────────────────────────────────

proptest! {
    #![proptest_config(ProptestConfig::with_cases(16))]

    /// Connecting to a `127.0.0.1` endpoint that has no listener
    /// must always fail with `AdapterError::ConnectFailed`, never
    /// `Timeout` (TCP connect has no first-class timeout) and never
    /// `HealthCheckFailed` (the connect phase never runs health).
    #[test]
    fn connect_to_dead_endpoint_returns_connect_failed(_dummy in 0u32..1) {
        let adapter = pheno_port_adapter::adapters::tcp::TcpAdapter::new();
        // Port 1 on localhost is reserved + almost never bound.
        let result = adapter.connect("127.0.0.1:1");
        prop_assert!(
            matches!(result, Err(AdapterError::ConnectFailed(_))),
            "expected ConnectFailed, got {:?}", result
        );
    }

    /// Connecting to an empty endpoint always fails with
    /// `ConnectFailed("empty endpoint")` regardless of any
    /// other state on the adapter.
    #[test]
    fn connect_to_empty_endpoint_returns_connect_failed(_dummy in 0u32..1) {
        let adapter = pheno_port_adapter::adapters::tcp::TcpAdapter::new();
        let result = adapter.connect("");
        prop_assert!(matches!(result, Err(AdapterError::ConnectFailed(_))));
    }

    /// `name()` is the stable `"tcp"` literal — used by
    /// tracing/metrics labels in production.
    #[test]
    fn tcp_adapter_name_is_tcp(_dummy in 0u32..1) {
        let adapter = pheno_port_adapter::adapters::tcp::TcpAdapter::new();
        prop_assert_eq!(adapter.name(), "tcp");
    }

    /// `disconnect()` on a freshly-constructed adapter is a no-op
    /// that returns `Ok(())`. Multiple consecutive disconnects all
    /// return `Ok(())` (idempotent).
    #[test]
    fn disconnect_is_idempotent_when_never_connected(n in 1u32..5) {
        let adapter = pheno_port_adapter::adapters::tcp::TcpAdapter::new();
        for _ in 0..n {
            prop_assert!(adapter.disconnect().is_ok());
        }
    }

    /// `health()` on a freshly-constructed (never-connected) adapter
    /// must return `Err(HealthCheckFailed(_))` rather than `Ok(())`,
    /// so consumers don't accidentally treat an unconnected adapter
    /// as healthy.
    #[test]
    fn health_when_disconnected_returns_health_check_failed(_dummy in 0u32..1) {
        let adapter = pheno_port_adapter::adapters::tcp::TcpAdapter::new();
        let result = adapter.health();
        prop_assert!(
            matches!(result, Err(AdapterError::HealthCheckFailed(_))),
            "expected HealthCheckFailed, got {:?}", result
        );
    }

    /// Successful connect/disconnect round-trip: connect to a real
    /// listener, then disconnect; afterwards the adapter must
    /// report `HealthCheckFailed` (proving the connection was
    /// actually torn down). We don't read `Connection::id`
    /// because it's `pub(crate)` and not part of the public API.
    #[test]
    fn connect_to_listener_then_disconnect_succeeds(_dummy in 0u32..1) {
        let (addr, handle) = tcp_listener();
        let adapter = pheno_port_adapter::adapters::tcp::TcpAdapter::new();
        let _conn = adapter.connect(&addr).expect("connect to echo listener");
        prop_assert!(adapter.disconnect().is_ok());
        // After disconnect the adapter is back to the unconnected state.
        prop_assert!(matches!(
            adapter.health(),
            Err(AdapterError::HealthCheckFailed(_))
        ));
        let _ = handle.join();
    }

    /// Reconnect replaces the previous connection: after a second
    /// `connect()` and a final `disconnect()` the adapter returns
    /// to the unconnected state.
    #[test]
    fn reconnect_replaces_previous_connection(_dummy in 0u32..1) {
        let (addr1, h1) = tcp_listener();
        let (addr2, h2) = tcp_listener();
        let adapter = pheno_port_adapter::adapters::tcp::TcpAdapter::new();
        let _ = adapter.connect(&addr1).expect("first connect");
        let _conn2 = adapter.connect(&addr2).expect("second connect");
        prop_assert!(adapter.disconnect().is_ok());
        prop_assert!(matches!(
            adapter.health(),
            Err(AdapterError::HealthCheckFailed(_))
        ));
        let _ = h1.join();
        let _ = h2.join();
    }
}

// ── UnixAdapter invariants (cfg(unix) only) ──────────────────────────

#[cfg(unix)]
proptest! {
    #![proptest_config(ProptestConfig::with_cases(8))]

    /// Unix-domain socket adapter round-trip uses a per-test
    /// tempfile fixture, so two test runs never share the same
    /// socket path (L25 test-isolation invariant). We don't
    /// assert on `conn.id` (pub(crate) only).
    #[test]
    fn unix_connect_disconnect_round_trip(_dummy in 0u32..1) {
        let (_dir, path, handle) = unix_listener_in_tempdir();
        let adapter = pheno_port_adapter::adapters::unix::UnixAdapter::new();
        let path_str = path.to_string_lossy().into_owned();
        let _conn = adapter.connect(&path_str).expect("connect to unix listener");
        prop_assert!(adapter.health().is_ok());
        prop_assert!(adapter.disconnect().is_ok());
        let _ = handle.join();
    }

    /// Unix `name()` is the stable `"unix"` literal.
    #[test]
    fn unix_adapter_name_is_unix(_dummy in 0u32..1) {
        let adapter = pheno_port_adapter::adapters::unix::UnixAdapter::new();
        prop_assert_eq!(adapter.name(), "unix");
    }

    /// Unix connect to an empty endpoint fails with
    /// `ConnectFailed("empty endpoint")`.
    #[test]
    fn unix_connect_to_empty_endpoint_fails(_dummy in 0u32..1) {
        let adapter = pheno_port_adapter::adapters::unix::UnixAdapter::new();
        let result = adapter.connect("");
        prop_assert!(matches!(result, Err(AdapterError::ConnectFailed(_))));
    }

    /// Unix `health()` on a freshly-constructed adapter returns
    /// `HealthCheckFailed`.
    #[test]
    fn unix_health_when_disconnected_returns_health_check_failed(_dummy in 0u32..1) {
        let adapter = pheno_port_adapter::adapters::unix::UnixAdapter::new();
        prop_assert!(matches!(
            adapter.health(),
            Err(AdapterError::HealthCheckFailed(_))
        ));
    }
}

// ── Cross-adapter invariant: shared-Arc adapter is Send+Sync ─────────
//
// The `PortAdapter` trait requires `Send + Sync`; this test pins
// the contract under `Arc` sharing across threads so a future
// refactor cannot silently strip the bound without breaking it.

proptest! {
    #![proptest_config(ProptestConfig::with_cases(4))]

    /// 8 threads concurrently calling `disconnect()` on the same
    /// `Arc<TcpAdapter>` must all observe `Ok(())` — the
    /// interior-mutex contract is linearizable.
    #[test]
    fn shared_tcp_adapter_disconnect_is_thread_safe(_dummy in 0u32..1) {
        let adapter = Arc::new(pheno_port_adapter::adapters::tcp::TcpAdapter::new());
        let handles: Vec<_> = (0..8)
            .map(|_| {
                let a = Arc::clone(&adapter);
                thread::spawn(move || a.disconnect().is_ok())
            })
            .collect();
        for h in handles {
            prop_assert!(h.join().expect("thread panicked"));
        }
    }

    /// 8 threads concurrently calling `health()` on a shared
    /// `Arc<TcpAdapter>` must all observe the same `HealthCheckFailed`
    /// error — the unconnected state is visible consistently.
    #[test]
    fn shared_tcp_adapter_health_is_thread_safe(_dummy in 0u32..1) {
        let adapter = Arc::new(pheno_port_adapter::adapters::tcp::TcpAdapter::new());
        let handles: Vec<_> = (0..8)
            .map(|_| {
                let a = Arc::clone(&adapter);
                thread::spawn(move || matches!(a.health(), Err(AdapterError::HealthCheckFailed(_))))
            })
            .collect();
        for h in handles {
            prop_assert!(h.join().expect("thread panicked"));
        }
    }

    /// A `connect` attempt against a dead localhost port must return
    /// a `ConnectFailed` error in bounded time (<5s). We use
    /// `127.0.0.1:1` (unprivileged, almost never bound) rather
    /// than an unroutable RFC 5737 address because the former
    /// fails with `ECONNREFUSED` in milliseconds, whereas the
    /// latter relies on the OS TCP-connect timeout (which can be
    /// tens of seconds on some platforms).
    #[test]
    fn connect_to_dead_localhost_returns_in_bounded_time(_dummy in 0u32..1) {
        let adapter = pheno_port_adapter::adapters::tcp::TcpAdapter::new();
        let start = std::time::Instant::now();
        let result = adapter.connect("127.0.0.1:1");
        let elapsed = start.elapsed();
        prop_assert!(matches!(result, Err(AdapterError::ConnectFailed(_))));
        prop_assert!(
            elapsed < Duration::from_secs(5),
            "connect took {:?} — possible hang", elapsed
        );
    }
}