# Security Policy

## Supported Versions

| Version | Supported          |
|---------|--------------------|
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

If you discover a security vulnerability in the Zamani compiler, please **do not** open a public GitHub issue.

Instead, report it privately by:

1. Emailing the maintainers via the GitHub private vulnerability reporting feature:
   https://github.com/Benwellonedge28/Zamani/security/advisories/new

2. Include a description of the issue, steps to reproduce, and the potential impact.

We aim to respond within **72 hours** and will coordinate a fix and disclosure timeline with you.

## Security Scanning

This repository runs automated security checks on every push:
- `cargo audit` — checks for known vulnerabilities in dependencies
- `cargo deny` — enforces license and advisory policies
- Trivy container scan — checks the Docker image for CVEs (CRITICAL/HIGH)
- GitHub CodeQL (via SARIF upload) for static analysis

## Dependency Policy

- All dependencies are pinned via `Cargo.lock`
- The `deny.toml` file enforces no-banned crates and license compliance
- Dependabot is configured to automatically open PRs for dependency updates
