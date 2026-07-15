---
name: 🚀 Feature request
description: Suggest a new feature or improvement
labels: enhancement
title: "[Feature]: "
---

**Is your feature request related to a problem? Please describe.**
A clear and concise description of what the problem is.

**Describe the solution you'd like**
A clear and concise description of what you want to happen.

**Describe alternatives you've considered**
A clear and concise description of any alternative solutions or features you have considered.

**Affected surface**
- [ ] Go daemon (TUI, server, runner, MCP, security)
- [ ] TUI panel
- [ ] HTTP API / MCP server
- [ ] Docs / screenshots

Note: KWatch itself is Go-only. The "TypeScript client / CLI bridge" line that used to be in this checklist was removed when the TS source layer was deleted in `e7eff93` (along with `proc-service.ts` / `proc-cli.ts`); if you want to bring the TS layer back, the checkbox can be re-added once the source is restored. See `AGENTS.md` and `CLAUDE.md` for the full picture.

**AgilePlus spec**
Link the AgilePlus spec that this work implements, or note that a new spec should be opened first (per `CONTRIBUTING.md`).

**Additional context**
Anything else that may help frame the request, including user-facing impact, expected adoption, or links to related work.
