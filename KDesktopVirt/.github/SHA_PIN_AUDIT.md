# SHA-Pin Audit — 2026-06-08

Workflow SHA-pin audit flagged this repo as highest-impact.

- 38 non-SHA-pinned action uses
- 3 workflows missing `permissions:` block
- 32 workflows missing `concurrency:` block

Follow-up: pin all `uses: ...@<ref>` to `uses: ...@<sha>`.
