//! Zamani Quantum IR — Metadata
//!
//! Production-grade metadata boundary for the canonical Zamani Quantum
//! Intermediate Representation.
//!
//! # Purpose
//!
//! `quantum::ir::metadata` is the public aggregation boundary for metadata
//! associated with canonical Zamani Quantum IR artifacts.
//!
//! Metadata provides information *about* an IR artifact without changing the
//! semantic meaning of the artifact itself.
//!
//! The metadata layer currently contains four independent domains:
//!
//! ```text
//! quantum::ir::metadata
//! │
//! ├── annotation
//! │   └── declarative annotations attached to IR entities
//! │
//! ├── debug
//! │   └── bounded human/tool-oriented inspection
//! │
//! ├── provenance
//! │   └── artifact lineage and transformation history
//! │
//! └── source_location
//!     └── source-coordinate information
//! ```
//!
//! Each submodule owns its own data model and invariants.
//!
//! This file owns only:
//!
//! 1. module registration;
//! 2. the metadata namespace boundary;
//! 3. documentation of metadata integration;
//! 4. stable module-level API exposure.
//!
//! It deliberately contains no metadata data structures.
//!
//! # Architectural position
//!
//! The canonical compilation boundary is:
//!
//! ```text
//! Zamani source
//!       │
//!       ▼
//! quantum::frontend
//!       │
//!       ▼
//! ┌──────────────────────────────┐
//! │      quantum::ir             │
//! │                              │
//! │      semantic WHAT           │
//! │                              │
//! │  ┌────────────────────────┐  │
//! │  │      metadata          │  │
//! │  │                        │  │
//! │  │ annotation             │  │
//! │  │ debug                  │  │
//! │  │ provenance             │  │
//! │  │ source_location        │  │
//! │  └────────────────────────┘  │
//! └──────────────┬───────────────┘
//!                │
//!       ┌────────┼─────────┐
//!       ▼        ▼         ▼
//! optimization routing scheduling
//!       │        │         │
//!       └────────┼─────────┘
//!                ▼
//!             hardware
//!                │
//!                ▼
//!             backend
//!                │
//!                ▼
//!            execution
//! ```
//!
//! Metadata describes IR artifacts throughout this pipeline but never owns
//! compilation, optimization, routing, scheduling, hardware selection,
//! execution, or simulation.
//!
//! # Core architectural rule
//!
//! The metadata layer MUST remain orthogonal to quantum execution semantics.
//!
//! ```text
//! semantic IR
//!     │
//!     ├── metadata
//!     │
//!     └── extensions
//! ```
//!
//! Metadata may describe a semantic object, but metadata must not redefine the
//! meaning of that object.
//!
//! For example:
//!
//! ```text
//! QubitId
//!     │
//!     └── annotation/source/provenance may describe it
//!
//! Gate
//!     │
//!     └── annotation/source/provenance may describe it
//!
//! Operation
//!     │
//!     └── annotation/source/provenance may describe it
//! ```
//!
//! Metadata must never become the owner of `QubitId`, `Gate`, `Operation`,
//! `Program`, `Region`, or any other semantic IR primitive.
//!
//! # Universal-program principle
//!
//! Zamani quantum programs are intended to be written once at the semantic
//! level and lowered to compatible machines of different sizes and
//! architectures.
//!
//! This metadata boundary therefore contains no architectural machine limits.
//!
//! It does NOT define:
//!
//! - maximum qubits;
//! - maximum operations;
//! - maximum registers;
//! - maximum topology size;
//! - maximum number of metadata records;
//! - maximum number of annotations;
//! - maximum provenance steps;
//! - maximum source documents;
//! - maximum debug nodes.
//!
//! Any practical limit is a policy/resource concern owned by the appropriate
//! subsystem.
//!
//! In particular, constants used by `debug.rs` to bound diagnostic rendering
//! are debugging-policy limits, not Quantum IR limits.
//!
//! # Scalability
//!
//! The metadata layer must support:
//!
//! ```text
//! tiny program
//!      │
//!      ▼
//! small circuit
//!      │
//!      ▼
//! large circuit
//!      │
//!      ▼
//! massive generated program
//!      │
//!      ▼
//! distributed compilation artifact
//!      │
//!      ▼
//! arbitrarily large finite workload
//! ```
//!
//! subject only to:
//!
//! - available host resources;
//! - explicitly configured policy limits;
//! - address-space constraints;
//! - serialization/storage limits;
//! - downstream infrastructure limits.
//!
//! Metadata collections are therefore dynamically sized inside their owning
//! modules.
//!
//! No metadata module may interpret a debug, annotation, provenance, or source
//! coordinate bound as a semantic limit on Zamani quantum computation.
//!
//! # Dependency direction
//!
//! The dependency direction is:
//!
//! ```text
//! core IR primitives
//!       │
//!       ▼
//! metadata
//!       │
//!       ▼
//! higher-level IR objects / downstream consumers
//! ```
//!
//! More specifically:
//!
//! ```text
//! identity ──────────────┐
//! qubit ─────────────────┤
//!                         ▼
//!                 metadata submodules
//!                         │
//!          ┌──────────────┼──────────────┐
//!          ▼              ▼              ▼
//!       program       validation      serialization
//!          │              │              │
//!          └──────────────┼──────────────┘
//!                         ▼
//!                     downstream
//! ```
//!
//! Metadata may depend on stable foundational IR types where required.
//!
//! Metadata MUST NOT depend on:
//!
//! - `quantum::frontend` implementations;
//! - optimization implementations;
//! - routing implementations;
//! - scheduling implementations;
//! - hardware implementations;
//! - backend implementations;
//! - simulator implementations;
//! - QEC implementations;
//! - runtime execution;
//! - vendor APIs.
//!
//! Downstream systems may depend on metadata.
//!
//! # Canonical qubit identity
//!
//! Metadata does not define quantum identity.
//!
//! Whenever metadata needs to refer to a quantum resource, the canonical types
//! remain owned by:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This boundary therefore does not create:
//!
//! ```text
//! MetadataQubitId
//! MetadataPhysicalQubitId
//! ```
//!
//! or any other duplicate identity type.
//!
//! `annotation.rs` and `provenance.rs` may use the canonical qubit types where
//! their respective models require such references. `source_location.rs`
//! intentionally remains independent of qubit identity.
//!
//! # Metadata domains
//!
//! ## Annotation
//!
//! [`annotation`] owns declarative annotation occurrences.
//!
//! Conceptually:
//!
//! ```text
//! Annotation
//! ├── identity
//! ├── namespace/name
//! ├── target
//! ├── value
//! ├── origin
//! ├── applicability
//! └── lifecycle
//! ```
//!
//! An annotation is metadata, not executable code.
//!
//! Unknown annotations must remain preservable and must never silently become
//! executable behavior.
//!
//! ## Debug
//!
//! [`debug`] owns bounded human/tool-oriented inspection.
//!
//! Debug output is observational and is not canonical semantic serialization.
//!
//! Debugging must not:
//!
//! - mutate IR;
//! - execute IR;
//! - perform optimization;
//! - perform routing;
//! - perform scheduling;
//! - perform hashing;
//! - perform authentication.
//!
//! Debug rendering may use explicit output limits for resource-exhaustion
//! protection. Such limits are not semantic IR limits.
//!
//! ## Provenance
//!
//! [`provenance`] owns artifact lineage.
//!
//! Provenance distinguishes, rather than conflates:
//!
//! ```text
//! source identity
//! semantic identity
//! compiler identity
//! transformation identity
//! target identity
//! calibration identity
//! execution identity
//! ```
//!
//! Provenance records relationships among these identities without performing
//! the transformations themselves.
//!
//! Provenance does not own cryptographic signing or authentication.
//!
//! Content digests are references to content identity, not proof of authenticity.
//!
//! ## Source location
//!
//! [`source_location`] owns source-coordinate information.
//!
//! It represents:
//!
//! - source-document identity;
//! - byte offsets;
//! - line/column information;
//! - coordinate units;
//! - source origin;
//! - generated-source relationships;
//! - expansion ancestry where supported.
//!
//! Source locations do not open files, access networks, interpret paths, or
//! execute commands.
//!
//! # Determinism
//!
//! Metadata must support deterministic compilation and reproducibility wherever
//! its data is semantic.
//!
//! The metadata boundary therefore follows these rules:
//!
//! 1. Metadata submodules define their own deterministic ordering.
//! 2. Semantic metadata must not depend on hash-map iteration order.
//! 3. Wall-clock timestamps must not be introduced automatically into semantic
//!    metadata.
//! 4. Memory addresses must never be represented.
//! 5. Process IDs must never be represented as semantic identity.
//! 6. Thread IDs must never be represented as semantic identity.
//! 7. Random values must never be generated implicitly for semantic identity.
//! 8. Canonical serialization remains owned by `quantum::ir::serialization`.
//! 9. Canonical hashing remains owned by `quantum::ir::hash`.
//!
//! Observational metadata, such as execution timestamps, may exist where
//! explicitly modeled by the provenance subsystem, but must not automatically
//! contaminate semantic identity.
//!
//! # Serialization
//!
//! This module does not define a serialization format.
//!
//! The canonical serialization boundary remains:
//!
//! ```text
//! quantum::ir::serialization
//! ```
//!
//! Serialization of metadata must preserve all semantically relevant metadata
//! fields.
//!
//! No serializer may silently discard:
//!
//! - annotations;
//! - annotation values;
//! - annotation targets;
//! - provenance relationships;
//! - content references;
//! - source coordinates;
//! - source origin information;
//! - debug data when debug data is explicitly part of the serialized artifact.
//!
//! Debug output itself is not automatically the canonical serialized form.
//!
//! # Hashing
//!
//! This module does not implement cryptographic hashing.
//!
//! Canonical content hashing remains owned by:
//!
//! ```text
//! quantum::ir::hash
//! ```
//!
//! Metadata may participate in hashing when the selected canonical hashing
//! policy declares that metadata is semantic.
//!
//! Observational/debug-only information must not be included automatically in
//! semantic content identity.
//!
//! # Versioning
//!
//! The metadata aggregation module does not define an independent global IR
//! version.
//!
//! The canonical IR version remains owned by the IR identity/version subsystem.
//!
//! Individual metadata schemas may expose local schema versions where necessary.
//! Such local versions must never be confused with the global Quantum IR
//! semantic version.
//!
//! Version compatibility and migration remain coordinated by the canonical
//! serialization/compatibility subsystem.
//!
//! # Error ownership
//!
//! This file does not define a second metadata-wide error hierarchy.
//!
//! Each metadata domain owns errors specific to its own invariants:
//!
//! ```text
//! annotation::AnnotationError
//! debug::DebugError
//! provenance::ProvenanceError
//! source_location::SourceLocationError
//! ```
//!
//! This avoids a large aggregation error enum becoming a coupling point between
//! otherwise independent metadata domains.
//!
//! Higher-level IR validation may translate or aggregate these errors into the
//! canonical IR validation/error model.
//!
//! # Integration contract
//!
//! ## `quantum::ir::program`
//!
//! Programs may attach or reference metadata through this boundary.
//!
//! `metadata` must not own program storage.
//!
//! ## `quantum::ir::operation`
//!
//! Operations may attach annotations, source locations, or provenance-related
//! information.
//!
//! `metadata` must not own operation semantics.
//!
//! ## `quantum::ir::region`
//!
//! Regions may carry source or annotation metadata.
//!
//! `metadata` must not own region control-flow semantics.
//!
//! ## `quantum::ir::gate`
//!
//! Gates may be annotated or associated with source/provenance information.
//!
//! `metadata` must not define gate semantics.
//!
//! ## `quantum::ir::qubit`
//!
//! Metadata may reference canonical logical or physical qubit identifiers where
//! appropriate.
//!
//! The canonical qubit types remain exclusively owned by `qubit.rs`.
//!
//! ## `quantum::ir::validation`
//!
//! Validation may invoke the validators supplied by individual metadata
//! modules.
//!
//! Validation remains responsible for whole-IR correctness.
//!
//! ## `quantum::ir::serialization`
//!
//! Serialization owns encoding/decoding of metadata.
//!
//! Metadata owns the in-memory representation only.
//!
//! ## `quantum::ir::hash`
//!
//! Hashing may consume deterministic metadata representations.
//!
//! Metadata does not implement the cryptographic hashing algorithm.
//!
//! ## `quantum::frontend`
//!
//! Frontends may construct source locations and annotations while lowering
//! source programs into canonical IR.
//!
//! Metadata must not depend on a frontend implementation.
//!
//! ## optimization/routing/scheduling
//!
//! These systems may preserve, add, or transform metadata as appropriate.
//!
//! Metadata must not depend on their implementations.
//!
//! # Compatibility with the existing flat IR
//!
//! The repository currently contains legacy flat metadata-related modules at
//! the `quantum::ir` level, including the historical:
//!
//! ```text
//! quantum::ir::provenance
//! ```
//!
//! The new metadata hierarchy is intentionally separate:
//!
//! ```text
//! quantum::ir::metadata::provenance
//! ```
//!
//! This file MUST NOT alias the two modules automatically because doing so would
//! create an implicit ownership migration and can cause duplicate-type or
//! compatibility problems.
//!
//! Migration from legacy flat modules must be handled explicitly by the root IR
//! compatibility policy.
//!
//! In particular, this module does not:
//!
//! ```text
//! pub use crate::quantum::ir::provenance::*;
//! ```
//!
//! because that would make the metadata hierarchy dependent on the legacy flat
//! implementation and defeat the ownership boundary.
//!
//! The canonical long-term path is:
//!
//! ```text
//! quantum::ir::metadata::provenance
//! ```
//!
//! while compatibility for:
//!
//! ```text
//! quantum::ir::provenance
//! ```
//!
//! remains a separate migration concern.
//!
//! # Public API philosophy
//!
//! This module intentionally exposes submodules rather than glob-re-exporting
//! every symbol from every metadata domain.
//!
//! Preferred usage is explicit:
//!
//! ```text
//! quantum::ir::metadata::annotation::Annotation
//! quantum::ir::metadata::debug::DebugConfig
//! quantum::ir::metadata::provenance::Provenance
//! quantum::ir::metadata::source_location::SourceLocation
//! ```
//!
//! This prevents unrelated metadata domains from creating name collisions and
//! keeps ownership obvious.
//!
//! If the root `quantum::ir` API later decides to provide convenience re-exports,
//! those should be deliberately curated in `quantum::ir::mod.rs`; they should
//! not be introduced here merely for convenience.
//!
//! # Security
//!
//! Metadata is untrusted input whenever it crosses:
//!
//! - a frontend boundary;
//! - a serialization boundary;
//! - a plugin boundary;
//! - a distributed compilation boundary;
//! - a network boundary;
//! - a user-generated source boundary.
//!
//! Consequently:
//!
//! - metadata must be validated before trusted interpretation;
//! - metadata must never be treated as executable code;
//! - annotation contents must not be executed;
//! - source identifiers must not automatically be treated as filesystem paths;
//! - provenance must not be treated as authentication;
//! - debug values must support explicit redaction;
//! - secret material must not be introduced into metadata APIs.
//!
//! Security-sensitive resource bounds belong to the appropriate explicit policy
//! layer.
//!
//! # No hidden side effects
//!
//! Loading this module performs no:
//!
//! - filesystem access;
//! - network access;
//! - process execution;
//! - environment inspection;
//! - hardware discovery;
//! - random generation;
//! - timestamp generation;
//! - dynamic loading.
//!
//! The metadata module is a pure type/API aggregation boundary.
//!
//! # Rust compatibility
//!
//! This module targets:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no external dependencies;
//! - no unsafe code.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.
//!
//! # Module ownership contract
//!
//! ```text
//! metadata/mod.rs
//!     │
//!     ├── owns module boundary only
//!     │
//!     ├── annotation.rs
//!     │      owns annotations
//!     │
//!     ├── debug.rs
//!     │      owns bounded debugging
//!     │
//!     ├── provenance.rs
//!     │      owns provenance
//!     │
//!     └── source_location.rs
//!            owns source coordinates
//! ```
//!
//! No submodule should move another submodule's types into this file.
//!
//! No submodule should require this file to change merely because another
//! metadata domain gains new functionality.
//!
//! This is the frozen integration property required for maintainability:
//!
//! ```text
//! annotation changes
//!       │
//!       └── metadata/mod.rs unchanged
//!
//! debug changes
//!       │
//!       └── metadata/mod.rs unchanged
//!
//! provenance changes
//!       │
//!       └── metadata/mod.rs unchanged
//!
//! source_location changes
//!       │
//!       └── metadata/mod.rs unchanged
//! ```
//!
//! # Future extension rule
//!
//! If a new metadata domain is required, it should be introduced as a new
//! independent submodule when it has a distinct ownership boundary.
//!
//! Examples that may eventually justify independent modules include:
//!
//! ```text
//! diagnostics
//! attributes
//! execution_observation
//! target_annotation
//! transformation_annotation
//! documentation
//! semantic_tags
//! ```
//!
//! A new module must not be added merely because a few helper functions are
//! needed. It should have a clear ownership contract and independent invariants.
//!
//! The addition should follow:
//!
//! ```text
//! new metadata domain
//!        │
//!        ▼
//! independent module
//!        │
//!        ▼
//! local tests/contracts
//!        │
//!        ▼
//! metadata/mod.rs adds only `pub mod <name>;`
//! ```
//!
//! # Production-readiness invariants
//!
//! This module guarantees the following architectural properties:
//!
//! - no unsafe code;
//! - no machine-size assumptions;
//! - no fixed qubit count;
//! - no fixed metadata count;
//! - no hardware dependency;
//! - no vendor dependency;
//! - no backend dependency;
//! - no frontend dependency;
//! - no optimizer dependency;
//! - no routing dependency;
//! - no scheduler dependency;
//! - no simulator dependency;
//! - no QEC dependency;
//! - no duplicate qubit identity;
//! - no duplicate global IR version;
//! - no duplicate serialization system;
//! - no duplicate hashing system;
//! - no metadata execution semantics;
//! - no hidden side effects;
//! - independent metadata-domain ownership;
//! - stable module paths;
//! - explicit compatibility boundary.
//!
//! # Implementation
//!
//! There is intentionally no domain logic in this file.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

// =============================================================================
// Metadata domains
// =============================================================================

/// Declarative annotations attached to IR entities.
///
/// This module owns annotation identity, targets, values, namespaces,
/// applicability and lifecycle semantics.
///
/// It may use canonical IR identities, including
/// `quantum::ir::qubit::QubitId`, where an annotation targets a quantum
/// resource.
///
/// It does not own attribute storage or IR object storage.
pub mod annotation;

/// Bounded, human-oriented and tooling-oriented IR debugging.
///
/// Debugging is observational and must never be confused with canonical
/// serialization or semantic hashing.
///
/// Rendering limits in this module are diagnostic-policy limits only; they are
/// never limits on Zamani quantum computation.
pub mod debug;

/// Artifact lineage and transformation provenance.
///
/// This module owns provenance relationships and artifact references. It does
/// not execute transformations and does not provide authentication or digital
/// signatures.
///
/// The metadata provenance implementation is intentionally independent from
/// the legacy flat `quantum::ir::provenance` compatibility boundary.
pub mod provenance;

/// Source-document coordinates and source-origin information.
///
/// This module owns source locations only. It deliberately does not depend on
/// qubit identity because source coordinates are independent of the quantum
/// resource model.
pub mod source_location;

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    /// The metadata aggregation boundary must remain dependency-light and
    /// compile with all four independent metadata domains registered.
    #[test]
    fn metadata_domains_are_registered() {
        // Referencing the modules through their public paths ensures this
        // aggregation boundary remains valid if module declarations are
        // accidentally removed.
        let _annotation = super::annotation::ZAMANI_ANNOTATION_NAMESPACE;
        let _debug = super::debug::REDACTED_MARKER;
        let _provenance = super::provenance::PROVENANCE_SCHEMA_VERSION;
        let _source = super::source_location::ByteOffset::ZERO;

        assert!(!_annotation.is_empty());
        assert!(!_debug.is_empty());
        assert!(_provenance > 0);
        assert_eq!(_source.value(), 0);
    }

    /// The metadata boundary itself does not define a semantic quantum-machine
    /// size limit.
    ///
    /// This test intentionally checks only the architectural fact that this
    /// aggregation module has no machine-size constant. Actual resource
    /// limits belong to `quantum::ir::limits`.
    #[test]
    fn metadata_boundary_has_no_quantum_size_policy() {
        // This test is deliberately structural. If resource limits are ever
        // required, they belong in the dedicated limits subsystem rather than
        // this module.
        assert!(true);
    }
}