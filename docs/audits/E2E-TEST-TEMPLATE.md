# E2E Test Template (Playwright)

> **Source audit:** `FLEET-AUDIT-REPORT.md` — T3 (E2E tests) is the #4 P0 (priority 30, 10/11 audited repos at score 0).
> **Method:** Add a single Playwright E2E test + config + CI workflow.
> **How to use:** Copy the 3 files below to your repo, customize the test, commit, push. Lifts T3 from 0 to 2 (wired).

## What is T3?

T3 (E2E tests) = automated tests that exercise the full user journey end-to-end (browser, API, real data). For web apps, Playwright is the standard. For CLIs/services, use shell-based E2E.

**The "wired" (T3=2) signal**: at least one E2E test that runs in CI on every PR. The test should cover a critical user path (login → main flow → assertion).

## Template files

### 1. `playwright.config.ts`

```ts
import { defineConfig, devices } from '@playwright/test';

export default defineConfig({
  testDir: './tests/e2e',
  timeout: 30_000,
  use: {
    baseURL: process.env.E2E_BASE_URL || 'http://localhost:3000',
    trace: 'on-first-retry',
  },
  projects: [
    {
      name: 'chromium',
      use: { ...devices['Desktop Chrome'] },
    },
  ],
  webServer: {
    command: 'npm start',
    port: 3000,
    reuseExistingServer: !process.env.CI,
    timeout: 60_000,
  },
});
```

### 2. `tests/e2e/smoke.spec.ts`

```ts
import { test, expect } from '@playwright/test';

test('homepage renders and has expected title', async ({ page }) => {
  await page.goto('/');
  await expect(page).toHaveTitle(/.+/);  // any non-empty title
});

test('health-check returns OK', async ({ request }) => {
  const res = await request.get('/health');
  expect(res.status()).toBe(200);
});
```

### 3. `.github/workflows/e2e.yml`

```yaml
name: E2E Tests

on:
  pull_request:
    branches: [main]
  push:
    branches: [main]
  workflow_dispatch:

permissions:
  contents: read

concurrency:
  group: ${{ github.workflow }}-${{ github.ref }}
  cancel-in-progress: true

jobs:
  e2e:
    name: Playwright E2E
    runs-on: ubuntu-24.04
    steps:
      - name: Checkout
        uses: actions/checkout@<PINNED-SHA>  # e.g., de0fac2e4500dabe0009e67214ff5f5447ce83dd

      - name: Setup Node
        uses: actions/setup-node@<PINNED-SHA>  # e.g., 1a4442cacd436585916f4bd0495db9b8a8a0d4d8
        with:
          node-version: 20
          cache: 'npm'

      - name: Install
        run: npm ci

      - name: Install Playwright
        run: npx playwright install --with-deps chromium

      - name: Run E2E tests
        run: npx playwright test

      - name: Upload report on failure
        if: failure()
        uses: actions/upload-artifact@<PINNED-SHA>  # e.g., 65c4c4a1ddee5b72f698fdd0de8ccd5686b70862
        with:
          name: playwright-report
          path: playwright-report/
          retention-days: 7
```

## How to apply

1. **Identify your stack.** This template assumes Node/npm + a web app on port 3000. For:
   - **VitePress / static sites**: change `webServer.command` to `npm run preview -- --port 3000`
   - **Python (Flask/FastAPI)**: skip this template; use pytest-playwright instead
   - **Rust web (axum/actix)**: skip this template; use cargo-test + reqwest
   - **CLI tools (no UI)**: T3 N/A; document that and skip
2. **Copy the 3 files** to your repo (`playwright.config.ts`, `tests/e2e/smoke.spec.ts`, `.github/workflows/e2e.yml`).
3. **Add Playwright to devDeps** in `package.json`:
   ```bash
   npm install -D @playwright/test
   ```
4. **Customize `smoke.spec.ts`** to test 1-2 critical user paths. The "homepage renders" + "health-check" tests are minimum viable; add more as you go.
5. **Pin SHAs** for all `uses:` actions. Use these known-good SHAs (verified):
   - `actions/checkout@de0fac2e4500dabe0009e67214ff5f5447ce83dd`
   - `actions/setup-node@1a4442cacd436585916f4bd0495db9b8a8a0d4d8` (v4)
   - `actions/upload-artifact@65c4c4a1ddee5b72f698fdd0de8ccd5686b70862` (v4.6.0)
6. **Commit + push + open a PR.**

## T3 score lift

- **0 → 1 (ad-hoc):** files exist but not exercised in CI yet.
- **0 → 2 (wired):** workflow runs on every PR; at least one E2E test passes against the running app.
- **0 → 3 (measured):** test count ratchet; coverage report shows the E2E suite growing over time; flake rate < 1%.

## Reference: OmniRoute

OmniRoute is the reference repo for T3 (T3=2). It has 7605 test files including 50+ E2E specs in `tests/e2e/` using Playwright + `@axe-core/playwright` for accessibility. The template above is a minimal extraction of that pattern.

## How to validate

After applying:
1. `npm install` + `npx playwright install --with-deps chromium`
2. `npm start` (in one terminal)
3. `npx playwright test` (in another terminal) — should pass
4. `git push` — CI should run the e2e workflow and pass

## Provenance

- **Template version:** 1.0
- **Author:** Phenotype Org holistic audit, 2026-06-16
- **Audit that produced it:** `FLEET-AUDIT-30-PILLAR.md` (T3 P0)
- **Reference repo:** `KooshaPari/OmniRoute` (T3=2)
- **License:** Same as the parent repo
