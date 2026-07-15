# Contributing to Planify

Thank you for your interest in Planify! This document outlines the development setup, code standards, and contribution process.

## Development Setup

### Prerequisites

- **Node.js** >= 22 (requires native ESM support)
- **pnpm** >= 9 (for `upstream/` Plane workspace) — install via `npm install -g pnpm`
- **Bun** >= 1.2 (for `site/` landing page) — install via `curl -fsSL https://bun.sh/install | bash`
- **Docker** (for infra and Plane API backend)

### Quick Start

```bash
# 1. Clone the repo
git clone git@github.com:KooshaPari/Planify.git
cd Planify

# 2. Install upstream Plane dependencies
cd upstream && pnpm install && cd ..

# 3. Install landing site dependencies
cd site && bun install && cd ..

# 4. Start Plane dev servers
cd upstream && pnpm dev

# 5. In another terminal — start landing site
cd site && bun run dev
```

## Code Standards

### TypeScript

- **Strict mode** is enabled across all Plane packages
- All files must be fully typed — avoid `any` unless absolutely necessary and document why
- Use `interface` over `type` for object shapes; use `type` for unions, intersections, and primitives
- Use `camelCase` for variables, functions, and file names (unless the file exports a React component — then PascalCase)
- Use `PascalCase` for components, types, interfaces, and enums
- Use `UPPER_SNAKE_CASE` for constants

### Formatting

- **Prettier** is used across the Plane workspace — run `pnpm fix:format` before committing
- 100 character line width, 2-space indentation, semicolons required
- Run `pnpm check` to validate formatting and linting before opening a PR

### Linting

- Plane uses **OxLint** — run `pnpm check:lint` to catch issues
- The upstream `upstream/` directory uses the shared `.oxlintrc.json` at its root

### Naming Conventions

| Element | Convention | Example |
|---------|------------|---------|
| Files (non-component) | `camelCase.ts` | `apiClient.ts` |
| Files (component) | `PascalCase.tsx` | `IssueCard.tsx` |
| Functions | `camelCase` | `getProjectIssues()` |
| Components | `PascalCase` | `ProjectDashboard` |
| Interfaces | `PascalCase` | `ProjectConfig` |
| Types | `PascalCase` | `IssueStatus` |
| Constants | `UPPER_SNAKE` | `MAX_RETRY_COUNT` |

### React / State

- Use **MobX** for state management (Plane convention)
- Components should be functional with hooks — no class components
- Build reusable components in `@plane/ui` with Storybook for isolated development

## PR Process

1. **Branch**: Create a feature branch from `main`
   ```
   git checkout main && git pull
   git checkout -b feat/my-feature
   ```

2. **Commits**: Use conventional commits (see `AGENTS.md`). Keep commits small and scoped.

3. **Before opening a PR**:
   - Run `cd upstream && pnpm check` to verify formatting, linting, and types
   - If you added features, include tests
   - Write or update docs as needed
   - Rebase on latest `main`: `git rebase main`

4. **PR Title**: Use conventional commit format, e.g. `feat(web): add issue bulk-edit`

5. **PR Description**: Include:
   - What changed and why
   - Screenshots for UI changes
   - Testing instructions
   - Related issues (if any)

6. **Review expectations**:
   - At least one approval required
   - All CI checks must pass
   - Address review feedback promptly
   - Squash commits on merge

## Testing Requirements

- **Unit tests**: All new features and bug fixes should include unit tests
- **Existing tests**: Run `pnpm test` inside the relevant package to ensure no regressions
- **E2E tests**: For UI changes touching core workflows, run the upstream Playwright suite if available
- **Landing site**: Test with `cd site && bun run build && bun run preview`

## Upstream Sync Policy

The `upstream/` directory is a verbatim seed of `makeplane/plane@preview` (v1.3.1).

- **Do not modify** files inside `upstream/` — all customizations must land outside it
- When upstream Plane releases a new version, a sync PR will be prepared:
  1. Fetch the new upstream tag
  2. Merge into `upstream/`
  3. Resolve conflicts in a controlled PR
  4. Test all Phenotype customizations against the new upstream code
- If you must patch an upstream file (rare), document it in `PATCHES.md` with the rationale and include a plan to upstream the fix

## Code of Conduct

This project follows the [Contributor Covenant](https://www.contributor-covenant.org/) code of conduct. See `upstream/CODE_OF_CONDUCT.md`.
