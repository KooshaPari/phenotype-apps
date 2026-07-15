# Security Policy

## Reporting a Vulnerability

> **Important:** Please **do not** open a public GitHub issue for security findings. Public disclosure leaks vulnerability details before a fix is available, which is harmful to users of KWatch and to the wider Phenotype ecosystem.

Use one of the private channels below, in order of preference:

1. **GitHub Security Advisories (preferred)** — open a private advisory at
   `https://github.com/Phenotype-org/KWatch/security/advisories/new`. This
   keeps the report off the public issue tracker until a fix is published.
2. **Direct maintainer contact** — reach out to the repository owner
   listed in `CODEOWNERS` if the advisory flow is not appropriate for
   the finding.

When reporting, include:

- A clear description of the issue and its impact
- Reproduction steps or a minimal PoC (call site, expected vs. actual
  behaviour, KWatch version, Go version, Node version, watched-project
  stack)
- Whether the issue is exploitable from a default install or requires
  specific configuration

## Response Timeline

| Stage               | Target            |
| ------------------- | ----------------- |
| Acknowledgment      | 48 hours          |
| Triage & Assessment | 5 business days   |
| Patch Release       | 14 business days (critical) |

