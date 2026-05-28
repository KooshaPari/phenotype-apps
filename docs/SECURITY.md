# Security Policy

## Reporting a Vulnerability

We take security vulnerabilities seriously. If you discover a security issue, please report it responsibly.

### How to Report

- **Private disclosure**: Please do not report security vulnerabilities through public GitHub issues.
- **Email**: Send a detailed report to the maintainers with:
  - Description of the vulnerability
  - Steps to reproduce
  - Potential impact
  - Any suggested fixes (optional)

### Scope

This policy applies to all code in this repository, including:
- Core application code
- Dependencies (direct and transitive)
- CI/CD workflows
- Configuration files

### Response Timeline

- **Initial response**: We aim to acknowledge reports within 48 hours
- **Status update**: We will provide a progress update within 7 days
- **Resolution**: We will work to address confirmed vulnerabilities as quickly as possible

### Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| main   | :white_check_mark: |
| older  | :x:                |

## Security Best Practices

When contributing code, please follow these security practices:

- Never commit secrets, credentials, or API keys
- Use environment variables for sensitive configuration
- Validate and sanitize all user input
- Follow the principle of least privilege
- Keep dependencies updated
