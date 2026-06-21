# Hexagonal Port Pattern (L4)

> **Status:** ACTIVE (ADR-038, 2026-06-18)
> **Owner:** pheno-port-adapter
> **Applies to:** every substrate repo that ships a pluggable capability

This document is the canonical reference for the **Hexagonal Port** (a.k.a. "ports and adapters") pattern as adopted by the pheno-* fleet. The pattern is enforced in CI by `scripts/check-hex-ports.sh` and the per-repo gate scripts derived from it. Read this before adding a new port, an adapter, or refactoring an existing capability boundary.

## 1. Why hexagonal

Hexagonal architecture (Alistair Cockburn, 2005) decouples **application code** from **infrastructure code** by inverting the dependency: the application declares *what it needs* as a port (a Rust trait) and infrastructure code provides the *concrete implementation* as an adapter. The application never imports an adapter directly; it depends only on the trait. This gives us three things for free:

1. **Testability** — drop in a `MockAdapter` (in-process) instead of a real Redis/Tcp/OTLP backend.
2. **Substitutability** — swap backends (in-memory → Redis → Dragonfly) without touching application code.
3. **Policy compliance** — every port is governed by the same naming, error, and observability rules, so the fleet is uniform.

The pattern is policy, not preference. New adapters MUST be added via a port trait, never by leaking a concrete type across a module boundary.

## 2. Port trait definition

A **port** is an `async`-capable trait in `pheno-port-adapter::ports::<capability>`. The trait captures one side of an application/service boundary without committing to a concrete backing technology. Example from the canonical `HexCachePort`:

```rust
// pheno-port-adapter/src/ports/cache.rs
#[async_trait]
pub trait HexCachePort: Send + Sync {
    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>, CacheError>;
    async fn put(&self, key: &str, value: Vec<u8>, ttl: Duration) -> Result<(), CacheError>;
    async fn invalidate(&self, key: &str) -> Result<(), CacheError>;
}
```

### 2.1 Required trait properties

Every port trait MUST:

| Property | Why |
| :-- | :-- |
| `Send + Sync` | Consumers run on multi-threaded executors; stored as `Arc<dyn Port>`. |
| Be `#[async_trait]` (when async) | Keeps the trait object-safe; callers may want `Box<dyn Port>` for late binding. |
| Surface its own error type | No shared global `AdapterError`. Each port names what *it* can fail. |
| Document at the trait level | (a) what it abstracts, (b) what guarantees adapters MUST uphold, (c) what is out of scope. |
| Be `dyn`-compatible | No generic methods, no `Self` in return position other than `-> Self` in builders. |

### 2.2 Synchronous ports (e.g. `PortAdapter`, `HexTimePort`)

Some ports are intentionally **not** `async` because their operations are cheap, side-effect-free queries. The same rules apply minus the `#[async_trait]` attribute:

```rust
pub trait HexTimePort: Send + Sync {
    fn now(&self) -> Instant;
    fn unix_nanos(&self) -> u64;
}
```

## 3. Adapter implementation pattern

An **adapter** is a concrete struct under `pheno-port-adapter::adapters::<backend>` that `impl PortTrait for AdapterStruct`. Adapters own their backend's state and lifecycle; the trait methods are thin wrappers that translate calls into the backend's native API.

```rust
// pheno-port-adapter/src/adapters/in_memory_cache.rs
#[async_trait]
impl HexCachePort for InMemoryCache {
    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>, CacheError> {
        if key.is_empty() {
            return Err(CacheError::InvalidKey("empty key".to_string()));
        }
        let mut guard = self.inner.lock().await;
        Ok(guard.get(key).cloned().map(|e| e.value))
    }
    // ... put, invalidate
}
```

### 3.1 Required adapter properties

Every adapter struct MUST:

| Property | Why |
| :-- | :-- |
| Be a **named struct** (not a type alias) | The CI gate (`check-hex-ports.sh`) discovers adapters by `pub struct <Name>` declaration. |
| Live in `pheno-port-adapter::adapters::<backend>` | Convention; do not scatter adapters across sub-crates. |
| Implement **at least one** port trait | The gate enforces this. An adapter that does not implement a port is a smell — promote it to a port first. |
| Be `pub` and re-exported from `adapters::mod` | So downstream crates can `use pheno_port_adapter::adapters::InMemoryCache`. |
| Be `Default + Clone + Debug` if stateful | Eases test setup and `Arc<dyn Port>` cloning. |
| Document its purpose at the struct level | One-paragraph doc comment naming the backend, the operational caveats, and any limits. |
| Map backend errors into the port's error type | Never leak `redis::RedisError` / `std::io::Error` across the trait boundary. |

### 3.2 State management

If the adapter holds mutable state (a connection, a hashmap, a clock handle), the state lives in an interior `Mutex<…>` / `RwLock<…>` / `tokio::sync::Mutex<…>` so the trait methods can take `&self` and remain object-safe. The choice of mutex flavour is dictated by whether the lock guard crosses an `.await` point:

| Mutex | When to use |
| :-- | :-- |
| `std::sync::Mutex` | Sync ports (`HexTimePort`); the guard never crosses `.await`. |
| `tokio::sync::Mutex` | Async ports where the guard crosses `.await` (e.g. `InMemoryCache`). |
| `parking_lot::Mutex` | Sync ports that want faster lock acquisition; equivalent to `std::sync::Mutex` for our use. |
| `tokio::sync::RwLock` | Read-heavy async paths where the read guard crosses `.await`. |

## 4. Naming conventions

| Construct | Convention | Example |
| :-- | :-- | :-- |
| **Port trait** | `Hex<Capability>Port` | `HexCachePort`, `HexTimePort`, `HexConfigPort` |
| **Port error** | `<Capability>Error` | `CacheError`, `TimeError` (the latter unused — `HexTimePort` is infallible) |
| **Adapter struct** | `<Backend><Capability>` or `<Capability><Backend>` | `InMemoryCache`, `RedisAdapter`, `SystemClock`, `MockClock`, `TcpAdapter` |
| **Module path** | `pheno_port_adapter::ports::<capability>` / `pheno_port_adapter::adapters::<backend>` | `ports::cache`, `adapters::in_memory_cache` |
| **Re-export** | Flat (not nested) at `pheno_port_adapter::` | `pub use ports::HexCachePort;` in `lib.rs` |
| **Doc comment** | One-paragraph `//!` module doc naming what + why + limits | See `ports/cache.rs` and `adapters/in_memory_cache.rs` |

### 4.1 Prefixes

- `Hex*` — hexagonal port trait (the "P").
- No prefix — adapter struct; the suffix (`Cache`, `Clock`, `Adapter`) signals the role.
- `Mock*` — test-only adapter; not for production. Lives next to the production adapter.
- `InMemory*` / `InProc*` — process-local adapter. Default for tests and single-node binaries.

## 5. Anti-patterns

These are CI-flagged violations. New code that hits one of these is rejected at review.

### 5.1 Leaking a concrete backend type across a trait boundary

```rust
// BAD: callers now depend on redis-rs
#[async_trait]
pub trait HexCachePort: Send + Sync {
    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>, redis::RedisError>;
}
```

The error type belongs to the **port**, not the backend. Map `redis::RedisError` → `CacheError::Backend(_)` inside the adapter.

### 5.2 Application code importing a concrete adapter

```rust
// BAD: application is now coupled to InMemoryCache
use pheno_port_adapter::adapters::InMemoryCache;
struct App { cache: InMemoryCache }

// GOOD: application depends on the trait
use pheno_port_adapter::HexCachePort;
struct App { cache: Arc<dyn HexCachePort> }
```

### 5.3 Adapter that does not implement a port

```rust
// BAD: this is a struct that does "adapter-like" things but has no port contract.
// Either give it a port, or downgrade it to a plain utility.
pub struct WeirdHelper;
```

If you cannot articulate "this implements HexXPort", you have not designed a port. Stop and design the port first.

### 5.4 Returning `Self` from a non-builder method

```rust
// BAD: breaks dyn-compatibility
pub trait HexCachePort: Send + Sync {
    async fn clone_cache(&self) -> Self;  // not object-safe
}
```

Use a separate `Clone` impl on the concrete adapter; the trait should not promise cloning.

### 5.5 Port trait that is generic over a backend

```rust
// BAD: the whole point of the port is to hide the backend
pub trait HexCachePort<B: Backend>: Send + Sync {
    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>, B::Error>;
}
```

The trait should be backend-agnostic. The adapter is the only place where the backend type appears.

### 5.6 Adapter that owns a global / static

```rust
// BAD: not testable, not multi-tenant
pub struct RedisAdapter {
    client: &'static redis::Client,
}
```

Own your state; pass it in via `new()` or `with_config()`. Static lifetimes are a code smell unless the type is genuinely `'static + Send + Sync` (e.g. an empty marker struct).

### 5.7 Missing `Send + Sync`

```rust
// BAD: cannot be stored as Arc<dyn Port> on a multi-thread executor
pub trait HexConfigPort {
    fn get(&self, key: &str) -> Option<String>;
}
```

Always `: Send + Sync` on the trait bound. If you have a real reason to drop it, document the reason in the trait's `//!` doc and expect a review challenge.

### 5.8 Sharing `Result<_, Box<dyn Error>>` across ports

```rust
// BAD: callers cannot pattern-match on the failure mode
async fn get(&self, key: &str) -> Result<Option<Vec<u8>>, Box<dyn std::error::Error>>;
```

Each port has a *typed* error. `Box<dyn Error>` is a tell that the port author did not think through the failure modes.

## 6. Adoption checklist

To add a new port + adapter pair, in order:

1. **Port trait** in `pheno-port-adapter/src/ports/<capability>.rs` — declares the trait, error type, and contract docs.
2. **Re-export** from `pheno-port-adapter/src/ports/mod.rs` AND from `pheno-port-adapter/src/lib.rs` (flat re-export).
3. **Adapter** in `pheno-port-adapter/src/adapters/<backend>.rs` — `pub struct` that `impl PortTrait for AdapterStruct`.
4. **Re-export** the adapter from `pheno-port-adapter/src/adapters/mod.rs`.
5. **Smoke test** under `pheno-port-adapter/tests/` exercising the adapter **through the trait** (not the concrete type) — proves object-safety and contract.
6. **Run the gate** — `just check-hex-ports` (or `./scripts/check-hex-ports.sh pheno-port-adapter`) must pass.
7. **Adoption marker** in adopting repos — drop a `docs/HEXAGONAL_PORTS.md` reference file linking back to this document.

## 7. References

- `pheno-port-adapter/src/ports/cache.rs` — canonical `HexCachePort` example.
- `pheno-port-adapter/src/ports/time.rs` — canonical sync-port example (`HexTimePort`).
- `pheno-port-adapter/src/adapters/in_memory_cache.rs` — canonical `#[async_trait]` adapter.
- `pheno-port-adapter/src/adapters/system_clock.rs` — canonical sync adapter.
- `docs/adr/2026-06-18/ADR-038-hexagonal-port-adapter-l4-policy.md` — the policy ADR.
- `docs/adr/2026-06-15/ADR-014-hexagonal-ports.md` — original direction.
- `scripts/check-hex-ports.sh` — CI gate.
