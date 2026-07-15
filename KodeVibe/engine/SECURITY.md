# Security Policy

## Supported Versions

We release patches for security vulnerabilities. Which versions are eligible for receiving such patches depends on the CVSS v3.0 Rating:

| Version | Supported          |
| ------- | ------------------ |
| 1.x.x   | :white_check_mark: |
| < 1.0   | :x:                |

## Reporting a Vulnerability

The KodeVibe team and community take security bugs seriously. We appreciate your efforts to responsibly disclose your findings, and will make every effort to acknowledge your contributions.

### How to Report a Security Vulnerability?

If you believe you have found a security vulnerability in KodeVibe, please report it to us through coordinated disclosure.

**Please do not report security vulnerabilities through public GitHub issues, discussions, or pull requests.**

Instead, please send an email to security@kodevibe.dev with the following information:

- Type of issue (e.g. buffer overflow, SQL injection, cross-site scripting, etc.)
- Full paths of source file(s) related to the manifestation of the issue
- The location of the affected source code (tag/branch/commit or direct URL)
- Any special configuration required to reproduce the issue
- Step-by-step instructions to reproduce the issue
- Proof-of-concept or exploit code (if possible)
- Impact of the issue, including how an attacker might exploit the issue

This information will help us triage your report more quickly.

### What to Expect

After you submit a report, we will:

1. **Acknowledge receipt** of your vulnerability report within 48 hours
2. **Confirm the problem** and determine the affected versions within 5 business days
3. **Audit code** to find any potential similar problems
4. **Prepare fixes** for all supported releases
5. **Release** the fixes and publicly disclose the vulnerability

## Security Update Process

1. Security updates will be released as soon as possible after a vulnerability is confirmed
2. We will coordinate the disclosure timeline with you
3. We will credit you in the security advisory unless you prefer to remain anonymous
4. We will publish a security advisory on GitHub

## Security Best Practices

When using KodeVibe, please follow these security best practices:

### For Users
- Always use the latest version of KodeVibe
- Validate and sanitize any configuration files before use
- Run KodeVibe with the minimum necessary permissions
- Review scan results carefully before applying auto-fixes
- Keep your Go installation and dependencies up to date

### For Contributors
- All code must pass security scans (gosec, govulncheck)
- Follow secure coding practices
- Validate all input data
- Use parameterized queries for any database operations
- Sanitize output data appropriately
- Handle errors securely without leaking sensitive information

## Security Features

KodeVibe includes several built-in security features:

### Input Validation
- All configuration files are validated against strict schemas
- File paths are sanitized to prevent directory traversal attacks
- User input is validated and sanitized

### Output Security
- Scan results are sanitized to prevent injection attacks
- Sensitive information is masked in logs and reports
- File permissions are preserved during auto-fix operations

### Network Security
- HTTPS is enforced for all network communications
- API endpoints implement proper authentication and authorization
- Rate limiting is applied to prevent abuse

### Data Protection
- No sensitive data is stored permanently
- Temporary files are securely cleaned up
- Configuration data is encrypted when stored

## Vulnerability Disclosure Timeline

- **Day 0**: Vulnerability reported
- **Day 1-2**: Initial response and acknowledgment
- **Day 3-7**: Vulnerability confirmed and impact assessed
- **Day 8-30**: Fix developed and tested
- **Day 31-45**: Fix released and coordinated disclosure
- **Day 46+**: Public disclosure (if not already disclosed)

## Hall of Fame

We would like to thank the following individuals for their responsible disclosure of security vulnerabilities:

_No vulnerabilities have been reported yet._

## Contact

For any security-related questions or concerns, please contact:

- **Email**: security@kodevibe.dev
- **PGP Key**: [Available upon request]

## Additional Resources

- [OWASP Top 10](https://owasp.org/www-project-top-ten/)
- [Go Security Best Practices](https://golang.org/doc/security)
- [GitHub Security Advisories](https://docs.github.com/en/code-security/security-advisories)

---

*This security policy was last updated on: July 2025*