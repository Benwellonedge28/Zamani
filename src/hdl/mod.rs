
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani HDL Computing Domain
//!
//! Path: `src/hdl/mod.rs`
//! Edition: Rust 2021.
//! Minimum supported Rust version: Rust 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! This module is the composition root for Zamani's hardware-description
//! language (HDL) domain and hardware/software co-design facilities.
//!
//! It registers the existing HDL subsystems and establishes their ownership
//! boundaries. It does not implement parsing, source-level semantic analysis,
//! a competing AST, a second semantic model, or a duplicate intermediate
//! representation.
//!
//! ## Normative authority
//!
//! HDL implementation and integration must conform to the applicable
//! repository authorities:
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/Zamani.g4`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/antlr/ZamaniLexer.g4`
//! - `grammar/lexer/tokens.g4`
//! - The applicable feature grammars under `grammar/hdl/`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/validation/semantic-boundaries.md`
//! - `src/hdl/hdlir/HDLIR-SPEC.md`
//! - `src/zuir/ZUIR-SPEC.md`, where present and authoritative
//!
//! The grammar owns syntax. The canonical frontend owns parsing, AST
//! construction, source locations, and source-level semantic analysis.
//! ZUIR owns the universal computational representation. HDLIR owns the
//! hardware-domain intermediate representation.
//!
//! An implementation must not silently override these authorities.
//!
//! ## Architectural ownership
//!
//! This composition root registers existing HDL modules. Their child
//! implementations retain ownership of their respective algorithms, types,
//! invariants, errors, and tests.
//!
//! - `api` exposes the existing canonical HDL parser and semantic API.
//! - `ids` provides HDL-specific identity integration.
//! - `design` owns logical design declarations and hierarchy.
//! - `types` and `values` provide HDL-specific type and value facilities.
//! - `logic` owns logical and behavioral HDL constructs.
//! - `memory` owns logical memory contracts.
//! - `timing` owns logical timing and clock-domain contracts.
//! - `analysis` owns target-independent HDL analyses.
//! - `verification` owns HDL verification contracts.
//! - `targets` owns target descriptions, capabilities, and feasibility
//!   assessment.
//! - `realization` owns realization plans and provider-facing contracts.
//! - `simulation` owns simulation request and execution interfaces.
//! - `interoperability` owns HDL import/export integration.
//! - `libraries` owns HDL library manifests and registry integration.
//! - `testing` owns shared HDL test fixtures and conformance facilities.
//! - `hdlir` owns all hardware-domain IR implementation.
//!
//! The root-level support modules provide HDL configuration, diagnostics,
//! errors, expressions, requirements, resource integration, provenance,
//! and width-resolution facilities.
//!
//! ## Canonical frontend integration
//!
//! The existing canonical frontend remains authoritative for parsing and
//! source-level semantic validation. HDL modules must consume its actual
//! public APIs and must not recreate those facilities.
//!
//! The intended integration boundary includes:
//!
//! - `crate::frontend::parser::domains::hdl`
//! - `crate::frontend::semantic::domains::hdl`
//! - `crate::frontend::ast::node`
//! - `crate::frontend::semantic::model`
//!
//! These paths must exist and be publicly reachable from the crate root
//! before the corresponding HDL integration can be considered compiled.
//! This file deliberately does not fabricate missing frontend modules or
//! substitute guessed APIs.
//!
//! ## IR ownership and dependency direction
//!
//! The intended compilation architecture is:
//!
//! ```text
//! Zamani grammar and specifications
//!                 |
//!                 v
//! canonical lexer and parser
//!                 |
//!                 v
//! canonical AST and semantic model
//!                 |
//!                 v
//!              ZUIR
//!                 |
//!                 v
//!       src/hdl/hdlir/
//!                 |
//!                 v
//! hardware-domain verification and optimization
//!                 |
//!                 v
//! downstream lowering and realization
//!                 |
//!                 v
//! target-specific hardware or simulation
//! ```
//!
//! This diagram specifies architectural ownership and intended data flow;
//! it does not claim that every stage is implemented or operational.
//!
//! All HDLIR-owned structures, operations, builders, verification,
//! analyses, transformations, serialization, and lowering must remain
//! within `src/hdl/hdlir/`. No second HDL IR may be introduced elsewhere
//! under `src/hdl/`.
//!
//! HDLIR must not replace ZUIR's universal representation or the canonical
//! frontend semantic model. Conversion between representations must preserve
//! supported semantics and report unsupported or lossy mappings explicitly.
//!
//! ## POCO-REAF and scalability
//!
//! Zamani's objective is:
//!
//! `Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever`
//!
//! HDL source must express portable hardware intent independently of
//! unnecessary physical-target assumptions.
//!
//! This module introduces no universal hardware-capacity ceilings. It must
//! not impose fixed maxima on widths, ports, signals, modules, instances,
//! memories, clock domains, pipeline stages, devices, or other hardware
//! resources.
//!
//! Symbolic quantities remain symbolic until the appropriate owning stage
//! can resolve them. Concrete arithmetic and conversions must be checked.
//! Operational budgets must be explicit and must not redefine language
//! semantics or universal hardware capacity.
//!
//! Exhausted budgets, unsupported capabilities, incomplete analyses, and
//! target infeasibility must remain distinguishable from successful
//! validation. A target that cannot realize a valid design must not cause
//! silent semantic degradation.
//!
//! No finite implementation can process literally infinite input. The
//! scalability objective is to avoid arbitrary language-level ceilings and
//! scale with available resources, representable quantities, supported
//! semantics, and explicitly configured operational budgets.
//!
//! ## Safety and untrusted input
//!
//! Unsafe Rust is forbidden throughout this module and its descendants.
//!
//! External HDL, serialized IR, imported designs, target descriptions, and
//! extension metadata must be treated as potentially malformed. The owning
//! subsystem must validate inputs, check arithmetic, apply relevant
//! operational budgets, and return structured failures.
//!
//! ## Module registration policy
//!
//! A child module may be registered here only when its source and required
//! module tree exist and its integration dependencies have been reviewed.
//! A directory's existence alone does not prove that its implementation
//! compiles or is production-ready.
//!
//! Child modules must not depend on this composition root for foundational
//! types when doing so would create a dependency cycle. Shared contracts
//! belong in their designated owning modules.
//!
//! Public APIs must be deliberate. This root avoids wildcard re-exports so
//! that ownership remains explicit and conflicting names are not concealed.
//!
//! ## Compatibility and determinism
//!
//! API evolution must be based on verified consumers, explicit contracts,
//! and tests. Persistent HDLIR artifacts must follow the HDLIR format and
//! compatibility policies owned by `hdlir`.
//!
//! Deterministic operations must not depend on unspecified iteration order,
//! filesystem traversal order, hardware discovery order, or incidental
//! memory addresses.
//!
//! ## Production qualification
//!
//! Registration does not imply implementation completeness or production
//! readiness. The integrated repository must be checked using its supported
//! Rust toolchain and applicable feature configurations.
//!
//! Required baseline checks:
//!
//! - `cargo fmt --all -- --check`
//! - `cargo check --all-targets`
//! - `cargo test --all-targets`
//! - `cargo clippy --all-targets -- -D warnings`
//!
//! These checks must be executed against the actual checkout. This file
//! makes no claim that they have passed.

/// Stable identifier for the Zamani HDL computing domain.
pub const HDL_DOMAIN_NAME: &str = "zamani.hdl";

/// Version of this composition-root API contract.
///
/// This is independent of the Zamani language, grammar, frontend semantic
/// model, crate, and HDLIR artifact versions.
pub const HDL_DOMAIN_API_VERSION: u16 = 1;

// -----------------------------------------------------------------------------
// Core HDL API and shared contracts
// -----------------------------------------------------------------------------

/// Canonical HDL parser and semantic-validation facade.
///
/// This module re-exports the existing frontend contracts; it does not
/// implement another parser or semantic analyzer.
pub mod api;

/// HDL-facing capability integration.
pub mod capabilities;

/// Explicit HDL analysis configuration and operational budgets.
pub mod config;

/// HDL behavioral-contract representation.
pub mod contract;

/// HDL-specific diagnostic vocabulary and helpers.
pub mod diagnostics;

/// Shared typed HDL subsystem errors.
pub mod error;

/// HDL expression integration facilities.
pub mod expressions;

/// HDL-specific identity integration.
pub mod ids;

/// HDL provenance conventions and adapters.
pub mod provenance;

/// HDL requirements, constraints, preferences, and hints.
pub mod requirements;

/// HDL resource-model integration.
pub mod resources;

/// Checked HDL width-resolution facilities.
pub mod widths;

// -----------------------------------------------------------------------------
// Logical hardware model and domain services
// -----------------------------------------------------------------------------

/// Target-independent logical design declarations and hierarchy.
pub mod design;

/// HDL logical and behavioral operations.
pub mod logic;

/// Logical memory declarations and access semantics.
pub mod memory;

/// HDL logical types and type-layout facilities.
pub mod types;

/// HDL values, constants, and symbolic values.
pub mod values;

/// Logical clocks, timing constraints, and clock-domain analysis.
pub mod timing;

/// Target-independent HDL analyses.
pub mod analysis;

/// HDL verification contracts and verification result models.
pub mod verification;

// -----------------------------------------------------------------------------
// Target integration and execution boundaries
// -----------------------------------------------------------------------------

/// HDL target descriptions, capabilities, constraints, and feasibility.
pub mod targets;

/// Hardware realization plans and provider-facing contracts.
pub mod realization;

/// HDL simulation interfaces and execution contracts.
pub mod simulation;

/// HDL library manifests and registry integration.
pub mod libraries;

/// HDL format import/export and interoperability.
pub mod interoperability;

/// HDL test fixtures and conformance facilities.
pub mod testing;

// -----------------------------------------------------------------------------
// Canonical HDL intermediate representation
// -----------------------------------------------------------------------------

/// Hardware-domain intermediate representation.
///
/// All HDLIR-owned types, builders, verification, analysis, transformations,
/// dialects, serialization, interchange, and lowering must be implemented
/// within `src/hdl/hdlir/`. This parent module only registers the boundary.
pub mod hdlir;

#[cfg(test)]
mod tests {
    use super::{HDL_DOMAIN_API_VERSION, HDL_DOMAIN_NAME};

    #[test]
    fn domain_identity_is_stable_and_nonempty() {
        assert!(!HDL_DOMAIN_NAME.is_empty());
        assert_eq!(HDL_DOMAIN_NAME, "zamani.hdl");
    }

    #[test]
    fn composition_api_version_is_nonzero() {
        assert_ne!(HDL_DOMAIN_API_VERSION, 0);
    }
}
