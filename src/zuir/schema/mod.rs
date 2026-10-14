#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Schema Module
//!
//! Canonical path: `src/zuir/schema/mod.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: Rust 1.97 or later.
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Define the public module-composition boundary for ZUIR schema contracts.
//!
//! This module brings together the schema-level type vocabulary, attribute
//! contracts, metadata policies, and extension manifests. It exposes their
//! existing implementations through one stable namespace without duplicating
//! their types or validation logic.
//!
//! Schema descriptors define the structure and declared constraints of
//! representation data. They are not a replacement for the resolved semantic
//! type system, the concrete ZUIR graph, the verifier, or target-specific
//! representation and allocation.
//!
//! ## STATUS
//!
//! Proposed production module boundary. Mark frozen only after the repository
//! compiles with this file and the schema integration checks pass.
//!
//! ## AUTHORITY
//!
//! This module is subordinate to:
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/schema/types.rs`
//! - `src/zuir/schema/attributes.rs`
//! - `src/zuir/schema/metadata.rs`
//! - `src/zuir/schema/extensions.rs`
//! - `grammar/DESIGN.md`
//! - `grammar/specification/`
//! - `grammar/spec/`
//! - `grammar/Zamani.g4`
//! - `grammar/antlr/ZamaniLexer.g4`
//! - `grammar/antlr/ZamaniParser.g4`
//! - `grammar/grammar.md`, as an implementation-conformance reference
//!
//! If an existing repository-wide authority rule conflicts with this module,
//! resolve the conflict in the authoritative specification. Do not establish
//! a second source of truth here.
//!
//! ## OWNS
//!
//! - Public composition of the schema submodules.
//! - Deliberate re-exports of stable, commonly used schema vocabulary.
//! - The schema module's dependency and visibility boundary.
//!
//! ## DOES_NOT_OWN
//!
//! - Definitions already owned by the child modules.
//! - Source syntax, lexical tokens, parser productions, or AST nodes.
//! - Name resolution or semantic type inference.
//! - Concrete IR operations, regions, blocks, or control flow.
//! - Dialect registration, extension trust, namespace authorization,
//!   dependency resolution, or executable plugin loading.
//! - Complete-program serialization, canonical hashing, or migrations.
//! - Physical hardware layout, allocation, scheduling, or target feasibility.
//! - Diagnostic rendering or backend execution.
//!
//! ## DEPENDENCY CONTRACT
//!
//! `types` is the foundational schema vocabulary.
//!
//! `attributes` may depend on `types` for qualified names and value schemas.
//!
//! `metadata` may depend on `attributes` and `types` for typed metadata
//! definitions, values, and validation policies.
//!
//! `extensions` may depend on `types` for qualified extension-owned names.
//! Extension manifests remain declarative; they do not authorize execution.
//!
//! This module must not introduce circular dependencies between these
//! submodules, import concrete IR nodes, or depend on dialect registries.
//!
//! ## GRAMMAR AND SEMANTIC INTEGRATION
//!
//! The canonical Zamani grammar remains responsible for source syntax.
//! Grammar changes must follow `grammar/DESIGN.md`, the normative language
//! specifications, and the established lexer/parser composition.
//!
//! Schema declarations do not add source-language keywords or syntax.
//! Semantic analysis resolves source meaning before that meaning is
//! represented in ZUIR. Schema validation checks schema contracts; it must
//! not reimplement parsing, source name resolution, or semantic analysis.
//!
//! OpenQASM, HDL, classical, quantum, and future domain frontends must use
//! their established frontend and semantic boundaries. This module must not
//! create a competing domain IR or bypass the canonical `quantum::ir`
//! boundary.
//!
//! ## PUBLIC API
//!
//! Child modules are public so callers can use the complete, explicitly
//! namespaced APIs:
//!
//! - `schema::types`
//! - `schema::attributes`
//! - `schema::metadata`
//! - `schema::extensions`
//!
//! Only foundational types with stable ownership are re-exported at this
//! level. Other types remain available from their owning child module.
//!
//! ## SCALABILITY AND RESOURCE POLICY
//!
//! This module imposes no universal limit on the number of schema entries,
//! extensions, operations, devices, qubits, processors, register widths,
//! tensor dimensions, or network nodes.
//!
//! Child modules use dynamically sized representations where appropriate.
//! Actual memory, address-space, and execution limits are determined by the
//! host environment and by explicit resource policies at integration
//! boundaries. No finite implementation can guarantee infinite resources.
//!
//! Untrusted serialized input must be decoded under explicit resource
//! budgets and validated before being accepted as trusted schema data.
//! This module does not itself decode untrusted input or establish trust.
//!
//! ## DETERMINISM
//!
//! Deterministic ordering and canonical representation are governed by the
//! owning child modules and the serialization/fingerprinting contracts.
//! Merely re-exporting a type does not make its serialized representation
//! canonical.
//!
//! ## ERROR CONTRACT
//!
//! Each child module owns its schema-specific validation errors.
//! Higher-level ZUIR APIs may translate those errors into canonical ZUIR
//! errors and diagnostics at the appropriate integration boundary.
//! This module must not duplicate error definitions or discard error context.
//!
//! ## COMPATIBILITY
//!
//! Public names and serialized schemas must follow the versioning and
//! compatibility rules in `src/zuir/ZUIR-SPEC.md` and the relevant schema
//! contracts. Additive public API changes must not silently alter existing
//! semantics. Breaking changes require an explicit compatibility decision
//! and any required migration.
//!
//! ## FREEZE CRITERIA
//!
//! - Every declared module exists and is included exactly once.
//! - Re-exports refer only to public types owned by their source modules.
//! - `cargo check --all-targets` succeeds.
//! - Existing schema and repository tests continue to pass.
//! - The grammar and semantic ownership boundaries remain intact.
//! - No unsafe code, duplicate schema definitions, or fixed hardware limits
//!   are introduced.
//! - Downstream code can use the child modules without requiring this file
//!   to be redesigned when a new schema extension is registered.

/// Schema attribute names, values, definitions, and validation policies.
pub mod attributes;

/// Declarative extension manifests, dependencies, and contribution contracts.
pub mod extensions;

/// Typed metadata definitions, classification, and transformation policies.
pub mod metadata;

/// Schema-level names, primitive descriptors, and compositional schema types.
pub mod types;

/// Common schema vocabulary re-exported from its authoritative definitions.
///
/// The definitions remain owned by `types`; this module introduces no
/// alternate type identities or validation implementations.
pub use types::{
    IntegerSignedness,
    IntegerWidth,
    QualifiedName,
    SchemaType,
    SchemaTypeError,
};