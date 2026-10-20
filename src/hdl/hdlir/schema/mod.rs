
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani HDLIR Schema
//!
//! Path: `src/hdl/hdlir/schema/mod.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Define the public composition boundary for HDLIR schema definitions and
//! registration. This module exposes the existing canonical schema registry
//! without duplicating its implementation or introducing another registry.
//!
//! ## Architectural authority
//!
//! This module must conform to:
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/Zamani.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/validation/semantic-boundaries.md`
//! - `src/hdl/ids.rs`
//! - `src/hdl/api.rs`
//! - `src/hdl/hdlir/errors.rs`
//! - `src/hdl/hdlir/config.rs`
//! - `src/hdl/hdlir/version.rs`
//! - `src/hdl/hdlir/HDLIR-SPEC.md`, when established
//! - `src/hdl/hdlir/compatibility.md`, when established
//! - `src/zuir/ZUIR-SPEC.md`
//!
//! The grammar owns source syntax. The canonical frontend owns AST
//! construction and source-level semantic validation. ZUIR owns the
//! universal computational representation. HDLIR owns hardware-domain
//! representation contracts and schema registration.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - The Rust module boundary for HDLIR schema definitions.
//! - The intentional public exports of the schema subsystem.
//! - The separation between schema registration and concrete IR construction.
//!
//! The registry owns schema identities, declarative definitions, extension
//! policy enforcement, registration validation, and immutable snapshots.
//!
//! This module does not own:
//!
//! - Source parsing, AST nodes, or source-level name resolution.
//! - A second semantic model or universal intermediate representation.
//! - Concrete HDLIR graph nodes, builders, or verification algorithms.
//! - Physical hardware allocation, synthesis, placement, routing, or timing
//!   closure.
//! - A closed, exhaustive list of every hardware operation that may exist.
//!
//! ## Public API and dependency direction
//!
//! Consumers should normally import schema contracts from this module,
//! rather than depending on internal implementation details of a particular
//! schema file.
//!
//! The canonical registry is implemented in `registry.rs`. Its definitions
//! are re-exported here so callers can use one stable schema namespace.
//!
//! The registry depends on the HDLIR configuration, error, and version
//! contracts. It must not depend on concrete IR graph nodes, builders,
//! lowering, or physical realization.
//!
//! Specialized operation contracts, type descriptors, and interface
//! contracts may be added in sibling modules after their implementations
//! and public APIs exist. They must integrate with the canonical registry;
//! they must not create parallel registries or conflicting definitions.
//!
//! ## Scalability and determinism
//!
//! This module imposes no universal hardware-capacity limits. It does not
//! restrict the number of registered dialects, operations, types, or
//! interfaces through hard-coded constants.
//!
//! Actual processing remains subject to available memory, representable
//! values, the registry's checked operations, and explicit operational
//! policies. No software implementation can process literally infinite
//! input.
//!
//! Registry ordering, duplicate handling, version compatibility, and
//! extension-policy behavior are defined by `registry.rs`. This module
//! introduces no global mutable state, hidden registration, environmental
//! dependencies, or nondeterministic initialization.
//!
//! ## Integration contract
//!
//! - `src/hdl/hdlir/mod.rs` declares this schema subsystem when integrating
//!   the HDLIR module tree.
//! - `registry.rs` remains the canonical owner of registry storage and
//!   registration behavior.
//! - Builders and verifiers consume schema definitions and snapshots; they
//!   must not mutate registry state as an incidental part of verification.
//! - Serialization must validate schema names, versions, required
//!   definitions, and extension compatibility before accepting an artifact.
//! - Frontend lowering must use actual canonical semantic contracts and
//!   preserve unsupported or unresolved constructs as explicit failures.
//! - Target-specific feasibility and physical implementation remain outside
//!   schema registration.
//!
//! ## Completion criteria
//!
//! - The schema module compiles with the supported Rust toolchain.
//! - Every exported item resolves to the canonical registry implementation.
//! - No duplicate schema registry or competing definition type is introduced.
//! - Public APIs have intentional visibility and documented contracts.
//! - Existing registry tests and repository-wide checks pass.
//!
//! This module only composes existing schema functionality. Its presence does
//! not imply that HDLIR operations, types, builders, or lowering are complete.

/// Canonical definitions and registry for HDLIR schema entities.
///
/// This module is the single authority for dialect, operation, type, and
/// interface registration until specialized contract modules are introduced
/// with an explicit integration into the registry.
pub mod registry;

/// Canonical qualified name for a schema entity.
pub use registry::QualifiedName;

/// Classification of a schema definition's maturity and compatibility.
pub use registry::DefinitionStability;

/// Declarative dialect schema definition.
pub use registry::DialectDefinition;

/// Operand/result count constraints for operation definitions.
pub use registry::Arity;

/// Operand/result signature for an operation definition.
pub use registry::OperationSignature;

/// Declarative operation schema definition.
pub use registry::OperationDefinition;

/// Declarative type schema definition.
pub use registry::TypeDefinition;

/// Declarative operation-interface schema definition.
pub use registry::InterfaceDefinition;

/// Atomic batch of schema definitions to register.
pub use registry::RegistryBatch;

/// Immutable view of the registered schema definitions.
pub use registry::RegistrySnapshot;

/// Canonical mutable schema registry.
pub use registry::SchemaRegistry;
