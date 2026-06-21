# pheno-pydantic-models — SPEC

## Scope

Canonical Pydantic v2 models for the pheno-* fleet. Mirrors the domain model
in the sibling `pheno-zod-schemas` TypeScript package so Python and
TypeScript services validate the same wire contract.

## Public API

- `class User(BaseModel)` — identity record
  - `id: UUID4`
  - `email: EmailStr`
  - `display_name: str` (1..80 chars)
  - `created_at: AwareDatetime`
- `class WorklogEntry(BaseModel)` — per-task execution record
  - `task_id: str`
  - `status: WorklogStatus` (enum of 6: Pending, InProgress, Blocked, Done,
    Failed, Cancelled)
  - `agent_id: str`
  - `commit_sha: str | None`
  - `started_at: AwareDatetime`
  - `completed_at: AwareDatetime | None`
  - `files_changed: list[str]`
- `class Project(BaseModel)` — project aggregate
  - `id: str` (slug)
  - `name: str` (1..120 chars)
  - `owner_email: EmailStr`
  - `members: list[EmailStr]` — owner must be a member (model validator)

## Model config

All three models use:

```python
model_config = ConfigDict(extra="forbid", frozen=True)
```

- `extra="forbid"` — unknown fields rejected at validation time.
- `frozen=True` — immutable post-construction; instances are hashable.

## Wire codes & invariants

Must match `pheno-zod-schemas` for round-trip interoperability:

| Field         | Type             | Invariant                          |
|---------------|------------------|------------------------------------|
| User.id       | UUID4            | unique per fleet                   |
| User.email    | EmailStr         | lowercase only                     |
| Project.id    | slug             | `^[a-z0-9-]+$`                     |
| Project.owner | EmailStr         | must appear in `members`           |
| Worklog.status| enum             | transitions Pending → InProgress → (Done / Failed / Cancelled) |

## Conventions

- **When to use:** any new pheno-* Python service that handles the canonical
  domain entities.
- **When NOT to use:** ad-hoc DTOs in one-off services (use plain Pydantic).
- **5-line quickstart:**
  ```python
  from pheno_pydantic_models import User, WorklogEntry, Project
  u = User(id="...", email="a@b.co", display_name="A", created_at="2026-06-18T00:00:00Z")
  ```

## Quality bar

- 71-pillar score: 24/71 (Tier 0)
- Test matrix: 5 schema tests (`tests/test_schemas.py`)
- License: dual (MIT + Apache-2.0)

## See also

- pheno-zod-schemas (TypeScript mirror)
- ADR-039 (pheno-flake template)