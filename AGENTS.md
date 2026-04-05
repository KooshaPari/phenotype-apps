# AGENTS.md — kwality

## Project Overview

- **Name**: kwality (Quality Assurance Platform)
- **Description**: Comprehensive quality assurance platform with automated testing, code review, and metrics
- **Location**: `/Users/kooshapari/CodeProjects/Phenotype/repos/kwality`
- **Language Stack**: TypeScript, Node.js 20+, Python 3.12+
- **Published**: Private (Phenotype org)

## Quick Start

```bash
# Navigate to project
cd /Users/kooshapari/CodeProjects/Phenotype/repos/kwality

# Install dependencies
npm install

# Start development
npm run dev
```

## Architecture

### Quality Platform

```
┌─────────────────────────────────────────────────────────────────┐
│                     Analysis Engine                              │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐   │
│  │   Code Quality  │  │   Test Results  │  │   Security      │   │
│  │   (SonarQube)   │  │   (Jest/etc)    │  │   (Snyk)        │   │
│  └────────┬────────┘  └────────┬────────┘  └────────┬────────┘   │
└───────────┼───────────────────┼───────────────────┼──────────────┘
            │                   │                   │
            └───────────────────┼───────────────────┘
                                │
┌───────────────────────────────▼───────────────────────────────┐
│                     Metrics Dashboard                              │
│  ┌──────────────────────────────────────────────────────────┐ │
│  │                    Quality Score                              │ │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐  │ │
│  │  │ Coverage │  │ Bugs     │  │ Debt     │  │ Grade    │  │ │
│  │  │ Trend    │  │ Trend    │  │ Ratio    │  │ History  │  │ │
│  │  └──────────┘  └──────────┘  └──────────┘  └──────────┘  │ │
│  └──────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

## Quality Standards

### Code Quality

- **Formatter**: Prettier
- **Linter**: ESLint
- **Tests**: Jest >80%
- **Coverage**: 80% minimum

## Git Workflow

### Branch Naming

Format: `<type>/<area>/<description>`

Examples:
- `feat/metrics/add-code-coverage-trend`
- `fix/reports/handle-large-repos`
- `integration/add-sonarqube-connector`

## CLI Commands

```bash
npm run dev
npm run build
npm test
```

## Resources

- [SonarQube](https://www.sonarqube.org/)
- [Phenotype Registry](https://github.com/KooshaPari/phenotype-registry)

## Agent Notes

**Critical Details:**
- Aggregate multiple quality tools
- Track trends over time
- Set quality gates
- Automated PR comments
