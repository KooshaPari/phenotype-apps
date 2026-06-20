# 2026 Router Architecture Research — 2026-06-20

## Stack

| Layer | Local Component | Upstream | Pin | Latest |
|-------|-----------------|----------|-----|--------|
| LLM Gateway | bifrost-extensions (KooshaPari) | github.com/maximhq/bifrost (Go) | v1.2.30 | v1.5.21 |
| LLM Proxy (Go) | cliproxyapi-plusplus | github.com/qifengz/cliproxyapi-plusplus | main | main |
| Router | Tokn (Rust, hexagonal) | none (proprietary) | n/a | n/a |
| LLM Gateway Wrappers | Argis (Go, fenix) | none (proprietary) | n/a | n/a |

## Bifrost Gap (v1.2.30 → v1.5.21)

3 minor versions behind. Estimated 30-50 commits worth of upstream features.

## 2026 Alternatives

- **LiteLLM** (BerriAI): 26.5k stars, 100+ providers. Python proxy.
- **Portkey**: 6k stars, AI gateway.
- **OpenRouter**: closed-source SaaS.
- **9router**: not on GitHub.
- **cliproxyapi-plusplus** (local fork): Go CLI proxy for Anthropic/OpenAI/etc.

## Decision

Stay with **bifrost-extensions** as the canonical LLM gateway layer. Bump to v1.5.21 in next wave. Continue using **Tokn** as canonical Rust routing substrate (per ADR-001).

