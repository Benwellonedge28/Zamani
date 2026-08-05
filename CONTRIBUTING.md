# Contributing to Zamani

Thank you for your interest in contributing to the Zamani Universal Meta-Compiler!

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/<your-username>/Zamani`
3. Install Rust: https://rustup.rs
4. Build: `cargo build`
5. Run tests: `cargo test`

## Development Workflow

1. Create a branch: `git checkout -b feat/your-feature`
2. Make your changes
3. Ensure formatting: `cargo fmt --all`
4. Ensure no lint errors: `cargo clippy --all-targets -- -D warnings`
5. Run the full test suite: `cargo test --all`
6. Commit and push
7. Open a pull request against `main`

## Code Standards

- All code must pass `cargo fmt` and `cargo clippy`
- New features must include tests
- Unsafe code requires a `// SAFETY:` comment explaining why it is safe
- Commit messages follow Conventional Commits: `feat:`, `fix:`, `test:`, `docs:`, `ci:`

## Compiler Pipeline

The ZUTC pipeline follows this order:

```
Source → Lexer → Parser → Semantic Analyser → IR Generator → Optimizer → Backend
```

Each stage lives in its own module under `src/`.

## Running Specific Tests

```bash
cargo test --test lexer_tests
cargo test --test parser_tests
cargo test --test semantic_tests
cargo test --test ir_gen_tests
cargo test --test optimizer_tests
cargo test --test backend_tests
cargo test --test compiler_pipeline
```

## Reporting Bugs

Open an issue with:
- A minimal reproducing example (`.zn` source if possible)
- The expected vs actual output
- Your OS and Rust version (`rustc --version`)

## Code of Conduct

Be respectful, constructive, and welcoming. We're building something ambitious — let's do it together.
