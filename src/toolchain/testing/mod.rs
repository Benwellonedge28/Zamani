
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Toolchain — Testing Infrastructure
//!
//! Path: `src/toolchain/testing/mod.rs`
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//! Safety: Safe Rust only.
//!
//! ## Mission
//!
//! Compose the reusable testing and conformance infrastructure for the
//! Zamani toolchain.
//!
//! This module supports verification of the toolchain's:
//!
//! - Public API contracts.
//! - Project and workspace integration.
//! - Manifest integration.
//! - Compiler and backend integration.
//! - Build lifecycle.
//! - Determinism.
//! - Reproducibility.
//! - Cross-target portability.
//! - Resource-aware scalability.
//! - Security behavior.
//! - Regression guarantees.
//!
//! These tests support the Zamani portability objective:
//!
//! `Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever`
//!
//! ## Ownership
//!
//! This module owns only:
//!
//! - Testing subsystem composition.
//! - Public testing-module declarations.
//! - Testing-module visibility.
//! - Test-only regression-module gating.
//!
//! Individual testing modules own their respective test contracts,
//! evidence, fixtures, assertions and verification procedures.
//!
//! This module does not own:
//!
//! - A second test execution framework.
//! - A compiler or compilation pipeline.
//! - A backend or code-generation implementation.
//! - A hardware simulator or resource allocator.
//! - A runtime or distributed scheduler.
//! - A second manifest parser.
//! - A dependency resolver.
//! - A cryptographic implementation.
//! - A second security-policy authority.
//! - A global test registry or mutable singleton.
//!
//! ## Canonical repository authorities
//!
//! Language grammar:
//!     `grammar/`
//!
//! Normative POCO-REAF contract:
//!     `grammar/specification/poco-reaf.md`
//!
//! Scalability contract:
//!     `grammar/specification/scalability-model.md`
//!
//! Semantic boundaries:
//!     `grammar/validation/semantic-boundaries.md`
//!
//! Grammar conformance:
//!     `grammar/CONFORMANCE.md`
//!
//! Grammar ownership:
//!     `grammar/OWNERSHIP.yaml`
//!
//! Project configuration:
//!     `crate::zamani_project_config`
//!
//! Compiler:
//!     `crate::compiler`
//!
//! Backend:
//!     `crate::backend`
//!
//! Hardware abstraction:
//!     `crate::zhal`
//!
//! Runtime:
//!     `crate::zur`
//!
//! Toolchain security:
//!     `crate::toolchain::security`
//!
//! Toolchain errors:
//!     `crate::toolchain::errors`
//!
//! Toolchain results:
//!     `crate::toolchain::result`
//!
//! ## Testing-module responsibilities
//!
//! - `fixtures`: isolated project, manifest, dependency and package fixtures.
//! - `conformance`: evidence-based evaluation of normative contracts.
//! - `integration`: actual compiler-service and toolchain-build integration.
//! - `determinism`: repeated-build determinism verification.
//! - `reproducibility`: independent build-observation comparison.
//! - `portability`: cross-target semantic and artifact conformance.
//! - `scalability`: resource-aware workload measurement and reporting.
//! - `security`: security validation, rejection and failure-propagation tests.
//! - `regression`: test-only regression coverage.
//!
//! ## Integration boundaries
//!
//! ```text
//!                 Zamani grammar and specifications
//!                              |
//!                              v
//!                    Testing infrastructure
//!                              |
//!             +----------------+----------------+
//!             |                |                |
//!             v                v                v
//!          Fixtures        Conformance       Security
//!             |                |                |
//!             +----------------+----------------+
//!                              |
//!                              v
//!                       Integration tests
//!                              |
//!                              v
//!                     Toolchain build API
//!                              |
//!                              v
//!                         Compiler
//!                              |
//!                              v
//!                    Backend and artifacts
//!                              |
//!                              v
//!                  Evidence and verification
//!                              |
//!             +----------------+----------------+
//!             |                |                |
//!             v                v                v
//!        Determinism      Reproducibility    Portability
//!             |                |                |
//!             +----------------+----------------+
//!                              |
//!                              v
//!                         Scalability
//!                              |
//!                              v
//!                         Regression
//! ```
//!
//! ## Scalability
//!
//! This composition module introduces no fixed limits on:
//!
//! - Test cases.
//! - Fixtures.
//! - Projects.
//! - Dependencies.
//! - Build tasks.
//! - Targets.
//! - Providers.
//! - Computing domains.
//! - Hardware resources.
//! - Quantum resources.
//! - Tensor dimensions.
//! - Distributed participants.
//!
//! Individual test operations remain subject to their configured
//! operational budgets and the resources actually available.
//!
//! This module does not promise physically infinite execution.
//!
//! ## Security
//!
//! - Unsafe Rust is prohibited.
//! - Test modules must use canonical security authorities.
//! - Synthetic providers must remain test-scoped.
//! - Untrusted test inputs must be validated by their owning subsystem.
//! - Missing or unresolved evidence must never be interpreted as success.
//! - Operational failures must remain distinguishable from rejected
//!   conformance or security results.
//! - Test infrastructure must not weaken production authorization.
//!
//! ## Public API policy
//!
//! The child modules are exposed as named modules.
//!
//! Their public types remain owned by their respective files.
//!
//! This module intentionally avoids wildcard re-exports so that:
//!
//! - Public names remain unambiguous.
//! - Ownership remains visible.
//! - Future modules can be added without introducing name collisions.
//! - Consumers can identify the authority of every imported type.
//!
//! ## Regression policy
//!
//! Regression tests are compiled only when the crate is built with
//! test configuration.
//!
//! Reusable testing and verification infrastructure remains available
//! to build-system consumers through the `buildsystem` feature.

// ============================================================================
// FIXTURES
// ============================================================================

/// Reusable, isolated fixtures for projects, manifests, dependencies
/// and package source trees.
///
/// Uses the canonical Zamani project-configuration parser and the
/// toolchain manifest adapter.
pub mod fixtures;

// ============================================================================
// NORMATIVE CONFORMANCE
// ============================================================================

/// Evidence-based conformance verification against Zamani's
/// architectural, language, compiler, security and operational contracts.
///
/// Does not replace the canonical grammar or semantic authorities.
pub mod conformance;

// ============================================================================
// REAL COMPILER AND TOOLCHAIN INTEGRATION
// ============================================================================

/// Integration harnesses for the actual compiler service and
/// public toolchain build-command boundary.
///
/// Does not simulate successful compilation or implement a second
/// compiler or backend.
pub mod integration;

// ============================================================================
// DETERMINISM AND REPRODUCIBILITY
// ============================================================================

/// Repeated-build determinism verification.
///
/// Consumes canonical build-input and artifact fingerprints.
pub mod determinism;

/// Reproducibility evidence collection and comparison.
///
/// Reuses canonical fingerprint, compiler, backend, dependency,
/// environment and artifact authorities.
pub mod reproducibility;

// ============================================================================
// PORTABILITY AND SCALABILITY
// ============================================================================

/// POCO-REAF portability conformance across independently assessed
/// targets and their verified realizations.
///
/// Does not implement target discovery or semantic-equivalence algorithms.
pub mod portability;

/// Resource-aware scalability measurement and reporting.
///
/// Reuses the canonical toolchain budget tracker and workload adapters.
///
/// The existing repository stores this module in the extensionless
/// `testing/scalability` file. The explicit path preserves that actual
/// repository path without requiring a second implementation or an
/// unrelated file move.
#[path = "scalability"]
pub mod scalability;

// ============================================================================
// SECURITY
// ============================================================================

/// Security conformance, fail-closed behavior, rejection,
/// unresolved-evidence and operational-failure tests.
///
/// Reuses the canonical toolchain, compiler and backend security
/// authorities. It does not implement cryptography or authorization.
pub mod security;

// ============================================================================
// REGRESSION TESTS
// ============================================================================

/// Test-only regression coverage for established toolchain contracts.
///
/// The source file also declares `#![cfg(test)]`, keeping its contents
/// unavailable in non-test builds.
#[cfg(test)]
pub mod regression;
