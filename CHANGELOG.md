# Changelog

All notable changes to the Zamani Universal Meta-Compiler (ZUTC) are documented here.

This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Added
- Full module coverage: all 193 .rs source files wired into the build graph
- Standard library integration: stdlib, runtime, toolchain, nimbus, nimbus_os, hdl
- Zamani-native source preservation strategy using `.zn` files + `include_str!`
- Bench harness: `compiler_bench` registered in Cargo.toml
- Fuzz targets: lexer, parser, and full pipeline
- CI/CD pipeline: formatting, linting, multi-platform testing, security audit, Docker
- Docker image: linux/amd64, multi-stage build with runtime layer < 50 MB
- Security audit via `cargo-audit` and Trivy container scanning
- All project standard files: Cargo.lock, LICENSE, .gitignore, CONTRIBUTING.md, deny.toml, rustfmt.toml

### Changed
- Merged duplicate module files (ast, backend, optimizer) into `/mod.rs` structure
- Upgraded Dockerfile base from `rust:1.78` to `rust:slim-bookworm` (latest stable)
- Removed `Cargo.lock` from `.gitignore` for reproducible binary builds
- Dropped multi-arch Docker build (arm64) for CI reliability; amd64-only

### Fixed
- Resolved `TokenType` enum variant duplication in lexer
- Fixed `include_str!` macros to use relative paths
- Corrected `rustfmt.toml` for stable compiler compatibility

---

## [0.1.0] — 2026-06-01

### Added
- Initial compiler pipeline: lexer → parser → semantic → ir_gen → optimizer → backend
- 132 automated tests across all compiler stages
- GitHub Actions CI with cargo fmt, clippy, build, test enforcement
