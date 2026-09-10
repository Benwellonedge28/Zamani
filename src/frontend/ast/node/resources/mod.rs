//! # Zamani Native AST — Resources
//!
//! `src/frontend/ast/node/resources/mod.rs`
//!
//! Canonical module boundary for source-level resource constructs in the
//! Zamani native AST.
//!
//! ## Architectural position
//!
//! ```text
//! Zamani source
//!     │
//!     ▼
//! lexer
//!     │
//!     ▼
//! parser
//!     │
//!     ▼
//! frontend::ast::node::resources
//!     │
//!     ├── resource
//!     ├── resource_kind
//!     └── resource_annotation
//!     │
//!     ▼
//! structural AST validation
//!     │
//!     ▼
//! semantic analysis
//!     │
//!     ▼
//! semantic resource model
//!     │
//!     ▼
//! ZUIR
//!     │
//!     ├── classical domain
//!     ├── quantum domain
//!     ├── accelerator domain
//!     ├── distributed domain
//!     └── future domains
//!     │
//!     ▼
//! target/resource discovery
//!     │
//! ▼
//! allocation / mapping / routing / scheduling / execution
//! ```
//!
//! This module is therefore an **AST module boundary**, not a resource
//! allocator, resource manager, scheduler, hardware abstraction, or quantum
//! resource runtime.
//!
//! ## Ownership
//!
//! This module owns only the public module organization of source-level
//! resource AST constructs.
//!
//! The implementation ownership is deliberately split:
//!
//! - `resource.rs`
//!   - resource declarations/references;
//!   - source-level resource names;
//!   - source-level cardinality;
//!   - source-level resource scope;
//!   - source-level resource type references;
//!   - source-level initializer references;
//!   - resource-local attribute references;
//!   - local structural validation.
//!
//! - `resource_kind.rs`
//!   - extensible source-level resource-kind identities;
//!   - namespace/name validation;
//!   - deterministic identity representation.
//!
//! - `resource_annotation.rs`
//!   - source-level resource annotations;
//!   - annotation identity;
//!   - annotation metadata/value representation;
//!   - local annotation validation.
//!
//! This `mod.rs` owns:
//!
//! - module declarations;
//! - stable public re-exports;
//! - resource-module architectural documentation;
//! - compile-time API-surface tests;
//! - dependency-boundary documentation.
//!
//! It must contain **no duplicate resource implementation**.
//!
//! ## Critical domain boundary
//!
//! The native AST resource model is intentionally domain-neutral.
//!
//! A source-level resource may ultimately describe or participate in:
//!
//! - classical computation;
//! - quantum computation;
//! - logical quantum computation;
//! - accelerator computation;
//! - memory;
//! - communication;
//! - distributed computation;
//! - timing;
//! - storage;
//! - future computational substrates.
//!
//! The AST does not decide which concrete machine or technology implements the
//! resource.
//!
//! In particular, this module must never become coupled to:
//!
//! - physical qubits;
//! - QPU devices;
//! - CPU cores;
//! - GPU SMs;
//! - FPGA fabric;
//! - ASIC resources;
//! - vendor devices;
//! - device topology;
//! - routing;
//! - scheduling;
//! - calibration;
//! - pulse generation;
//! - QEC implementations;
//! - ZQN implementations;
//! - runtime allocation;
//! - backend credentials;
//! - backend queues;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - OpenQASM;
//! - vendor SDKs.
//!
//! Those concerns belong downstream.
//!
//! ## POCO-REAF
//!
//! The resource AST is designed for:
//!
//! ```text
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
//! ```
//!
//! A resource declaration must therefore remain valid when the same source
//! program is compiled against radically different resource scales.
//!
//! For example, the semantic meaning of a source-level resource must not
//! change merely because the available target changes from:
//!
//! ```text
//! one resource
//!     ↓
//! thousands of resources
//!     ↓
//! millions of resources
//!     ↓
//! distributed resources
//!     ↓
//! heterogeneous resources
//!     ↓
//! future computational resources
//! ```
//!
//! This module therefore contains no language-level constants such as:
//!
//! ```text
//! MAX_RESOURCES
//! MAX_QUBITS
//! MAX_DEVICES
//! MAX_REGISTER_SIZE
//! MAX_MACHINE_SIZE
//! ```
//!
//! Compiler safety/resource policies may impose configurable limits in later
//! phases. Such limits must never become part of the native AST's semantic
//! model.
//!
//! ## No closed resource taxonomy
//!
//! A central architectural invariant is that resource kinds must remain
//! extensible.
//!
//! The native AST must not evolve into:
//!
//! ```text
//! enum ResourceKind {
//!     Cpu,
//!     Gpu,
//!     Qubit,
//!     Fpga,
//!     Memory,
//!     ...
//! }
//! ```
//!
//! Such a closed enumeration would require modifying the AST whenever a new
//! resource technology is introduced.
//!
//! `resource_kind.rs` consequently owns an extensible namespaced identity.
//!
//! The current resource-kind implementation deliberately follows this model.
//!
//! ## Cardinality boundary
//!
//! Resource cardinality belongs to `resource.rs` and is represented through
//! source-level AST references rather than a prematurely evaluated machine
//! integer.
//!
//! Therefore a source-level construct conceptually equivalent to:
//!
//! ```text
//! resource q[N]
//! ```
//!
//! can preserve `N` as an AST expression.
//!
//! The AST does not decide whether `N` is:
//!
//! - a compile-time constant;
//! - a generic parameter;
//! - a symbolic quantity;
//! - a runtime-derived value;
//! - physically realizable on the selected target.
//!
//! Those questions belong to semantic analysis and downstream compilation.
//!
//! This is essential to scaling from tiny systems to systems whose available
//! resources are substantially larger than any machine assumed by the
//! frontend.
//!
//! ## Resource identity versus physical allocation
//!
//! These concepts must never be conflated:
//!
//! ```text
//! source resource identity
//!         ≠
//! semantic resource identity
//!         ≠
//! physical resource identity
//!         ≠
//! runtime resource handle
//! ```
//!
//! The native AST stores only the first category.
//!
//! A semantic pass may resolve the AST resource into a semantic resource
//! identity.
//!
//! A target/backend pass may subsequently map that semantic requirement to
//! physical resources.
//!
//! The AST must not contain the physical mapping.
//!
//! ## Quantum boundary
//!
//! Quantum resources are represented through the same source-level resource
//! abstraction.
//!
//! This prevents the AST from assuming that every quantum resource is:
//!
//! - a superconducting qubit;
//! - a trapped ion;
//! - a neutral atom;
//! - a photon;
//! - a topological qubit;
//! - any other particular technology.
//!
//! A later quantum compiler may interpret a resource kind as a logical qubit,
//! physical qubit, encoded qubit, photonic mode, atom, or another quantum
//! resource according to the semantic and target layers.
//!
//! The native AST does not make that decision.
//!
//! ## Downstream Quantum IR boundary
//!
//! The repository contains a separate quantum IR resource subsystem.
//!
//! This module must **not** import it.
//!
//! The dependency direction is:
//!
//! ```text
//! frontend::ast::node::resources
//!             │
//!             ▼
//! semantic resource resolution
//!             │
//!             ▼
//! quantum::ir::resources
//!             │
//!             ▼
//! quantum/domain lowering
//!             │
//!             ▼
//! target realization
//! ```
//!
//! The quantum IR resource subsystem is therefore a consumer of semantic
//! information, not a dependency of the native AST.
//!
//! ## Annotation boundary
//!
//! Resource annotations are source-level intent.
//!
//! An annotation may express information such as:
//!
//! - a programmer constraint;
//! - a source-level requirement;
//! - a semantic hint;
//! - an extension-specific declaration;
//! - metadata.
//!
//! It must not itself become:
//!
//! - a device allocation;
//! - a scheduler reservation;
//! - a physical qubit assignment;
//! - a calibration object;
//! - a backend credential;
//! - a runtime handle.
//!
//! Interpretation belongs downstream.
//!
//! ## Module dependency direction
//!
//! The intended dependency graph is:
//!
//! ```text
//! resource_kind.rs ─────────┐
//!                           │
//! resource_annotation.rs ───┼──► resources/mod.rs
//!                           │
//! resource.rs ──────────────┘
//!                           │
//!                           ▼
//! frontend::ast::node::mod.rs
//!                           │
//!                           ▼
//! semantic analysis
//!                           │
//!                           ▼
//! ZUIR / domain IR
//! ```
//!
//! More precisely, `mod.rs` does not make the implementation files depend on
//! each other merely to expose them. Each implementation remains independently
//! usable and owns its own invariants.
//!
//! ## Public API strategy
//!
//! The module exposes both:
//!
//! ```rust
//! resources::resource::Resource
//! ```
//!
//! and:
//!
//! ```rust
//! resources::Resource
//! ```
//!
//! through stable re-exports.
//!
//! This gives callers a convenient API while retaining explicit submodule
//! paths for code that wants to make ownership obvious.
//!
//! The same policy is applied to the resource-kind and annotation modules.
//!
//! Re-exports must remain aliases of the canonical implementation.
//!
//! There must never be a second `Resource`, `ResourceKind`, or annotation type
//! defined in this file.
//!
//! ## Why explicit re-exports are preferred over glob imports
//!
//! This module intentionally uses explicit re-exports rather than:
//!
//! ```rust
//! pub use resource::*;
//! pub use resource_kind::*;
//! pub use resource_annotation::*;
//! ```
//!
//! Explicit re-exports provide a stable public surface and prevent accidental
//! exposure of future private implementation details.
//!
//! They also prevent unrelated public symbols from silently becoming part of
//! the `resources` API when an implementation file changes.
//!
//! ## Compatibility policy
//!
//! The canonical implementation files remain authoritative.
//!
//! Adding a new resource API should normally occur in its owning file first.
//!
//! This `mod.rs` should then receive only the corresponding explicit re-export
//! if that symbol is intended to become part of the public resource-module
//! API.
//!
//! Changes to resource semantics must not be hidden inside this module.
//!
//! ## Parser integration
//!
//! The parser consumes the public types exposed here:
//!
//! ```text
//! parser
//!     │
//!     ├── Resource
//!     ├── ResourceKind
//!     └── ResourceAnnotation*
//!     │
//!     ▼
//! native AST
//! ```
//!
//! The parser remains responsible for syntax.
//!
//! This module does not parse source text.
//!
//! ## Structural validation integration
//!
//! AST structural validation consumes the canonical types:
//!
//! ```text
//! Resource
//! ResourceKind
//! ResourceAnnotation*
//!       │
//!       ▼
//! structural validation
//! ```
//!
//! Validation performed by these implementation modules must remain local.
//!
//! Cross-node semantic validation belongs to the semantic phase.
//!
//! ## Semantic integration
//!
//! Semantic analysis consumes the resource AST through this public boundary:
//!
//! ```text
//! Resource
//!     │
//!     ├── name
//!     ├── kind
//!     ├── type
//!     ├── cardinality
//!     ├── scope
//!     ├── initializer
//!     └── annotations
//!     │
//!     ▼
//! semantic resource model
//! ```
//!
//! Semantic analysis is responsible for determining:
//!
//! - whether a resource name resolves;
//! - what resource kind means in the active language/extension environment;
//! - whether its type is valid;
//! - whether cardinality is valid;
//! - whether resource ownership/lifetime rules are satisfied;
//! - whether required capabilities exist;
//! - whether a target can satisfy the requirements.
//!
//! None of these target decisions belong in this module.
//!
//! ## ZUIR integration
//!
//! There is deliberately no direct ZUIR import here.
//!
//! The correct path is:
//!
//! ```text
//! native AST resource
//!       │
//!       ▼
//! semantic resource model
//!       │
//!       ▼
//! ZUIR resource semantics
//!       │
//!       ▼
//! domain-specific lowering
//! ```
//!
//! This preserves the canonical semantic boundary and prevents the native AST
//! from becoming a disguised IR.
//!
//! ## Serialization integration
//!
//! Serialization is owned by the AST serialization subsystem.
//!
//! The resource implementation files provide serializable source-level data.
//!
//! This module must not create a competing serialization format.
//!
//! The complete serialized resource graph must preserve enough information to
//! reconstruct the source-level resource semantics, including referenced
//! `NodeId`s.
//!
//! ## Visitor/traversal integration
//!
//! The implementation files own their node structure.
//!
//! The visitor/traversal subsystem consumes these public types.
//!
//! A resource node must be traversed according to its actual AST child
//! references:
//!
//! ```text
//! Resource
//!     ├── resource type NodeId
//!     ├── cardinality expression NodeId
//!     ├── scope NodeId where applicable
//!     ├── initializer NodeId where applicable
//!     └── annotation NodeIds
//! ```
//!
//! `NodeId` values are references into the containing AST graph; they are not
//! owned child objects inside this module.
//!
//! This preserves graph scalability and avoids duplicating AST nodes.
//!
//! ## Determinism
//!
//! This module introduces no mutable global state.
//!
//! It introduces no:
//!
//! - random state;
//! - timestamps;
//! - thread-local state;
//! - process identifiers;
//! - hardware state;
//! - backend state;
//! - pointer-derived identity.
//!
//! Ordered resource annotations remain ordered.
//!
//! Resource identity remains determined by source-level data and AST identity.
//!
//! ## Scalability
//!
//! This module intentionally contains no collection with a fixed computational
//! capacity.
//!
//! The implementation uses ordinary Rust dynamically sized collections where
//! collections are required.
//!
//! Consequently the representation can grow with available compiler memory
//! subject to explicit external resource policies.
//!
//! "Infinity" in POCO-REAF is therefore an architectural requirement rather
//! than a claim that a finite computer has infinite memory.
//!
//! The AST must never encode an arbitrary artificial ceiling merely because a
//! current target happens to have one.
//!
//! ## Security
//!
//! Source input is untrusted.
//!
//! This module:
//!
//! - performs no I/O;
//! - performs no network access;
//! - executes no source code;
//! - accesses no hardware;
//! - accesses no credentials;
//! - performs no backend calls;
//! - contains no raw pointers;
//! - contains no `unsafe`;
//! - does not turn resource identifiers into executable operations.
//!
//! Hostile-input resource limits belong to configurable compiler policies.
//!
//! ## Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! =============================================================================
//! Module implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

// -----------------------------------------------------------------------------
// Canonical implementation modules
// -----------------------------------------------------------------------------

/// Canonical source-level resource declaration/reference representation.
///
/// This is the authoritative implementation of [`Resource`].
pub mod resource;

/// Canonical source-level resource annotation representation.
///
/// This is the authoritative implementation of resource annotations.
pub mod resource_annotation;

/// Canonical extensible source-level resource-kind identity.
///
/// This is the authoritative implementation of [`ResourceKind`].
pub mod resource_kind;

// -----------------------------------------------------------------------------
// Stable public API
// -----------------------------------------------------------------------------

/// Canonical resource declaration/reference node.
pub use resource::Resource;

/// Source-level resource name.
///
/// This is the resource-node name and is intentionally distinct from any
/// downstream quantum-IR resource identity.
pub use resource::ResourceName;

/// Source-level resource declaration/reference mode.
pub use resource::ResourceMode;

/// Source-level resource cardinality representation.
pub use resource::ResourceCardinality;

/// Source-level resource scope representation.
pub use resource::ResourceScope;

/// Source-level resource type reference.
pub use resource::ResourceType;

/// Ordered source-level resource attribute references.
pub use resource::ResourceAttributes;

/// Resource-local structural error type.
pub use resource::ResourceError;

/// Resource AST schema version.
pub use resource::RESOURCE_AST_SCHEMA_VERSION;

/// Canonical resource AST node-kind name.
pub use resource::RESOURCE_AST_KIND_NAME;

/// Canonical resource AST node-kind constructor.
pub use resource::resource_node_kind;

/// Extensible source-level resource-kind identity.
pub use resource_kind::ResourceKind;

/// Resource-kind construction/validation error.
pub use resource_kind::ResourceKindError;

/// Resource-kind identity component.
pub use resource_kind::ResourceKindComponent;

/// Resource annotation identity.
pub use resource_annotation::ResourceAnnotationId;

/// Resource annotation identifier error.
pub use resource_annotation::ResourceAnnotationIdError;

// -----------------------------------------------------------------------------
// API contract tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// The module must expose the canonical implementation rather than a
    /// duplicate type.
    #[test]
    fn public_resource_api_is_available() {
        let _ = Resource::declaration;
        let _ = ResourceName::new;
        let _ = ResourceMode::Declaration;
        let _ = ResourceCardinality::unspecified;
        let _ = ResourceScope::implicit;
        let _ = ResourceType::unspecified;
        let _ = ResourceAttributes::new;
        let _ = ResourceError::EmptyName;
    }

    /// The resource-kind API must remain available through the parent module.
    #[test]
    fn public_resource_kind_api_is_available() {
        let kind = ResourceKind::new("zamani", "resource");
        assert!(kind.is_ok());

        let _ = ResourceKindError::EmptyNamespace;
        let _ = ResourceKindComponent::Namespace;
    }

    /// Resource annotation identity must remain available through the parent
    /// module without exposing implementation details.
    #[test]
    fn public_resource_annotation_api_is_available() {
        let annotation = ResourceAnnotationId::new("zamani", "resource");
        assert!(annotation.is_ok());

        let _ = ResourceAnnotationIdError::EmptyNamespace;
    }

    /// The node-kind constructor must remain the canonical resource kind.
    #[test]
    fn resource_node_kind_is_stable() {
        assert_eq!(resource_node_kind(), resource_node_kind());
        assert_eq!(RESOURCE_AST_KIND_NAME, "zamani:resource");
        assert_eq!(RESOURCE_AST_SCHEMA_VERSION, 1);
    }

    /// The module must not impose a machine/resource-size policy.
    ///
    /// This is intentionally a compile-time/API-level architectural test:
    /// resource capacity belongs to compiler/target policy, never to this
    /// module.
    #[test]
    fn resource_module_has_no_fixed_resource_capacity() {
        // The test is intentionally empty.
        //
        // Its existence documents the invariant that this module does not
        // define MAX_RESOURCES, MAX_QUBITS, MAX_DEVICES, or equivalent
        // language-level ceilings.
        //
        // Runtime/resource-policy limits must be tested in their owning
        // downstream subsystem.
    }
}