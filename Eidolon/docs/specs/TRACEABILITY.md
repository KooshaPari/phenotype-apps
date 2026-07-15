# Traceability Matrix — Eidolon Phase 3

Maps each Functional Requirement to its source files and test coverage.

| FR ID | Title | Source File(s) | Test File(s) | Test Functions |
|---|---|---|---|---|
| FR-EIDOLON-001 | Unified Trait-Based API | `crates/eidolon-core/src/traits.rs` | `crates/eidolon-sandbox/tests/test_sandbox.rs`, `crates/eidolon-desktop/tests/test_desktop.rs`, `crates/eidolon-mobile/tests/test_mobile.rs` | `sandbox_client_is_send_sync`, `full_lifecycle`, `multiple_clients_independent` |
| FR-EIDOLON-002 | Viewport Resolution & Orientation | `crates/eidolon-core/src/viewport.rs` | `crates/eidolon-core/tests/test_core.rs`, `crates/eidolon-core/tests/test_fr_phase3.rs` | `viewport_new_landscape`, `viewport_new_portrait`, `viewport_new_square`, `viewport_desktop_fhd`, `viewport_mobile_fhd`, `viewport_tablet_qhd`, `viewport_fr002_orientation_boundary` |
| FR-EIDOLON-003 | Pointer & Text Input Serialisation | `crates/eidolon-core/src/input.rs` | `crates/eidolon-core/tests/test_core.rs`, `crates/eidolon-core/tests/test_fr_phase3.rs` | `pointer_input_serialize`, `pointer_input_deserialize`, `text_input_serialize`, `text_input_deserialize`, `fr003_pointer_round_trip`, `fr003_text_round_trip` |
| FR-EIDOLON-004 | Automation Event Audit Log | `crates/eidolon-core/src/event.rs` | `crates/eidolon-core/tests/test_core.rs`, `crates/eidolon-core/tests/test_fr_phase3.rs` | `automation_event_pointer`, `automation_event_text`, `automation_event_screenshot`, `automation_event_unique_ids`, `fr004_event_unique_ids_bulk` |
| FR-EIDOLON-005 | Sandbox Lifecycle Management | `crates/eidolon-sandbox/src/lib.rs`, `crates/eidolon-sandbox/src/docker/mod.rs` | `crates/eidolon-sandbox/tests/test_sandbox.rs` | `start_idempotent`, `stop_idempotent`, `start_stop_sequence`, `exec_returns_output`, `full_lifecycle` |
| FR-EIDOLON-006 | Cross-Platform Send+Sync Safety | `crates/eidolon-core/src/traits.rs` | `crates/eidolon-sandbox/tests/test_sandbox.rs`, `crates/eidolon-core/tests/test_fr_phase3.rs` | `sandbox_client_is_send_sync`, `fr006_send_sync_viewport`, `fr006_send_sync_pointer_input` |
| FR-EIDOLON-007 | Version Constant Exposure | `crates/eidolon-core/src/lib.rs` | `crates/eidolon-core/tests/test_core.rs`, `crates/eidolon-core/tests/test_fr_phase3.rs` | `version_defined`, `fr007_version_nonempty` |

---

## Coverage Notes

- `test_fr_phase3.rs` is the Phase 3 annotated test file added in this PR.
  Every test in that file carries a `// Traces to: FR-EIDOLON-NNN` comment
  per the requirement in `FUNCTIONAL_REQUIREMENTS.md`.
- Existing tests in `test_core.rs` and `test_sandbox.rs` provide baseline
  coverage; Phase 3 tests augment coverage for boundary conditions and
  FR-specific acceptance criteria.
- `FUNCTIONAL_REQUIREMENTS.md` (repo root) and `docs/specs/FR.md` (this
  Phase 3 spec) are companion documents. `FR.md` adds rationale and
  acceptance criteria columns missing from the root doc.
