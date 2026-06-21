"""Quickstart — pheno-pydantic-models.

Demonstrates building a Project aggregate from User + WorklogEntry records.
Run: `python -m pheno_pydantic_models.examples.quickstart` (or `python examples/quickstart.py`).
"""

from datetime import datetime, timezone

from pheno_pydantic_models import Project, User, WorklogEntry, WorklogStatus


def main() -> None:
    alice = User(
        id="11111111-1111-4111-8111-111111111111",
        email="alice@pari.dev",
        display_name="Alice",
        created_at=datetime.now(timezone.utc),
    )

    bob = User(
        id="22222222-2222-4222-8222-222222222222",
        email="bob@pari.dev",
        display_name="Bob",
        created_at=datetime.now(timezone.utc),
    )

    task = WorklogEntry(
        task_id="L8-1",
        status=WorklogStatus.Done,
        agent_id="forge",
        commit_sha="abcdef1234567890",
        started_at=datetime(2026, 6, 18, tzinfo=timezone.utc),
        completed_at=datetime(2026, 6, 18, 0, 30, tzinfo=timezone.utc),
        files_changed=["pheno-pydantic-models/SPEC.md"],
    )

    project = Project(
        id="phenotype-fleet",
        name="Phenotype Fleet",
        owner_email="alice@pari.dev",
        members=["alice@pari.dev", "bob@pari.dev"],
    )

    print(f"owner: {alice.email}")
    print(f"project: {project.name} (slug={project.id})")
    print(f"task: {task.task_id} status={task.status.value} sha={task.commit_sha[:7]}")


if __name__ == "__main__":
    main()