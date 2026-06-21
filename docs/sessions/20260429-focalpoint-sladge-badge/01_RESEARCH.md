# Research

## Repo Fit

FocalPoint is in scope for the sladge rollout because its README lists
LLM-coached rituals and MCP-bridged connector scaffolding, and its architecture
docs describe an always-on agent pipeline with cheap-LLM integration.

## Local State

Canonical `FocalPoint` had broad unrelated local edits across README, iOS,
Android, docs-site, FFI, audio, connector, generated framework, and status
files. The badge change was prepared in an isolated worktree to avoid mixing
those changes.

## Decision

Treat this as a documentation/governance badge update only. Do not modify Rust
workspace code, mobile app code, generated FFI/framework assets, or docs-site
content.
