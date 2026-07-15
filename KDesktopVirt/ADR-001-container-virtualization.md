# ADR-001: Container-Based Desktop Virtualization over VM-Based VDI

Date: 2025-04-04

## Context

KDesktopVirt requires a virtualization layer for desktop sessions. We must choose between traditional VM-based Virtual Desktop Infrastructure (VDI) and modern container-based approaches.

Traditional VDI (Citrix, VMware Horizon, Amazon WorkSpaces) uses full virtual machines:

```
VM-Based VDI:
┌─────────────────────────────────────────┐
│  Windows/Linux VM (8-16GB RAM)         │
│  ├─ Full OS kernel                      │
│  ├─ Multiple services                     │
│  ├─ Persistent state                    │
│  └─ Minutes to boot                     │
└─────────────────────────────────────────┘
```

Container-based desktops use Linux namespaces and cgroups:

```
Container Desktop:
┌─────────────────────────────────────────┐
│  Linux Container (2-4GB RAM)            │
│  ├─ Shared kernel (host)                │
│  ├─ Minimal services                    │
│  ├─ Selective persistence               │
│  └─ Seconds to start                     │
└─────────────────────────────────────────┘
```

The choice impacts:
- Resource efficiency (cost per session)
- Startup latency (user/agent wait time)
- Isolation guarantees (security boundaries)
- Operational complexity (maintenance burden)

## Decision

We will use container-based desktop virtualization as the primary virtualization layer, with optional VM-based isolation via Kata Containers for high-security scenarios.

### Rationale

1. **Resource Efficiency**
   - Containers share the host kernel, reducing memory overhead by 60-80%
   - Typical session: 2-4GB vs 8-16GB for VMs
   - Enables 3-4x session density on same hardware

2. **Startup Speed**
   - Cold start: 2-3 seconds vs 30-60 seconds for VMs
   - Critical for ephemeral agent workflows
   - No boot process—container runtime starts directly

3. **Operational Simplicity**
   - Docker ecosystem mature and well-understood
   - Existing tooling for image management
   - CI/CD integration proven at scale

4. **Isolation Adequacy**
   - Linux namespaces provide sufficient isolation for automation use cases
   - gVisor or Kata available for enhanced security
   - Single-purpose sessions reduce attack surface

### Architecture

```
KDesktopVirt Virtualization Stack:

┌─────────────────────────────────────────┐
│  Desktop Session Container              │
│  ├─ KDE Plasma / XFCE / Custom         │
│  ├─ Application stack                   │
│  ├─ Automation agent                     │
│  └─ X11/VNC display                      │
├─────────────────────────────────────────┤
│  Container Runtime (Docker/containerd)    │
│  ├─ Namespaces (pid, net, mount)         │
│  ├─ Cgroups (cpu, memory, io)           │
│  ├─ Seccomp (syscall filtering)         │
│  └─ Capabilities (dropped privileges)   │
├─────────────────────────────────────────┤
│  Optional: Kata/gVisor (VM isolation)     │
├─────────────────────────────────────────┤
│  Host Kernel                            │
└─────────────────────────────────────────┘
```

### Image Strategy

Base images follow a hierarchy:

```
Image Hierarchy:

kvirtualstage/base:ubuntu-24.04
    │
    ├── kvirtualstage/desktop:kde-plasma
    │       └─ Pre-installed: KDE, Firefox, LibreOffice
    │
    ├── kvirtualstage/desktop:xfce
    │       └─ Pre-installed: XFCE, basic apps
    │
    └── kvirtualstage/custom:<user-defined>
            └─ Derived from base + user packages
```

## Status

Accepted

## Consequences

### Positive

- **Lower infrastructure costs**: 3-4x better resource utilization
- **Faster agent workflows**: Sub-3-second session startup
- **Simpler operations**: Leverage Docker ecosystem
- **Faster CI/CD**: Container images build and deploy quickly

### Negative

- **Reduced isolation vs VMs**: Shared kernel (mitigated by gVisor/Kata option)
- **Linux-only guests**: No Windows desktop support (acceptable for target use cases)
- **X11 dependency**: Wayland support still maturing for automation

### Neutral

- **Image management**: Requires container registry and versioning strategy
- **Storage**: Container layers require different backup strategy than VM disks

## Alternatives Considered

### Full VM (KVM/Xen)

Rejected due to:
- Resource overhead too high for ephemeral use cases
- Boot time unacceptable for agent workflows
- Operational complexity of VM lifecycle management

### Kata Containers as Default

Rejected due to:
- 2-3x resource overhead vs containers
- Startup time 5-10 seconds (acceptable but slower)
- Kept as opt-in for high-security scenarios

### WebAssembly (Wasm)

Rejected due to:
- No GUI support ( WASI not ready)
- Application recompilation required
- Browser-focused, not desktop automation

## Related Decisions

- ADR-002: UI Automation Architecture
- ADR-003: MCP as Primary Integration Interface

## References

1. Felter, W., et al. (2015). "An Updated Performance Comparison of Virtual Machines and Linux Containers". IC2E.
2. Kata Containers Documentation. https://katacontainers.io/
3. Docker Security Documentation. https://docs.docker.com/engine/security/
