//! Zamani Quantum Frontend — production test-suite orchestrator.
//!
//! This module is the single orchestration boundary for the test modules under
//! `src/quantum/frontend/tests/`.
//!
//! It intentionally contains **no implementation logic** and no duplicated
//! frontend behavior. Its sole responsibilities are:
//!
//! - registering every frontend test module;
//! - keeping the test suite organized by architectural responsibility;
//! - enforcing the dependency/order model of the frontend test suite;
//! - documenting the public-API boundary used by cross-layer tests;
//! - providing one stable test-module entry point from
//!   `crate::quantum::frontend`.
//!
//! # Architectural position
//!
//! ```text
//!                         quantum::frontend
//!                                │
//!                                ▼
//!                     frontend::tests::mod
//!                                │
//!          ┌─────────────────────┼─────────────────────┐
//!          │                     │                     │
//!          ▼                     ▼                     ▼
//!       core tests          generic contract       OpenQASM tests
//!          │                     │                     │
//!          ├─ source             ├─ contracts          ├─ lexer
//!          ├─ limits             ├─ capabilities       ├─ parser
//!          └─ diagnostics        └─ public API         ├─ validation
//!                                                      ├─ import
//!                                                      ├─ export
//!                                                      └─ round-trip
//!
//!                                │
//!                                ▼
//!                     security / robustness
//!                                │
//!                     ┌──────────┴──────────┐
//!                     ▼                     ▼
//!               malformed input      resource exhaustion
//! ```
//!
//! # Test ownership
//!
//! The individual files own their respective test responsibilities.
//!
//! This file must **not**:
//!
//! - reimplement tests from child modules;
//! - contain production frontend logic;
//! - import OpenQASM implementation internals merely to make registration
//!   possible;
//! - construct a second frontend API;
//! - become a central format-specific test dispatcher;
//! - contain `#[test]` functions whose responsibility belongs in a child
//!   module.
//!
//! # Public API boundary
//!
//! Tests which verify integration across frontend layers must use the public
//! API exposed by `crate::quantum::frontend` whenever possible.
//!
//! In particular, cross-layer tests should prefer:
//!
//! ```text
//! crate::quantum::frontend
//! ```
//!
//! rather than reaching directly into:
//!
//! ```text
//! formats::openqasm::lexer
//! formats::openqasm::parser
//! formats::openqasm::ast
//! formats::openqasm::validation
//! ```
//!
//! Format-specific implementation tests are permitted in the dedicated
//! OpenQASM test files when the implementation contract itself is what is
//! being tested.
//!
//! # Test layers
//!
//! The suite is intentionally divided into the following layers.
//!
//! ## Layer 1 — core contracts
//!
//! These establish the behavior of the shared frontend infrastructure:
//!
//! - `source.rs`
//! - `limits.rs`
//! - `diagnostics.rs`
//!
//! ## Layer 2 — generic frontend contracts
//!
//! These establish format-independent contracts:
//!
//! - `format.rs` capabilities and versions;
//! - `contracts.rs` generic API contracts.
//!
//! ## Layer 3 — OpenQASM lexical and syntactic correctness
//!
//! These establish:
//!
//! - `openqasm_lexer.rs`
//! - `openqasm_parser.rs`
//!
//! ## Layer 4 — OpenQASM semantic correctness
//!
//! This establishes:
//!
//! - `openqasm_validation.rs`
//!
//! ## Layer 5 — OpenQASM import/export
//!
//! These establish:
//!
//! - `openqasm_import.rs`
//! - `openqasm_export.rs`
//!
//! ## Layer 6 — semantic round-trip
//!
//! This establishes:
//!
//! - `openqasm_roundtrip.rs`
//!
//! ## Layer 7 — security and robustness
//!
//! These establish:
//!
//! - `malformed_inputs.rs`
//! - `resource_exhaustion.rs`
//!
//! ## Layer 8 — capability contract
//!
//! This establishes:
//!
//! - `capabilities.rs`
//!
//! Capability tests deliberately remain separate from concrete OpenQASM
//! behavior because capabilities belong to the generic format contract.
//!
//! # Production invariants
//!
//! The complete suite registered here is intended to establish the following
//! frontend invariants.
//!
//! ## Invariant 1 — public API correctness
//!
//! The generic frontend can be consumed through its public API without
//! requiring callers to know concrete implementation details.
//!
//! ## Invariant 2 — format isolation
//!
//! OpenQASM tests must not establish a dependency whereby another future format
//! must modify OpenQASM implementation code.
//!
//! ## Invariant 3 — canonical IR authority
//!
//! Successful imports must lower into the canonical Quantum IR rather than a
//! second quantum semantic representation.
//!
//! ## Invariant 4 — source-location consistency
//!
//! Source, lexer, parser, diagnostics and lowering provenance must use the
//! shared frontend source-location contracts.
//!
//! ## Invariant 5 — bounded resources
//!
//! Untrusted input must remain subject to the configured frontend limits.
//!
//! ## Invariant 6 — structured failures
//!
//! Invalid, unsupported, and resource-exhausting input must produce structured
//! frontend failures rather than requiring consumers to parse human-readable
//! error strings.
//!
//! ## Invariant 7 — deterministic behavior
//!
//! Given the same:
//!
//! - source;
//! - configuration;
//! - frontend version;
//! - limits;
//!
//! externally observable frontend behavior must be deterministic.
//!
//! ## Invariant 8 — no implicit side effects
//!
//! Frontend parsing/importing/exporting tests must not require:
//!
//! - filesystem access;
//! - network access;
//! - process execution;
//! - shell execution;
//! - QPU access;
//! - hardware discovery;
//! - calibration execution.
//!
//! ## Invariant 9 — no silent semantic loss
//!
//! A construct must be:
//!
//! ```text
//! supported → validated → lowered
//! ```
//!
//! or:
//!
//! ```text
//! unsupported → structured error
//! ```
//!
//! or:
//!
//! ```text
//! invalid → structured error
//! ```
//!
//! It must never become:
//!
//! ```text
//! parsed → silently discarded
//! ```
//!
//! ## Invariant 10 — deterministic export
//!
//! The same canonical Quantum IR and export options must produce the same
//! external representation.
//!
//! ## Invariant 11 — semantic round-trip
//!
//! For supported semantics:
//!
//! ```text
//! OpenQASM
//!     ↓
//! QuantumCircuit₁
//!     ↓
//! OpenQASM
//!     ↓
//! QuantumCircuit₂
//! ```
//!
//! must preserve the semantics represented by the canonical IR.
//!
//! # Dependency order
//!
//! The test suite follows the same architectural order as the production
//! frontend.
//!
//! ```text
//! source
//!   ↓
//! limits
//!   ↓
//! diagnostics
//!   ↓
//! generic contracts
//!   ↓
//! OpenQASM lexer
//!   ↓
//! OpenQASM parser
//!   ↓
//! OpenQASM validation
//!   ↓
//! OpenQASM importer
//!   ↓
//! OpenQASM exporter
//!   ↓
//! OpenQASM round-trip
//!   ↓
//! malformed input
//!   ↓
//! resource exhaustion
//! ```
//!
//! Rust's test harness does not guarantee execution order between individual
//! `#[test]` functions. Therefore this ordering is an **architectural
//! dependency order**, not an assumption that one test executes before another.
//!
//! Every test must be independently executable.
//!
//! # Test isolation
//!
//! Child test modules must not communicate mutable state between tests.
//!
//! Tests must not rely on:
//!
//! - global mutable registries;
//! - test execution order;
//! - environment-specific filesystem state;
//! - network availability;
//! - external processes;
//! - hardware;
//! - QPU availability;
//! - wall-clock timing except where explicitly testing timing behavior;
//! - random values without a deterministic seed.
//!
//! # Test determinism
//!
//! Where tests compare serialized output, they must compare the complete
//! deterministic representation rather than relying on incidental formatting.
//!
//! Where tests compare diagnostics, they should compare stable error codes,
//! spans, severity, and structured information rather than only human-readable
//! messages.
//!
//! Where tests compare canonical circuits, they should compare canonical IR
//! semantics rather than source formatting.
//!
//! # Rust compatibility
//!
//! This test facade targets:
//!
//! - Rust 2021 edition;
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - stable Rust only;
//! - no nightly features;
//! - no external test dependencies;
//! - no unsafe code.
//!
//! # Module registration
//!
//! All test modules in this directory are registered here.
//!
//! Keeping registration centralized prevents the parent frontend facade from
//! becoming a long list of individual test paths and makes this directory a
//! self-contained test boundary.
//!
//! # Existing test modules
//!
//! The production test suite currently consists of:
//!
//! ```text
//! capabilities.rs
//! contracts.rs
//! diagnostics.rs
//! limits.rs
//! malformed_inputs.rs
//! openqasm_export.rs
//! openqasm_import.rs
//! openqasm_lexer.rs
//! openqasm_parser.rs
//! openqasm_roundtrip.rs
//! openqasm_validation.rs
//! resource_exhaustion.rs
//! source.rs
//! ```
//!
//! The repository currently contains the round-trip file with a trailing
//! whitespace character in its filename:
//!
//! ```text
//! openqasm_roundtrip.rs␠
//! ```
//!
//! Until that repository filename is normalized, the explicit `#[path]` below
//! intentionally references the actual filename. This prevents silently
//! referring to a different, nonexistent file.
//!
//! Once the filename is renamed to the canonical `openqasm_roundtrip.rs`, the
//! corresponding `#[path]` attribute can be removed and the normal Rust module
//! declaration can be used.
//!
//! # Adding a new test module
//!
//! When a new test file is added under this directory:
//!
//! 1. Give it one architectural responsibility.
//! 2. Keep cross-layer tests on the public frontend API.
//! 3. Add exactly one module declaration here.
//! 4. Do not duplicate the same test through another registration path.
//! 5. Keep tests independently executable.
//! 6. Do not introduce production dependencies solely for testing.
//!
//! For example:
//!
//! ```ignore
//! pub mod openqasm_conformance;
//! ```
//!
//! # Integration with `frontend/mod.rs`
//!
//! The parent frontend module should register this entire test facade with:
//!
//! ```ignore
//! #[cfg(test)]
//! mod tests;
//! ```
//!
//! The child test modules themselves do not need to be registered individually
//! from `frontend/mod.rs`.
//!
//! This creates exactly one test registration boundary:
//!
//! ```text
//! frontend/mod.rs
//!       │
//!       └── tests/mod.rs
//!               │
//!               ├── capabilities.rs
//!               ├── contracts.rs
//!               ├── diagnostics.rs
//!               ├── limits.rs
//!               ├── malformed_inputs.rs
//!               ├── openqasm_export.rs
//!               ├── openqasm_import.rs
//!               ├── openqasm_lexer.rs
//!               ├── openqasm_parser.rs
//!               ├── openqasm_roundtrip.rs
//!               ├── openqasm_validation.rs
//!               ├── resource_exhaustion.rs
//!               └── source.rs
//! ```
//!
//! # Important: no production API pollution
//!
//! This module must remain compiled only under `#[cfg(test)]` from the parent
//! frontend module.
//!
//! It must never become part of the normal production binary/library API.
//!
//! The public APIs being tested remain the APIs exported by
//! `crate::quantum::frontend`; this module is only a test harness boundary.
//!
//! # Important: no cross-test orchestration
//!
//! This file does **not** call child test functions.
//!
//! Rust's test harness owns execution of individual `#[test]` functions.
//!
//! The word "orchestrator" here means module orchestration:
//!
//! ```text
//! register modules
//!       ↓
//! Rust test harness discovers tests
//!       ↓
//! tests execute independently
//! ```
//!
//! It does not mean:
//!
//! ```text
//! mod.rs
//!   ↓
//! call test_a()
//!   ↓
//! call test_b()
//!   ↓
//! call test_c()
//! ```
//!
//! The latter would make tests coupled to execution order and would be
//! unsuitable for a production compiler test suite.
//!
//! # Failure policy
//!
//! A test failure must identify the violated architectural contract.
//!
//! Child modules should therefore prefer assertions on:
//!
//! - stable error codes;
//! - structured diagnostic severity;
//! - source spans;
//! - format/version identity;
//! - canonical IR properties;
//! - deterministic exported bytes;
//! - resource-limit classification;
//! - absence of silent semantic loss.
//!
//! Human-readable diagnostic text may additionally be checked where it is
//! itself part of the documented presentation contract.
//!
//! # Security-test boundary
//!
//! `malformed_inputs.rs` and `resource_exhaustion.rs` are intentionally separate
//! from normal semantic tests.
//!
//! This prevents security assumptions from becoming accidental consequences of
//! ordinary happy-path coverage.
//!
//! Security tests must establish that untrusted frontend input cannot cause:
//!
//! - unbounded parsing;
//! - uncontrolled AST expansion;
//! - uncontrolled diagnostic expansion;
//! - uncontrolled export expansion;
//! - recursion exhaustion beyond configured limits;
//! - panics at public boundaries;
//! - implicit external I/O.
//!
//! # Future formats
//!
//! Adding another format, such as QIR or Quil, should normally add dedicated
//! tests such as:
//!
//! ```text
//! qir_import.rs
//! qir_export.rs
//! qir_validation.rs
//! qir_roundtrip.rs
//! ```
//!
//! and register them here.
//!
//! Existing generic contract tests must remain format-independent.
//!
//! This is important because `frontend/mod.rs` explicitly defines concrete
//! formats as independently removable adapters and the generic contracts as
//! their extension point. 
//!
//! # Production completion criterion
//!
//! This module is complete when:
//!
//! - every test file in this directory is registered exactly once;
//! - no test file depends on registration order;
//! - the parent frontend only registers this facade;
//! - generic tests use the generic public API;
//! - format tests remain isolated to their format;
//! - security tests remain side-effect free;
//! - the suite compiles on Rust 1.97/1.97.1;
//! - no nightly feature is required;
//! - no external test dependency is required.
//!
//! This file intentionally contains no executable production logic.

#![cfg(test)]


// =============================================================================
// Layer 1 — shared frontend infrastructure
// =============================================================================

/// Source identity, spans, UTF-8 locations, line/column conversion, and source
/// map contract tests.
pub mod source;

/// Frontend resource-limit and bounded-work contract tests.
pub mod limits;

/// Structured diagnostic and diagnostic-budget contract tests.
pub mod diagnostics;


// =============================================================================
// Layer 2 — generic frontend public contracts
// =============================================================================

/// Generic importer/exporter/lowering/frontend API contract tests.
pub mod contracts;

/// Format identity, version, compatibility, and capability contract tests.
pub mod capabilities;


// =============================================================================
// Layer 3 — OpenQASM lexical and syntactic contracts
// =============================================================================

/// OpenQASM lexical conformance tests.
pub mod openqasm_lexer;

/// OpenQASM grammar/parser conformance tests.
pub mod openqasm_parser;


// =============================================================================
// Layer 4 — OpenQASM semantic contracts
// =============================================================================

/// OpenQASM semantic validation and type/scope/resource tests.
pub mod openqasm_validation;


// =============================================================================
// Layer 5 — OpenQASM import/export contracts
// =============================================================================

/// OpenQASM → canonical Quantum IR import tests.
pub mod openqasm_import;

/// Canonical Quantum IR → OpenQASM export tests.
pub mod openqasm_export;


// =============================================================================
// Layer 6 — semantic round-trip contracts
// =============================================================================

// The repository currently contains this filename with one trailing whitespace
// character. Keep the explicit path until the repository filename is normalized
// to `openqasm_roundtrip.rs`.
#[path = "openqasm_roundtrip.rs "]
pub mod openqasm_roundtrip;


// =============================================================================
// Layer 7 — malformed input and resource-exhaustion security contracts
// =============================================================================

/// Malformed/untrusted-input robustness tests.
pub mod malformed_inputs;

/// Resource exhaustion and frontend-limit enforcement tests.
pub mod resource_exhaustion;


// =============================================================================
// Test-suite architectural boundary
// =============================================================================

/// Compile-time documentation of the complete frontend test architecture.
///
/// This function is deliberately not a `#[test]`. The Rust test harness should
/// discover and execute the individual tests from the child modules rather than
/// having this facade call them manually.
///
/// Keeping this declaration here makes the intended dependency layers explicit
/// to rustdoc and to maintainers without coupling test execution order.
#[allow(dead_code)]
const _TEST_ARCHITECTURE: &str = concat!(
    "frontend test architecture: ",
    "source -> limits -> diagnostics -> generic contracts -> ",
    "OpenQASM lexer -> parser -> validation -> import/export -> ",
    "round-trip -> malformed-input -> resource-exhaustion",
);