#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Universal Intermediate Representation — Dialect System
//!
//! Canonical path: `src/zuir/dialect/mod.rs`
//! Rust edition: 2021
//! Minimum supported Rust version: 1.97
//! Safety: Safe Rust only; `unsafe` is forbidden.
//!
//! ## PURPOSE
//!
//! Provide the canonical public module boundary for ZUIR dialects.
//!
//! This module composes the existing dialect implementation into one
//! stable namespace. It does not duplicate dialect definitions, operation
//! schemas, interface contracts, versioning rules, or registry behavior.
//!
//! ## STATUS
//!
//! Proposed module-composition implementation.
//! Freeze only after the parent ZUIR module is valid Rust and the complete
//! repository compiles and passes its relevant tests.
//!
//! ## AUTHORITY
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/version.rs`
//! - `src/zuir/schema/types.rs`
//! - `src/zuir/schema/attributes.rs`
//! - `src/zuir/dialect/definition.rs`
//! - `src/zuir/dialect/operation.rs`
//! - `src/zuir/dialect/interfaces.rs`
//! - `src/zuir/dialect/traits.rs`
//! - `src/zuir/dialect/versioning.rs`
//! - `src/zuir/dialect/registry.rs`
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/lexer/tokens.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/extensibility.md`
//! - `grammar/specification/compatibility.md`
//! - `grammar/specification/poco-reaf.md`
//!
//! ## OWNS
//!
//! - Dialect module composition.
//! - Public exports for stable dialect APIs.
//! - The boundary between dialect declarations, operation schemas,
//!   interfaces, versioning, and registry functionality.
//!
//! ## DOES_NOT_OWN
//!
//! - Dialect manifest field definitions.
//! - Operation schema validation algorithms.
//! - Concrete IR operations, values, regions, or blocks.
//! - Whole-program verification.
//! - Source-language parsing, keywords, grammar, or AST construction.
//! - Semantic analysis or type inference.
//! - Extension-code loading, sandboxing, or authorization.
//! - Physical device discovery, resource allocation, scheduling,
//!   routing, or execution.
//!
//! ## DEPENDENCY CONTRACT
//!
//! `definition` owns declarative dialect manifests.
//! `operation` owns declarative operation schemas.
//! `interfaces` owns interface declarations and their contracts.
//! `traits` owns reusable dialect-facing abstractions.
//! `versioning` owns dialect compatibility evaluation and version plans.
//! `registry` owns registration, namespace resolution, and registry policy.
//!
//! This module may re-export those public APIs but must not implement
//! their responsibilities or introduce circular dependencies.
//!
//! ## GRAMMAR INTEGRATION
//!
//! This module does not add source syntax or lexer keywords.
//! Dialect-qualified operation names remain schema-level identifiers.
//! Source syntax and parsing remain governed by the canonical Zamani
//! grammar and its established lexer/parser composition.
//!
//! Adding a dialect must not require adding every operation name to the
//! universal lexer, root grammar, or a closed Rust operation enum.
//!
//! ## SCALABILITY AND POCO-REAF
//!
//! This module imposes no fixed limit on dialects, operations, interfaces,
//! types, computational domains, qubits, processors, devices, nodes,
//! tensor dimensions, register widths, or other hardware resources.
//!
//! Actual resource use is bounded by available memory, execution time,
//! configured safety budgets, implementation capabilities, and physical
//! constraints. Literal infinite capacity is not promised.
//!
//! Unknown declarations remain inert until explicitly resolved and
//! validated. Registration does not imply that an operation is executable
//! or that a particular target can implement it.
//!
//! ## FREEZE CRITERIA
//!
//! - Every declared child module exists exactly once.
//! - Re-exports refer to existing public items.
//! - The parent `src/zuir/mod.rs` correctly declares this module.
//! - The repository compiles with the supported Rust baseline.
//! - Relevant existing tests pass.
//! - No duplicate type definitions, unsafe code, or hardware ceilings
//!   are introduced.
//! - Adding a new dialect does not require changing this file unless
//!   a deliberately approved public API export is needed.

/// Declarative dialect manifests, names, exports, dependencies, and trust
/// requirements.
pub mod definition;

/// Versioned interfaces that dialects and operations may require or provide.
pub mod interfaces;

/// Declarative operation schemas, operand/result groups, region contracts,
/// effects, cardinalities, and terminator requirements.
pub mod operation;

/// Dialect registration, namespace uniqueness, dependency resolution,
/// deterministic lookup, and registry policy.
pub mod registry;

/// Reusable dialect declaration traits and read-only contracts.
pub mod traits;

/// Dialect compatibility evaluation, version requirements, and migration
/// planning contracts.
pub mod versioning;

// -----------------------------------------------------------------------------
// Stable public API
// -----------------------------------------------------------------------------
//
// Each item below is re-exported from its authoritative module. These exports
// provide convenient imports without creating alternate definitions or
// validation implementations.
//
// Keep this list deliberate: internal implementation details should remain
// accessible through their owning modules, not automatically promoted to
// the top-level dialect namespace.

// Dialect manifests and declarative exports.
pub use self::definition::{
    DialectCompatibility,
    DialectDefinition,
    DialectDefinitionError,
    DialectDependency,
    DialectExport,
    DialectExportName,
    DialectImplementation,
    DialectName,
    DialectTrust,
    ExportKind,
    VersionRequirement,
};

// Interface declarations and interface-provider contracts.
pub use self::interfaces::{
    validate_implementation,
    ImplementsInterface,
    InterfaceDefinition,
    InterfaceError,
    InterfaceRequirement,
    InterfaceVersionRequirement,
};

// Open-ended operation schemas.
pub use self::operation::{
    Cardinality,
    EffectDeclaration,
    OperandGroup,
    OperationDefinition,
    OperationSchemaError,
    RegionGroup,
    ResultGroup,
    TerminatorContract,
};

// Registry and resolved registry entries.
pub use self::registry::{
    DialectRegistry,
    ExportReference,
    RegisteredDialect,
    RegistryError,
    RegistryPolicy,
};

// Reusable declaration and inspection traits.
pub use self::traits::{
    CanonicalIdentity,
    DependencyDeclaration,
    DialectManifest,
    ExportDeclaration,
    HasDialectVersion,
    HasTrustRequirement,
    LoweringRequirements,
    ValidateContract,
};

// Version compatibility and migration planning.
pub use self::versioning::{
    evaluate_definition_compatibility,
    evaluate_dialect_compatibility,
    DialectCompatibilityReport,
    RequirementDecision,
    VersionAction,
    VersionPlan,
    VersionStatus,
    VersioningError,
    VersioningPolicy,
};