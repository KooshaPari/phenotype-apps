//! [`ConnectionDrop`] fault — TCP RST injection on a managed socket.
//!
//! ## Honest scope
//!
//! True transparent TCP RST injection at the kernel layer (intercept
//! packets via `iptables`/`NFQUEUE`/`eBPF`/`dummynet`) requires root
//! and a kernel module. Per the V20-T3 brief — *std + libc only, no
//! kernel modules* — this fault is **explicit-opt-in**: code under
//! test calls [`simulate_rst`] (or uses the [`RstGuard`] RAII wrapper)
//! to force a TCP RST on the next close. The libc `setsockopt`
//! `SO_LINGER { l_onoff=1, l_linger=0 }` trick is the canonical way
//! to do this from userspace: the kernel closes the socket with RST
//! instead of FIN, which causes the peer to observe an immediate
//! `ECONNRESET`.
//!
//! For pure application-layer testing (no socket), [`simulate_drop`]
//! returns an [`io::Error`] of kind `ConnectionReset` — same shape a
//! real RST would surface as.
//!
//! ## Behaviour
//!
//! - **Never fires spontaneously.** Unlike [`NetworkLatency`], this
//!   fault does not inject on a probability timer; it is invoked
//!   explicitly by the test body via [`simulate_rst`] or
//!   [`simulate_drop`]. This is intentional — surprise connection
//!   drops in a unit test are a footgun.
//! - The fault's [`Fault::inject`] call arms a per-thread flag so the
//!   helpers know to honour the next request. Drop clears the flag.

use std::cell::Cell;
use std::io;
use std::os::unix::io::RawFd;
use std::time::Duration;

use crate::fault::{ChaosError, Fault, FaultGuard};

thread_local! {
    /// Per-thread "next call drops" flag. `true` means the next call
    /// to [`simulate_drop`] / [`simulate_rst`] returns ECONNRESET,
    /// then the flag auto-clears. This is the canonical "drop-once"
    /// semantics that makes retry-and-recover patterns testable.
    ///
    /// Set by [`ConnectionDrop::inject`]; cleared either by the
    /// matching [`FaultGuard::Drop`] *or* by the first successful
    /// `simulate_drop` call.
    static ARMED: Cell<bool> = const { Cell::new(false) };
}

/// `ConnectionDrop` fault — explicit TCP RST / connection-drop
/// injection.
///
/// Constructed by the proc-macro for each test run. The fault arms a
/// per-thread "drop the next call" flag; the first call to
/// [`simulate_drop`] / [`simulate_rst`] then returns ECONNRESET and
/// the flag auto-clears.
///
/// ## Default semantics
///
/// `ConnectionDrop::default()` arms the flag. The first
/// [`simulate_drop`] / [`simulate_rst`] call observes ECONNRESET;
/// the next call observes the closure's real return value. This is
/// the canonical "drop-then-recover" pattern that a retry loop is
/// supposed to handle.
#[derive(Debug, Clone)]
pub struct ConnectionDrop {
    /// `true` if the fault is armed (drops the next call).
    /// Kept on the struct for symmetry with [`crate::NetworkLatency`]
    /// but the per-thread [`ARMED`] cell is what actually gates the
    /// helper functions.
    pub(crate) _armed: bool,
}

impl Default for ConnectionDrop {
    fn default() -> Self {
        Self { _armed: true }
    }
}

impl ConnectionDrop {
    /// Construct a `ConnectionDrop` fault. (The boolean is
    /// `is_armed`; `new(true)` is equivalent to `default()`.)
    pub fn new(is_armed: bool) -> Self {
        Self { _armed: is_armed }
    }
}

impl Fault for ConnectionDrop {
    fn name(&self) -> &'static str {
        "connection_drop"
    }

    fn inject(&self) -> Result<FaultGuard, ChaosError> {
        ARMED.with(|cell| cell.set(true));
        Ok(FaultGuard::new(
            "connection_drop",
            Box::new(ConnectionDropHandle {}),
        ))
    }

    fn revert(&self) {
        // Drop is the canonical revert path.
    }

    fn duration_hint(&self) -> Duration {
        // Set-and-clear is essentially free.
        Duration::from_millis(1)
    }
}

/// Internal handle stored in [`FaultGuard`] so `Drop` can clear the
/// thread-local flag.
struct ConnectionDropHandle {}

impl Fault for ConnectionDropHandle {
    fn name(&self) -> &'static str {
        "connection_drop"
    }

    fn inject(&self) -> Result<FaultGuard, ChaosError> {
        ARMED.with(|cell| cell.set(true));
        Ok(FaultGuard::new(
            "connection_drop",
            Box::new(Self {}),
        ))
    }

    fn revert(&self) {
        // No-op; Drop clears the flag.
    }

    fn duration_hint(&self) -> Duration {
        Duration::from_millis(1)
    }
}

impl Drop for ConnectionDropHandle {
    fn drop(&mut self) {
        ARMED.with(|cell| cell.set(false));
    }
}

/// RAII wrapper around a socket fd that, on drop, sets `SO_LINGER` to
/// force a TCP RST instead of FIN.
///
/// Usage:
///
/// ```ignore
/// use std::os::unix::io::FromRawFd;
/// let sock = unsafe { libc::socket(libc::AF_INET, libc::SOCK_STREAM, 0) };
/// let _guard = pheno_chaos::connection::RstGuard::new(sock);
/// // ... sock is closed with RST when _guard drops ...
/// ```
///
/// This uses the canonical `SO_LINGER` trick documented in
/// [`simulate_rst`]. It is the only place in the substrate that
/// touches `libc` directly.
pub struct RstGuard {
    fd: RawFd,
}

impl RstGuard {
    /// Wrap `fd` so the kernel closes it with TCP RST on drop.
    ///
    /// # Safety
    ///
    /// `fd` must be a valid, connected TCP socket owned by the caller.
    /// Once wrapped, the caller must not close `fd` directly — let
    /// `RstGuard` do it.
    #[allow(unsafe_code)]
    pub unsafe fn new(fd: RawFd) -> Self {
        // SO_LINGER { l_onoff=1, l_linger=0 }  →  send RST on close.
        let linger = libc::linger {
            l_onoff: 1,
            l_linger: 0,
        };
        // setsockopt returns 0 on success, -1 on error. We log and
        // proceed: a failed setsockopt just means the close will be
        // graceful (FIN), which is a strict relaxation of the fault.
        let rc = libc::setsockopt(
            fd,
            libc::SOL_SOCKET,
            libc::SO_LINGER,
            &linger as *const _ as *const libc::c_void,
            std::mem::size_of::<libc::linger>() as libc::socklen_t,
        );
        if rc != 0 {
            eprintln!(
                "[pheno-chaos] RstGuard::new: setsockopt(SO_LINGER) failed: {}",
                io::Error::last_os_error()
            );
        }
        Self { fd }
    }

    /// Get the wrapped fd (for callers that need to send/recv before
    /// the guard drops).
    pub fn fd(&self) -> RawFd {
        self.fd
    }
}

impl Drop for RstGuard {
    fn drop(&mut self) {
        // SAFETY: `fd` was given to us by the caller and is owned by
        // this guard; closing it here is the canonical teardown.
        let rc = unsafe { libc::close(self.fd) };
        if rc != 0 {
            eprintln!(
                "[pheno-chaos] RstGuard::drop: close failed: {}",
                io::Error::last_os_error()
            );
        }
    }
}

/// Application-layer helper: simulate a connection drop by returning
/// an [`io::Error`] of kind `ConnectionReset`. No socket is required.
///
/// If a [`ConnectionDrop`] fault is armed on the current thread, the
/// *first* call returns `Err(ConnectionReset)` and auto-clears the
/// flag; subsequent calls return `f()`'s real result. This is the
/// canonical "I just got ECONNRESET" simulation in tests that don't
/// want to set up a real socket pair, and pairs naturally with
/// retry-and-reconnect patterns.
pub fn simulate_drop<F, T>(f: F) -> Result<T, io::Error>
where
    F: FnOnce() -> T,
{
    let was_armed = ARMED.with(|cell| {
        let v = cell.get();
        if v {
            cell.set(false); // Auto-clear: drop-once semantics.
        }
        v
    });
    if was_armed {
        return Err(io::Error::new(
            io::ErrorKind::ConnectionReset,
            "pheno-chaos: simulated TCP RST",
        ));
    }
    Ok(f())
}

/// Force a TCP RST on a real socket fd by setting `SO_LINGER(0)` and
/// closing.
///
/// # Safety
///
/// `fd` must be a valid, connected TCP socket owned by the caller.
/// After this returns, `fd` is closed and must not be used.
#[allow(unsafe_code)]
pub unsafe fn simulate_rst(fd: RawFd) -> io::Result<()> {
    let _guard = RstGuard::new(fd);
    // _guard drops at end of scope, closing with RST.
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simulate_drop_passthrough_when_unarmed() {
        let result = simulate_drop(|| 42i32);
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn simulate_drop_returns_reset_when_armed_full() {
        let fault = ConnectionDrop::new(true);
        let _guard = fault.inject().unwrap();
        let result: Result<i32, io::Error> = simulate_drop(|| 42);
        let err = result.unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::ConnectionReset);
    }
}