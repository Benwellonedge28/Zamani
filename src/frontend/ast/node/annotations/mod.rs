//! # Zamani Native AST — Annotations
//!
//! `src/frontend/ast/node/annotations/mod.rs`
//!
//! Canonical module boundary for source-level annotations, attributes,
//! pragmas, and directives in the Zamani native AST.
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
//! frontend::ast::node::annotations
//!     │
//!     ├── annotation
//!     ├── attribute
//!     ├── pragma
//!     └── directive
//!     │
//!     ▼
//! structural AST validation
//!     │
//!     ▼
//! semantic analysis
//!     │
//!     ▼
//! semantic model
//!     │
//!     ▼
//! ZUIR
//!     │
//!     ▼
//! domain / target lowering
//! ```
//!
//! This module is an **AST module boundary**.
//!
//! It is not:
//!
//! - a semantic resolver;
//! - a capability registry;
//! - a compiler configuration store;
//! - a hardware configuration store;
//! - a quantum scheduler;
//! - a quantum router;
//! - a calibration database;
//! - a backend API;
//! - a runtime metadata registry;
//! - a ZUIR representation;
//! - a QIR representation;
//! - an OpenQASM AST;
//! - an LLVM/MLIR representation.
//!
//! The native AST records source-level intent and structure. Interpretation
//! belongs to later compilation phases.
//!
//! ## Ownership
//!
//! This `mod.rs` owns only:
//!
//! - annotation submodule organization;
//! - stable public re-exports;
//! - module-level architectural invariants;
//! - dependency-boundary documentation;
//! - compile-time API-surface tests.
//!
//! The implementation modules own their respective data structures:
//!
//! ```text
//! annotation.rs
//!     └── generic source-level annotation
//!
//! attribute.rs
//!     └── source-level attribute
//!
//! pragma.rs
//!     └── source-level pragma/directive-like compiler hint
//!
//! directive.rs
//!     └── source-level directive
//! ```
//!
//! There must be exactly one authoritative implementation of each concept.
//!
//! This file must never duplicate those structures.
//!
//! ## Domain neutrality
//!
//! An annotation is source-level information.
//!
//! It may eventually be consumed by:
//!
//! - classical compilation;
//! - quantum compilation;
//! - hybrid compilation;
//! - HDL compilation;
//! - accelerator compilation;
//! - distributed compilation;
//! - AI compilation;
//! - future computational domains.
//!
//! The annotation layer must therefore not contain a closed domain taxonomy.
//!
//! In particular, this module must not introduce structures such as:
//!
//! ```text
//! enum Annotation {
//!     Quantum(...),
//!     Cpu(...),
//!     Gpu(...),
//!     Ibm(...),
//!     IonQ(...),
//! }
//! ```
//!
//! Such a design would make the AST depend on a finite list of technologies.
//!
//! Instead, annotation identity and interpretation remain extensible and
//! namespaced.
//!
//! ## Annotation versus execution
//!
//! An annotation describes source-level information.
//!
//! It does not grant authority or cause execution by itself.
//!
//! For example, an annotation conceptually expressing:
//!
//! ```text
//! @requires("some.capability")
//! ```
//!
//! must not itself allocate a resource, select hardware, or authorize access.
//!
//! Likewise:
//!
//! ```text
//! @target("some.backend")
//! ```
//!
//! if such syntax exists in the language, is source-level target intent or a
//! constraint. It must not directly instantiate a backend object.
//!
//! Interpretation belongs downstream.
//!
//! ## Quantum boundary
//!
//! Quantum annotations may eventually express source-level intent concerning:
//!
//! - quantum operations;
//! - resource requirements;
//! - execution constraints;
//! - timing intent;
//! - measurement semantics;
//! - error-management intent;
//! - fault-tolerance intent;
//! - domain selection;
//! - compiler hints.
//!
//! However, this module must never contain:
//!
//! - physical qubit assignments;
//! - coupling maps;
//! - routing decisions;
//! - scheduler reservations;
//! - pulse definitions;
//! - calibration records;
//! - decoder state;
//! - QEC lattice layouts;
//! - backend credentials;
//! - QPU handles;
//! - runtime execution jobs.
//!
//! Those belong to downstream quantum compilation/runtime subsystems.
//!
//! ## POCO-REAF
//!
//! The annotation representation must preserve the Zamani invariant:
//!
//! ```text
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
//! ```
//!
//! Source-level annotations must therefore remain meaningful independently of
//! the physical scale of the target.
//!
//! The AST must not contain assumptions such as:
//!
//! ```text
//! MAX_QUBITS
//! MAX_CPUS
//! MAX_GPUS
//! MAX_DEVICES
//! MAX_MACHINE_SIZE
//! ```
//!
//! Nor may annotations implicitly encode a fixed hardware topology.
//!
//! A program may be compiled against a tiny target, a large target, or a
//! heterogeneous/distributed target. The annotation remains source-level
//! information while later phases determine whether and how it can be
//! satisfied.
//!
//! "Infinity" in POCO-REAF means that the representation has no artificial
//! machine-size ceiling. Actual execution remains bounded by the resources
//! available to the compiler and execution environment.
//!
//! ## Extensibility
//!
//! Annotation identity must remain extensible.
//!
//! New annotation namespaces must not require modifying unrelated AST nodes.
//!
//! Conceptually:
//!
//! ```text
//! namespace + name + optional version + source-level arguments
//! ```
//!
//! The exact representation is owned by `annotation.rs` and related
//! implementation files.
//!
//! This module deliberately does not define a closed enum containing every
//! possible annotation known today.
//!
//! This permits future language/domain extensions without redesigning the
//! native AST.
//!
//! ## Namespaces
//!
//! Annotation names should be interpreted as namespaced source-level symbols,
//! rather than globally reserved strings.
//!
//! This permits independently evolving extensions while reducing collisions.
//!
//! For example, conceptually:
//!
//! ```text
//! zamani.compiler.optimize
//! zamani.quantum.intent
//! zamani.resource.require
//! extension.example.annotation
//! ```
//!
//! These examples are illustrative only. The actual accepted namespaces are
//! determined by the language specification and semantic extension registry.
//!
//! The AST preserves the source-level identity; semantic analysis determines
//! whether the namespace is known and whether its use is valid.
//!
//! ## Unknown annotations
//!
//! Unknown annotations must have an explicit policy.
//!
//! At the AST boundary, unknown annotations should remain representable when
//! their syntax is structurally valid.
//!
//! Later compilation modes may:
//!
//! - preserve them;
//! - defer their interpretation;
//! - issue a warning;
//! - reject them during semantic analysis;
//! - reject them for a specific target.
//!
//! The native AST must not silently discard structurally valid source
//! annotations merely because the current compiler does not understand their
//! semantics.
//!
//! Conversely, preserving an annotation does not mean granting it semantic
//! authority.
//!
//! ## Source preservation
//!
//! Annotation structures must preserve sufficient information for:
//!
//! - diagnostics;
//! - IDE tooling;
//! - formatting;
//! - source-to-source transformations;
//! - semantic analysis;
//! - deterministic serialization;
//! - extension processing.
//!
//! Source spelling and source location information remain AST concerns.
//!
//! Resolved semantic identities remain downstream concerns.
//!
//! ## Parser integration
//!
//! The parser is responsible for recognizing annotation syntax.
//!
//! The intended dependency direction is:
//!
//! ```text
//! lexer
//!     │
//!     ▼
//! parser
//!     │
//!     ▼
//! Annotation / Attribute / Pragma / Directive
//!     │
//!     ▼
//! native AST
//! ```
//!
//! This module performs no parsing.
//!
//! It must not:
//!
//! - read source files;
//! - access the network;
//! - invoke external tools;
//! - execute directives;
//! - resolve backend names;
//! - select hardware.
//!
//! Parser-specific recovery policy remains owned by the parser.
//!
//! ## Semantic integration
//!
//! Semantic analysis consumes these source-level structures and determines
//! their meaning in the active language and extension environment.
//!
//! Conceptually:
//!
//! ```text
//! source annotation
//!       │
//!       ▼
//! semantic resolution
//!       │
//!       ├── namespace resolution
//!       ├── name resolution
//!       ├── argument/type checking
//!       ├── contextual validation
//!       ├── capability interpretation
//!       └── domain interpretation
//!       │
//!       ▼
//! semantic annotation model
//! ```
//!
//! This module does not perform those operations.
//!
//! ## ZUIR integration
//!
//! There is intentionally no direct dependency on ZUIR.
//!
//! The correct architectural direction is:
//!
//! ```text
//! Native AST annotation
//!          │
//!          ▼
//! Semantic annotation model
//!          │
//!          ▼
//! ZUIR / domain semantics
//!          │
//!          ▼
//! target-specific realization
//! ```
//!
//! An annotation must not directly construct ZUIR operations or target
//! instructions.
//!
//! This preserves the canonical AST → semantic model → ZUIR boundary.
//!
//! ## Quantum IR integration
//!
//! The native annotation layer must not import:
//!
//! ```text
//! crate::quantum::ir
//! ```
//!
//! Quantum IR is downstream.
//!
//! If a quantum annotation affects quantum compilation, semantic analysis
//! interprets the annotation and produces the appropriate semantic information
//! before quantum IR lowering.
//!
//! Therefore:
//!
//! ```text
//! frontend::ast::node::annotations
//!             │
//!             ▼
//! semantic analysis
//!             │
//!             ▼
//! quantum semantic model
//!             │
//!             ▼
//! quantum::ir
//! ```
//!
//! ## OpenQASM and external formats
//!
//! External quantum languages must not become the native annotation model.
//!
//! An external format such as OpenQASM may have its own syntax tree and
//! annotation/directive representation:
//!
//! ```text
//! external format source
//!       │
//!       ▼
//! external-format AST
//!       │
//!       ▼
//! format validation
//!       │
//!       ▼
//! Zamani semantic representation
//!       │
//!       ▼
//! native AST / semantic pipeline
//! ```
//!
//! The native annotation model must not be shaped around one external format.
//!
//! ## Dependency boundary
//!
//! This module may depend only on its own annotation implementation modules
//! and on foundational AST types required by those modules.
//!
//! It must not depend on:
//!
//! - semantic analysis;
//! - compiler orchestration;
//! - ZUIR;
//! - quantum IR;
//! - hardware;
//! - optimization;
//! - scheduling;
//! - routing;
//! - calibration;
//! - QEC;
//! - ZQN;
//! - runtime;
//! - backend SDKs;
//! - vendor APIs;
//! - filesystem access;
//! - networking.
//!
//! The dependency direction is:
//!
//! ```text
//! foundational AST types
//!          │
//!          ▼
//! annotation implementation modules
//!          │
//!          ▼
//! annotations/mod.rs
//!          │
//!          ▼
//! frontend::ast::node
//!          │
//!          ▼
//! semantic analysis
//!          │
//!          ▼
//! ZUIR / domain IR / targets
//! ```
//!
//! `mod.rs` itself should remain as close to dependency-free as practical.
//!
//! ## Public API policy
//!
//! The canonical implementation modules are public so that the module tree
//! remains explicit:
//!
//! ```rust
//! annotations::annotation
//! annotations::attribute
//! annotations::pragma
//! annotations::directive
//! ```
//!
//! Stable commonly used types are additionally re-exported explicitly:
//!
//! ```rust
//! annotations::Annotation
//! annotations::Attribute
//! annotations::Pragma
//! annotations::Directive
//! ```
//!
//! Explicit re-exports are intentional.
//!
//! Do not use glob imports such as:
//!
//! ```rust
//! pub use annotation::*;
//! ```
//!
//! Glob exports would accidentally make future implementation details part of
//! the public API.
//!
//! ## Compatibility policy
//!
//! A public annotation type should be implemented and stabilized in its owning
//! file before being re-exported here.
//!
//! This means a future change normally follows:
//!
//! ```text
//! implementation file
//!     │
//!     ├── implementation
//!     ├── invariants
//!     ├── tests
//!     └── documentation
//!     │
//!     ▼
//! annotations/mod.rs
//!     │
//!     └── explicit re-export
//! ```
//!
//! This prevents `mod.rs` from becoming a hidden implementation dependency.
//!
//! ## Serialization
//!
//! This module does not define a serialization format.
//!
//! Serialization belongs to the canonical AST serialization subsystem.
//!
//! The annotation implementation types must nevertheless be suitable for
//! deterministic serialization where required by the AST serialization
//! contract.
//!
//! Serialization must preserve, as applicable:
//!
//! - namespace;
//! - name;
//! - version information;
//! - arguments;
//! - attributes;
//! - source identity;
//! - source spans;
//! - extension payloads;
//! - ordering.
//!
//! Serialization must never contain:
//!
//! - memory addresses;
//! - runtime handles;
//! - hardware pointers;
//! - backend credentials;
//! - process IDs;
//! - timestamps introduced solely for serialization;
//! - nondeterministic map iteration as canonical ordering.
//!
//! ## Determinism
//!
//! This module owns no global mutable state.
//!
//! It creates no:
//!
//! - random identifiers;
//! - timestamps;
//! - process-specific identities;
//! - hardware-dependent identities;
//! - pointer-derived identifiers.
//!
//! Annotation ordering must follow the canonical AST/source ordering rules.
//!
//! If an annotation collection has semantic ordering requirements, that order
//! must be represented explicitly rather than reconstructed from a hash map.
//!
//! ## Scalability
//!
//! No fixed annotation count is imposed by this module.
//!
//! Annotation collections must be able to grow according to available
//! compiler resources.
//!
//! The architecture must not introduce artificial limits such as:
//!
//! ```text
//! MAX_ANNOTATIONS
//! MAX_ATTRIBUTES
//! MAX_DIRECTIVES
//! MAX_PRAGMAS
//! ```
//!
//! If the compiler requires resource-exhaustion protection, those limits belong
//! to configurable compiler/resource policies, not the language's AST
//! semantics.
//!
//! ## Memory ownership
//!
//! Annotation nodes must follow the ownership model established by the
//! surrounding native AST.
//!
//! In particular, this module must not introduce a second AST ownership model.
//!
//! Where the surrounding AST uses `NodeId` references and centralized storage,
//! annotation child references must use that canonical mechanism rather than
//! embedding duplicated AST graphs.
//!
//! ## Traversal
//!
//! The annotation implementation modules participate in the canonical AST
//! visitor/traversal system.
//!
//! Conceptually:
//!
//! ```text
//! parent AST node
//!      │
//!      ▼
//! annotation collection
//!      │
//!      ├── annotation
//!      ├── attribute
//!      ├── pragma
//!      └── directive
//!      │
//!      ▼
//! visitor / traversal
//! ```
//!
//! `mod.rs` does not implement traversal.
//!
//! The visitor subsystem must consume the canonical public types exposed here.
//!
//! ## Validation boundary
//!
//! Local structural validation belongs in the annotation implementation files
//! and the canonical AST validation subsystem.
//!
//! Validation must distinguish:
//!
//! ```text
//! structural validity
//!       ≠
//! semantic validity
//!       ≠
//! target capability
//!       ≠
//! runtime authorization
//! ```
//!
//! For example, an annotation can be structurally valid while semantically
//! unknown. It can also be semantically valid while unsupported by a selected
//! target.
//!
//! Those are different compilation states and must not be conflated.
//!
//! ## Security
//!
//! Annotations originate from potentially untrusted source input.
//!
//! This module must:
//!
//! - perform no I/O;
//! - perform no network access;
//! - execute no annotation payload;
//! - invoke no commands;
//! - access no credentials;
//! - access no hardware;
//! - contain no raw pointers;
//! - contain no `unsafe`;
//! - treat annotation values as data.
//!
//! An annotation such as a compiler directive must never execute merely because
//! it exists in the AST.
//!
//! Semantic interpretation and execution authority remain outside the AST.
//!
//! ## No hidden execution channel
//!
//! This is particularly important for annotations that may eventually describe
//! compiler actions.
//!
//! The AST must represent:
//!
//! ```text
//! "the programmer wrote this directive"
//! ```
//!
//! rather than:
//!
//! ```text
//! "the compiler has already performed the directive"
//! ```
//!
//! This guarantees that parsing remains deterministic and side-effect free.
//!
//! ## Versioning
//!
//! Annotation schema versioning must remain separate from:
//!
//! - Zamani language version;
//! - compiler version;
//! - serialized AST version;
//! - extension version;
//! - target/backend version.
//!
//! This module does not invent a second versioning system.
//!
//! Version fields required by an annotation's source syntax belong to the
//! annotation implementation.
//!
//! Serialization schema versioning belongs to the AST serialization layer.
//!
//! ## Migration contract
//!
//! During migration from legacy AST structures, every legacy annotation,
//! attribute, pragma, and directive representation must map to exactly one
//! canonical implementation.
//!
//! The migration process should maintain a mapping of the form:
//!
//! ```text
//! legacy construct
//!     │
//!     ▼
//! canonical annotation type
//!     │
//!     ▼
//! structural validation
//!     │
//!     ▼
//! semantic interpretation
//!     │
//!     ▼
//! ZUIR lowering where applicable
//! ```
//!
//! No duplicate legacy and native representations should remain authoritative.
//!
//! ## Testing contract
//!
//! The implementation files own their behavioral tests.
//!
//! The module boundary additionally verifies that the intended public API is
//! available without exposing accidental implementation details.
//!
//! Required integration coverage elsewhere includes:
//!
//! - construction;
//! - malformed annotation handling;
//! - namespace/name validation;
//! - source-span preservation;
//! - traversal;
//! - serialization round-trips;
//! - deterministic serialization;
//! - unknown-extension preservation;
//! - semantic lowering;
//! - large annotation collections;
//! - deeply nested argument structures where supported;
//! - fuzzing of annotation syntax and values.
//!
//! ## Rust compatibility
//!
//! This module targets:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! The implementation intentionally uses no external dependency directly.
//!
//! ## Module implementation
//!
//! The declarations below are the only implementation ownership introduced by
//! this module boundary.
//!
//! Keep this list synchronized with the actual files in this directory.
//! A missing implementation file is a build error rather than a silently
//! ignored feature.
//!
//! =============================================================================
//! Canonical implementation modules
//! =============================================================================

// -----------------------------------------------------------------------------
// Generic annotation
// -----------------------------------------------------------------------------

/// Canonical source-level annotation representation.
///
/// This module owns the generic annotation data model and its local
/// invariants.
pub mod annotation;

// -----------------------------------------------------------------------------
// Attribute
// -----------------------------------------------------------------------------

/// Canonical source-level attribute representation.
///
/// Attributes are source metadata/intent and are not execution permissions.
pub mod attribute;

// -----------------------------------------------------------------------------
// Pragma
// -----------------------------------------------------------------------------

/// Canonical source-level pragma representation.
///
/// Pragmas remain source data until interpreted by an appropriate downstream
/// compiler phase.
pub mod pragma;

// -----------------------------------------------------------------------------
// Directive
// -----------------------------------------------------------------------------

/// Canonical source-level directive representation.
///
/// Directives are not executed by the parser or AST.
pub mod directive;

// -----------------------------------------------------------------------------
// Stable explicit public re-exports
// -----------------------------------------------------------------------------
//
// These re-exports intentionally name only canonical public types.
//
// If an implementation file uses a different authoritative public type name,
// that type must be changed in its owning file before this module is changed.
// Do not introduce aliases here merely to conceal incompatible designs.

pub use annotation::Annotation;
pub use attribute::Attribute;
pub use directive::Directive;
pub use pragma::Pragma;

// -----------------------------------------------------------------------------
// Compile-time API-surface tests
// -----------------------------------------------------------------------------
//
// These tests intentionally remain small. Behavioral tests belong beside the
// implementation types and in the AST integration test suite.
//
// Their purpose here is to ensure that the module boundary remains coherent.

#[cfg(test)]
mod tests {
    use super::{
        Annotation,
        Attribute,
        Directive,
        Pragma,
    };

    /// Ensures the four canonical annotation-family types remain publicly
    /// reachable through this module.
    ///
    /// This test intentionally performs no construction because constructors
    /// and invariants belong to the owning implementation files.
    #[test]
    fn public_annotation_api_is_exposed() {
        fn assert_type<T>() {}

        assert_type::<Annotation>();
        assert_type::<Attribute>();
        assert_type::<Directive>();
        assert_type::<Pragma>();
    }
}