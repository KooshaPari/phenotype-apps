# Contributing to KodeVibe

First off, thank you for considering contributing to **KodeVibe**! It's people like you who make this code quality tool better for everyone.

## Code of Conduct

By participating in this project, you agree to abide by our Code of Conduct.

## How Can I Contribute?

### Reporting Bugs

- Use the Bug Report issue template
- Provide a clear and descriptive title
- Describe the exact steps to reproduce the problem
- Include your Go version and OS

### Suggesting Enhancements

- Check the Issues to see if the enhancement has already been suggested
- Use the Feature Request issue template
- Describe the use case and why it would benefit the project

### Pull Requests

1. Fork the repo and create your branch from `main`
2. If you've added code that should be tested, add tests
3. If you've changed APIs, update the documentation
4. Ensure the test suite passes (`make test`)
5. Make sure your code lints (`make lint`)
6. Format your code (`make format`)

#### Branch Naming

```
feat/<feature-name>     # New features
fix/<bug-description>   # Bug fixes
docs/<topic>            # Documentation
refactor/<area>        # Code refactoring
```

#### Commit Messages

Follow conventional commits:

```
feat(security): add new secret detection pattern
fix(scanner): correct file exclusion logic
docs(readme): update usage examples
```

## Development Setup

```bash
# Clone the repository
git clone https://github.com/KooshaPari/KodeVibe-Go.git
cd KodeVibe-Go

# Install dependencies
make deps

# Run tests
make test

# Run linter
make lint

# Format code
make format

# Run all checks
make pre-commit
```

## Architecture Guidelines

KodeVibe follows a modular vibe-based architecture:

- **Vibes**: Individual checkers in `pkg/vibes/`
- **Scanner**: Core engine in `pkg/scanner/`
- **Fixer**: Auto-fix engine in `pkg/fix/`
- **Server**: HTTP/API in `pkg/server/`

### Key Principles

1. **Vibe Interface**: All vibes implement the `Checker` interface
2. **Configuration-Driven**: Vibes are configured via YAML
3. **Extensible**: Easy to add new vibes
4. **Testable**: All vibes have unit tests

## Adding New Vibes

1. Create new vibe checker in `pkg/vibes/`
2. Implement the `Checker` interface
3. Register in `registry.go`
4. Add tests and documentation
5. Update configuration schema

## License

By contributing, you agree that your contributions will be licensed under the MIT license.

---

Thank you for your contributions!
