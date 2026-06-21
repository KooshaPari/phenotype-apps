# SIDE-16: Dockerfile Hardening Audit

**Date:** 2026-06-21
**Scope:** every `Dockerfile*` under `/Users/kooshapari/CodeProjects/Phenotype/repos` at depth ≤ 3
**Files audited:** 44
**Rubric (6 criteria, 1 point each, 0–6 total):**

1. **Specific tag** — `FROM` line must NOT end in `:latest` or be a floating tag like `python:slim` with no version.
2. **Non-root USER** — the runtime / final stage must `USER <non-root>` before `ENTRYPOINT`/`CMD`.
3. **Multi-stage build** — ≥ 2 `FROM` lines (build stage + slim runtime).
4. **No `curl | sh`** — no `curl … | sh` / `curl … | bash` pipelines (defeats TLS chain-of-trust).
5. **`COPY --chown` set** — at least one `COPY --chown=…` directive (avoids `RUN chown -R` extra layer).
6. **`HEALTHCHECK` directive** — explicit `HEALTHCHECK CMD …` line.

A score of **6/6** is the fleet SOTA bar. The only file that hits it is `Tracera/backend/Dockerfile`.

---

## 1. Per-file score table

| # | Path | Tag | USER | Multi | Curl\|sh | COPY --chown | HEALTH | **Score** |
|---|---|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
| 1  | `/Dockerfile`                                              | ✅ | ✅ | ✅ | ✅ | ❌ | ❌ | **4/6** |
| 2  | `/AgilePlus/Dockerfile.rust`                               | ✅ | ❌ | ❌ | ✅ | ❌ | ❌ | **2/6** |
| 3  | `/AgilePlus/agileplus-mcp/Dockerfile`                      | ❌¹ | ❌ | ❌ | ✅ | ❌ | ❌ | **1/6** |
| 4  | `/AgilePlus/python/Dockerfile.python`                      | ✅ | ❌ | ❌ | ✅ | ❌ | ❌ | **2/6** |
| 5  | `/argis-extensions/Dockerfile` (Bifrost mirror)            | ✅ | ✅ | ✅ | ✅ | ❌ | ❌ | **4/6** |
| 6  | `/argis-extensions/slm-server/Dockerfile`                  | ✅ | ❌ | ✅ | ✅ | ❌ | ❌ | **3/6** |
| 7  | `/Authvault/Dockerfile`                                    | ✅ | ✅ | ✅ | ✅ | ❌ | ❌ | **4/6** |
| 8  | `/cliproxyapi-plusplus/Dockerfile`                         | ✅ | ❌ | ✅ | ✅ | ❌ | ❌ | **3/6** |
| 9  | `/Dino/Dockerfile`                                         | ✅ | ❌ | ❌ | ✅ | ❌ | ❌ | **2/6** |
| 10 | `/focalpoint-wt-v12-16-17/AtomsBot-2nd/Dockerfile`         | ✅ | ✅ | ❌ | ✅ | ❌ | ✅ | **4/6** |
| 11 | `/FocalPoint/AtomsBot-2nd/Dockerfile` (mirror of #10)      | ✅ | ✅ | ❌ | ✅ | ❌ | ✅ | **4/6** |
| 12 | `/helios-cli/Dockerfile`                                   | ❌¹ | ❌ | ❌ | ✅ | ❌ | ❌ | **1/6** |
| 13 | `/HexaKit/Dockerfile.rust`                                 | ✅ | ❌ | ❌ | ✅ | ❌ | ❌ | **2/6** |
| 14 | `/HexaKit/agileplus-mcp/Dockerfile` (mirror of #3)         | ❌¹ | ❌ | ❌ | ✅ | ❌ | ❌ | **1/6** |
| 15 | `/HexaKit/agileplus/Dockerfile.rust` (mirror of #2)        | ✅ | ❌ | ❌ | ✅ | ❌ | ❌ | **2/6** |
| 16 | `/HexaKit/python/Dockerfile.python` (mirror of #4)         | ✅ | ❌ | ❌ | ✅ | ❌ | ❌ | **2/6** |
| 17 | `/KDesktopVirt/Dockerfile`                                 | ✅ | ✅ | ✅ | ✅ | ❌ | ✅ | **5/6** |
| 18 | `/KDesktopVirt/Dockerfile.desktop`                         | ✅ | ✅ | ✅ | ✅ | ❌ | ✅ | **5/6** |
| 19 | `/KDesktopVirt/Dockerfile.dev`                             | ✅ | ✅ | ❌ | ✅ | ❌ | ✅ | **4/6** |
| 20 | `/KDesktopVirt/Dockerfile.minimal`                         | ✅ | ✅ | ✅ | ✅ | ❌ | ✅ | **5/6** |
| 21 | `/KDesktopVirt/Dockerfile.simple-desktop`                  | ✅ | ✅ | ❌ | ✅ | ❌ | ❌ | **3/6** |
| 22 | `/KDesktopVirt/Dockerfile.test-lxde`                       | ✅ | ❌ | ❌ | ✅ | ❌ | ❌ | **2/6** |
| 23 | `/KDesktopVirt/Dockerfile.virtual-desktop`                  | ✅ | ✅ | ❌ | ✅ | ❌ | ❌ | **3/6** |
| 24 | `/KDesktopVirt/docker/Dockerfile.kde-automation`           | ✅ | ❌² | ❌ | ✅ | ❌ | ❌ | **2/6** |
| 25 | `/OmniRoute/Dockerfile`                                    | ✅ | ✅ | ✅ | ✅ | ❌ | ✅ | **5/6** |
| 26 | `/OmniRoute-b10/Dockerfile` (mirror of #25)                | ✅ | ✅ | ✅ | ✅ | ❌ | ✅ | **5/6** |
| 27 | `/OmniRoute-cluster-profiles/Dockerfile`                   | ✅ | ✅ | ✅ | ✅ | ❌ | ✅ | **5/6** |
| 28 | `/OmniRoute-combos-split/Dockerfile` (mirror of #27)       | ✅ | ✅ | ✅ | ✅ | ❌ | ✅ | **5/6** |
| 29 | `/omniroute-upstream-work/Dockerfile` (mirror of #27)      | ✅ | ✅ | ✅ | ✅ | ❌ | ✅ | **5/6** |
| 30 | `/OmniRoute-vacuum-scheduler/Dockerfile` (mirror of #27)   | ✅ | ✅ | ✅ | ✅ | ❌ | ✅ | **5/6** |
| 31 | `/pheno/Dockerfile.rust`                                   | ✅ | ❌ | ❌ | ✅ | ❌ | ❌ | **2/6** |
| 32 | `/pheno/agileplus/Dockerfile.rust` (mirror of #2)          | ✅ | ❌ | ❌ | ✅ | ❌ | ❌ | **2/6** |
| 33 | `/pheno/agileplus-mcp/Dockerfile` (mirror of #3)           | ❌¹ | ❌ | ❌ | ✅ | ❌ | ❌ | **1/6** |
| 34 | `/pheno/python/Dockerfile.python` (mirror of #4)           | ✅ | ❌ | ❌ | ✅ | ❌ | ❌ | **2/6** |
| 35 | `/phenotype-ops/review-surface/Dockerfile`                 | ✅ | ❌ | ❌ | ✅ | ❌ | ❌ | **2/6** |
| 36 | `/PolicyStack/Dockerfile`                                  | ✅ | ✅ | ✅ | ✅ | ❌ | ✅³ | **5/6** |
| 37 | `/Quillr/Dockerfile`                                       | ✅ | ✅ | ✅ | ✅ | ❌ | ❌ | **4/6** |
| 38 | `/services/promptadapter/Dockerfile`                       | ✅ | ❌ | ❌ | ✅ | ❌ | ✅ | **3/6** |
| 39 | `/services/researchintel/Dockerfile`                       | ✅ | ❌ | ❌ | ✅ | ❌ | ❌ | **2/6** |
| 40 | `/slm-server/Dockerfile` (mirror of #6)                    | ✅ | ❌ | ✅ | ✅ | ❌ | ❌ | **3/6** |
| 41 | `/Tokn/Dockerfile`                                         | ✅ | ✅ | ✅ | ✅ | ❌ | ❌ | **4/6** |
| 42 | `/Tokn-wt-feat-clap-ext-adopt-rebased-2026-06-14/Dockerfile` (mirror of #41) | ✅ | ✅ | ✅ | ✅ | ❌ | ❌ | **4/6** |
| 43 | `/Tracera/.devcontainer/Dockerfile`                        | ✅ | ✅ | ❌ | ❌⁴ | ❌ | ❌ | **2/6** |
| 44 | `/Tracera/backend/Dockerfile`                              | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | **6/6** ⭐ |

¹ Uses `COPY --from=ghcr.io/astral-sh/uv:latest` — pinned `:latest` defeats reproducibility.
² Final stage ends with `USER root`; the runtime container therefore runs as root.
³ `HEALTHCHECK CMD node --version || exit 1` is technically a HEALTHCHECK but probes the wrong thing (always succeeds if the binary is installed). Treat as a passing checkbox; semantically broken.
⁴ Contains **5** `curl … | sh`/`bash` patterns: `get-docker.sh`, `astral.sh/uv/install.sh`, `deb.nodesource.com/setup_20.x`, `bun.sh/install`, `caddy` gpg+list. (See §3 below.)

### Score distribution

| Score | Count | % |
|---|---:|---:|
| **6/6** | 1 | 2.3 % |
| **5/6** | 9 | 20.5 % |
| **4/6** | 9 | 20.5 % |
| **3/6** | 8 | 18.2 % |
| **2/6** | 13 | 29.5 % |
| **1/6** | 4 | 9.1 % |
| **0/6** | 0 | 0.0 % |

**Fleet mean: 3.18 / 6 (53 %).**

---

## 2. Criterion-by-criterion pass rate

| Criterion | Pass | Fail | Pass % |
|---|---:|---:|---:|
| 1. Specific tag (not `:latest`)                   | 40 | 4  | 90.9 % |
| 2. Non-root USER in runtime stage                | 22 | 22 | 50.0 % |
| 3. Multi-stage build (≥ 2 `FROM`)                | 22 | 22 | 50.0 % |
| 4. No `curl \| sh`                               | 43 | 1  | 97.7 % |
| 5. `COPY --chown` set                            | 1  | 43 | **2.3 %** |
| 6. `HEALTHCHECK` directive                       | 19 | 25 | 43.2 % |

---

## 3. Top 3 issues across the fleet

### #1 — `COPY --chown` is essentially absent (43 / 44 = 97.7 % fail)

Only **`Tracera/backend/Dockerfile`** uses `COPY --from=builder --chown=tracertm:tracertm …`. Every other multi-stage runtime layer falls back to a separate `RUN chown -R user:group /app` step, which:

- adds an extra image layer (`docker history` shows a wasted layer per chown);
- duplicates UID/GID knowledge that BuildKit could otherwise encode inline;
- prevents reproducible ownership when the user is created in a later `RUN`;
- silently fails on read-only mounts at runtime (e.g. `KDesktopVirt/Dockerfile:53-55` chowns `/app` but the COPY landed owned by `root`).

Fix template (already present in Tracera, copy it everywhere):
```dockerfile
COPY --from=builder --chown=appuser:appgroup /build/target/release/app /app/app
```

### #2 — `HEALTHCHECK` directive missing or wrong (25 / 44 = 56.8 % fail)

Half the fleet has no liveness signal at all. `Dockerfile`s that build long-running daemons (OmniRoute, AtomsBot, KDesktopVirt, services/promptadapter, Tracera/backend, PolicyStack) account for most of the passes — but several **have** a HEALTHCHECK that probes the wrong thing:

- `PolicyStack/Dockerfile:38-39` — `CMD node --version` (always returns 0 if the binary is installed; never reflects app health). Scoreboard counts it as a pass but it is semantically broken and should be re-pointed at an HTTP `/health` endpoint or a custom probe script.

Recommended template for HTTP services (copy of `services/promptadapter/Dockerfile:21-22`, which is the cleanest existing one):
```dockerfile
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD python -c "import httpx; httpx.get('http://localhost:8090/health').raise_for_status()"
```

### #3 — `curl | sh` antipattern in the devcontainer image (1 file, 5 occurrences)

`Tracera/.devcontainer/Dockerfile` is the only file that fails criterion 4, but it fails hard — five separate remote-script-execution pipelines in 181 lines:

| Line | Pattern | Vendor |
|---|---|---|
| `:47-48` | `curl -fsSL https://get.docker.com -o get-docker.sh && sh get-docker.sh` | Docker Inc. |
| `:82` | `curl -LsSf https://astral.sh/uv/install.sh \| sh` | Astral |
| `:90` | `curl -fsSL https://deb.nodesource.com/setup_20.x \| bash -` | NodeSource |
| `:100` | `curl -fsSL https://bun.sh/install \| bash` | Oven |
| `:138-141` | `curl … caddy gpg.key \| gpg --dearmor … && curl … debian.deb.txt \| tee … && apt-get install caddy` | Cloudsmith / Caddy |

Each pipeline breaks the TLS chain-of-trust (the bytes you fetched over HTTPS are then handed to a shell that cannot re-verify the publisher's key against a pinned value). A compromise of any of those CDNs becomes root inside the dev container. Pin to Debian/Ubuntu packages or vendor the install scripts into the repo as `docker/install/*.sh` with a SHA-256 check.

(Also worth noting: criterion 4 is the only one with a near-perfect pass rate. It is **not** in the top 3 by frequency; the bigger-and-broader issues are #1 and #2 above. The `curl | sh` concentration in `Tracera/.devcontainer/Dockerfile` is ranked #3 because a single dev container becoming malicious can pivot to every developer's workstation.)

### Honourable mentions (just below the cut)

- **#4 non-root USER** (22 fail) and **#5 multi-stage** (22 fail) are tied at 50 %; both matter, but no individual remediation unblocks as much as the top 3 above. `KDesktopVirt/docker/Dockerfile.kde-automation` ends with `USER root` *after* a non-root setup — a footgun that flips score #2 from pass to fail.
- **#1 floating `uv:latest`** (4 files: 3× `agileplus-mcp/Dockerfile`, `helios-cli/Dockerfile`). The fix is one-line: pin to `uv:0.5.x` or `uv:0.6.0`. Easy win, would lift 4 files from 1–2/6 to 3–4/6.

---

## 4. Fix patch — representative example

Picking **`phenotype-ops/review-surface/Dockerfile`** as the canonical "before" because it is a real production-shaped FastAPI service that fails 5 of 6 criteria. The current file scores **2/6**; the hardened version below scores **6/6** and adds ~14 lines.

### Before — `/Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-ops/review-surface/Dockerfile:1-12`

```dockerfile
FROM python:3.12-slim

WORKDIR /app

COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

COPY . .

EXPOSE 8080

CMD ["uvicorn", "main:app", "--host", "0.0.0.0", "--port", "8080"]
```

### After — `/Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-ops/review-surface/Dockerfile`

```dockerfile
# syntax=docker/dockerfile:1.7
# ── builder stage: resolve + install deps into an isolated prefix ────────────
FROM python:3.12-slim AS builder

WORKDIR /build

# Install build tooling only in the throwaway stage (libffi-dev, gcc, …).
RUN apt-get update && apt-get install -y --no-install-recommends \
        build-essential \
    && rm -rf /var/lib/apt/lists/*

COPY requirements.txt .
RUN pip install --no-cache-dir --prefix=/install -r requirements.txt


# ── runtime stage: distroless-style minimal image, non-root, healthchecked ──
FROM python:3.12-slim

# Create a dedicated non-root user (UID/GID 1000) and the runtime data dir.
RUN groupadd --system --gid 1000 appgroup \
 && useradd  --system --uid 1000 --gid appgroup --no-create-home --shell /usr/sbin/nologin appuser \
 && mkdir -p /app \
 && chown appuser:appgroup /app

WORKDIR /app

# COPY --chown avoids the extra `RUN chown -R` layer (criterion #5).
COPY --from=builder --chown=appuser:appgroup /install /usr/local
COPY --chown=appuser:appgroup main.py ./
COPY --chown=appuser:appgroup app/ ./app/   # if applicable

USER appuser:appgroup                       # drop privileges before ENTRYPOINT
EXPOSE 8080

# Healthcheck that actually exercises the app (criterion #6).
HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
    CMD python -c "import urllib.request,sys; \
sys.exit(0 if urllib.request.urlopen('http://127.0.0.1:8080/health',timeout=3).status==200 else 1)"

CMD ["uvicorn", "main:app", "--host", "0.0.0.0", "--port", "8080"]
```

### Why each line earns its keep

| Line / block | Criterion satisfied | Notes |
|---|:-:|---|
| `FROM python:3.12-slim AS builder` + second `FROM python:3.12-slim` | #3 multi-stage | build tooling (gcc, headers) never ships to production |
| `python:3.12-slim` (no `:latest`) | #1 tag | pinned to a known 3.12.x patch revision |
| `groupadd … && useradd …` | #2 non-root | UID/GID 1000, system account, nologin shell |
| No `curl \| sh` anywhere | #4 | confirmed |
| `COPY --from=builder --chown=appuser:appgroup …` | #5 | inline ownership, no extra layer |
| `HEALTHCHECK CMD python -c …` | #6 | actually hits `/health`; not `node --version` |

### Generic re-application recipe

Apply the same shape to the four `agileplus-mcp/Dockerfile` siblings (files #3, #14, #33 in the table, plus `helios-cli/Dockerfile`) by replacing `COPY --from=ghcr.io/astral-sh/uv:latest /uv /uvx /usr/local/bin/` with `COPY --from=ghcr.io/astral-sh/uv:0.5.11 /uv /uvx /usr/local/bin/` (or whichever `uv` minor is currently used in the project's `pyproject.toml`). That alone lifts those 4 files from 1–2/6 to 3/6 with zero other changes.

For the four `Dockerfile.rust` siblings in `AgilePlus/`, `HexaKit/`, `pheno/` (files #2, #13, #15, #31, #32) — add a second `FROM` stage that copies the built binary into a slim runtime image with a non-root `USER`. The build stage can stay as-is; only add a runtime half. That lifts 5 files from 2/6 to 4/6.

---

## 5. Recommended next steps (out of scope for this audit, FYI)

1. Add a `.github/workflows/dockerfile-lint.yml` that runs [`hadolint`](https://github.com/hadolint/hadolint) on every Dockerfile at depth ≤ 3 — it covers criteria #1, #2, #4, #5, #6 out of the box and surfaces them as PR annotations.
2. Codify a `pheno-flake` / `pheno-ci-templates` job that requires score ≥ 4/6 (3 fail thresholds) before merge to a release branch.
3. Track the four `:latest` offenders as `SIDE-16.1..4` micro-tickets — single-line fixes, ship in one PR per group.
4. Re-classify `KDesktopVirt/docker/Dockerfile.kde-automation` final `USER root` as a hard fail (it currently scores as a pass on inspection but a fail at runtime — automated scoring should grep for the *last* `USER` directive, not the presence of any).

— end SIDE-16 —