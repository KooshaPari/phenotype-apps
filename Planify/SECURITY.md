# Security Policy

## Supported Versions

The latest release of Planify receives security updates. We do not maintain
long-term support for older versions — please keep your deployment up to date.

## Reporting a Vulnerability

We take security seriously. If you discover a vulnerability in Planify or any
of its dependencies (including the upstream Plane codebase), please report it
privately before disclosing it publicly.

### How to report

- **Email**: `security@phenotype.space`
- **GPG key fingerprint**: `0000 0000 0000 0000 0000  0000 0000 0000 0000 0000`
  (placeholder — we will publish a real key after first disclosure)

### What to include

- A clear description of the issue
- Steps to reproduce (or a proof of concept)
- Affected versions or components
- Any suggested fix (if available)

### What to expect

1. **Acknowledgment** within 48 hours of your report
2. **Initial assessment** within 5 business days — we will confirm the issue,
   its severity, and an estimated timeline for a fix
3. **Coordinated disclosure** — we will work with you to schedule a public
   announcement after the fix is released

We aim to release fixes for:
- **Critical / High** severity: within 14 days
- **Medium** severity: within 30 days
- **Low** severity: within 90 days

## Scope

This policy covers all code in the `KooshaPari/Planify` repository, including
the `upstream/` Plane seed (which may have its own reporting channels at
`makeplane/plane`). When the vulnerability originates from upstream Plane
code, we will coordinate with the Plane security team.

## Bug Bounty

We do not currently offer a bug bounty program. Security researchers who
report valid vulnerabilities will be acknowledged in release notes (with
consent).

## Responsible Disclosure

We ask that you:
- Do not disclose the vulnerability publicly until we have released a fix
  and given the community time to update
- Do not exploit the vulnerability beyond what is necessary to demonstrate it
- Do not access, modify, or exfiltrate user data
