# Zamani per-component Docker images

Honest status as of this writing, so nobody wastes time on an image that
can't build:

| Component     | Image                   | Status | Notes |
|---------------|--------------------------|--------|-------|
| Compiler (ZUTC) | `Dockerfile.compiler`  | ✅ Working | Default cargo features. Lexer→parser→semantic→ir_gen→optimizer→backend + CLI. 196 tests pass. |
| Language Server | `Dockerfile.lsp`       | ✅ Working | `zamani-lsp` — real stdio LSP server. Diagnostics come from the actual lexer/parser, not mocked. Run with `docker run -i`. |
| Build System / Package Manager | `Dockerfile.buildsystem` | ✅ Working | `zamani-pkg` — real `Zamani.toml` TOML parsing + dependency resolution. Registry fetch/publish and multi-target codegen are simulated (clearly logged as such) — no real registry exists yet. |
| Runtime | `Dockerfile.runtime` | 🚧 Blocked | `src/runtime/*` (sankofa memory, quantum runtime, nimbus_os, mts, distributed) is aspirational scaffolding with ~230 real compile errors under `--features full` — mismatched types across files, not a container problem. |
| Standard Library | `Dockerfile.stdlib` | 🚧 Blocked | `src/stdlib/*` (100+ modules) carries the largest share of the remaining `full`-feature compile errors — inconsistent type names across files (`Fact`/`FactObject`/`KnowledgeBase`/`KnowledgeGraph`), a few missing imports/crates. |

## What changed to make LSP/build-system possible

The whole `full` cargo feature (which used to be the *only* way to reach
LSP/runtime/stdlib/build-system code) started at **1438 compile errors** and
didn't build at all. This session:

1. Found and fixed the #1 systemic bug — a `"format string".to_string()`
   pattern inside `println!`/`format!`/etc. macro calls across 65 files,
   which broke Rust's macro parsing. Fixing this mechanically dropped errors
   from 1438 → 433.
2. Fixed the `toolchain` module's aggregator (`init_toolchain`/
   `shutdown_toolchain`) which called function names that didn't match what
   the submodules actually exported.
3. Split `full` into independently-buildable features (`lsp`, `buildsystem`)
   so the genuinely working LSP and build-system code doesn't need the rest
   of the aspirational surface (formal_verification, hyper_ascension, etc.)
   to compile first.
4. Added real entrypoints: `zamani-lsp` (stdio LSP backed by the actual
   parser) and `zamani-pkg` (manifest parsing + dependency resolution),
   neither of which existed before — the LSP/build-system code was
   previously just library modules with no way to run them at all.

Remaining errors (408, all in `runtime`/`stdlib`/a few `compiler/language_spec`
files) are genuine per-file API mismatches — undefined types, inconsistent
naming between modules — that need real design decisions file by file, not
a mechanical fix. That's tracked as follow-up work; see the per-file error
counts by running `cargo build --features full` in the repo root.

## Usage

```bash
# Compiler
docker build -f docker/Dockerfile.compiler -t zamani/compiler .
docker run --rm zamani/compiler zamani --help

# Language server (speaks LSP over stdio — wire into an editor, e.g. Neovim/VSCode)
docker build -f docker/Dockerfile.lsp -t zamani/lsp .
docker run -i --rm zamani/lsp

# Build system / package manager
docker build -f docker/Dockerfile.buildsystem -t zamani/pkg .
docker run --rm -v "$(pwd)":/project zamani/pkg deps --manifest Zamani.toml
```

Or via compose: `docker compose -f docker/docker-compose.yml build compiler lsp buildsystem`.
